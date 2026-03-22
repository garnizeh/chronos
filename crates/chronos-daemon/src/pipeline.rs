use crate::database::Database;
use chronos_core::error::{ChronosError, Result};
use chronos_core::models::{Frame, SemanticLog};
use chronos_core::traits::VisionInference;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

/// The core orchestrator. Receives frames, analyzes them, stores results.
/// Generic over its dependencies — accepts any VisionInference implementation.
pub struct CaptureEngine<V: VisionInference> {
    /// The VLM client used for frame analysis.
    vision: V,
    /// The database handle for storing results.
    database: Database,
}

impl<V: VisionInference> CaptureEngine<V> {
    /// Creates a new `CaptureEngine` with the given vision inference engine and database.
    pub fn new(vision: V, database: Database) -> Self {
        Self { vision, database }
    }

    /// Processes a single frame by analyzing it with the VLM and storing the result in the database.
    ///
    /// Implements bounded retry with exponential backoff for transient errors
    /// (timeouts, database locks, or VLM inference hiccups).
    ///
    /// **Go Parallel (Didactic):** This is like a robust worker function that
    /// manages its own retry lifecycle before giving up. In Go, you'd use a
    /// simple for-loop with a `time.Sleep` and a switch on error types.
    pub async fn process_frame(&self, frame: &Frame) -> Result<SemanticLog> {
        let mut attempts = 0;
        let max_attempts = 3;
        let mut backoff = Duration::from_millis(500);

        loop {
            attempts += 1;

            // Attempt the unit of work
            let result = async {
                let log = self.vision.analyze_frame(frame).await?;
                self.database.insert_semantic_log(&log).await?;
                Ok(log)
            }
            .await;

            match result {
                Ok(log) => return Ok(log),
                Err(e) if attempts < max_attempts && is_transient(&e) => {
                    warn!(
                        "Transient error processing frame {} (attempt {}/{}): {}. Retrying in {:?}...",
                        frame.id, attempts, max_attempts, e, backoff
                    );
                    tokio::time::sleep(backoff).await;
                    backoff *= 2; // Exponential backoff
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Runs the main pipeline loop, receiving frames from the channel and processing them.
    /// This method blocks until the channel is closed.
    ///
    /// **Go Parallel:** This is your `for frame := range ch { ... }` loop.
    pub async fn run_pipeline(&self, mut rx: mpsc::Receiver<Frame>) -> Result<()> {
        info!("Starting pipeline loop");
        while let Some(frame) = rx.recv().await {
            // We pass a reference to avoid cloning the large image data buffer inside Frame
            if let Err(e) = self.process_frame(&frame).await {
                // After retries are exhausted, we log the final failure and move to the next frame.
                error!(
                    "Pipeline failed to process frame {} after {} attempts: {:?}",
                    frame.id, 3, e
                );
            }
        }
        info!("Pipeline loop ended (channel closed)");
        Ok(())
    }
}

/// Helper to identify transient errors that are worth retrying.
fn is_transient(err: &ChronosError) -> bool {
    matches!(
        err,
        ChronosError::Timeout(_) | ChronosError::Inference(_) | ChronosError::Database(_)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;
    use chronos_core::error::ChronosError;
    use chronos_core::traits::mocks::MockVision;
    use ulid::Ulid;

    #[tokio::test]
    async fn test_capture_engine_creation() {
        let db = Database::new_in_memory()
            .await
            .expect("Failed to create in-memory DB");
        let vision = MockVision;

        let _engine = CaptureEngine::new(vision, db);
    }

    #[tokio::test]
    async fn test_process_frame_with_mocks() {
        let db = Database::new_in_memory()
            .await
            .expect("Failed to create in-memory DB");
        let vision = MockVision;
        let engine = CaptureEngine::new(vision, db);

        let frame = Frame {
            id: Ulid::new(),
            timestamp: Utc::now(),
            image_data: vec![0, 1, 2, 3],
            width: 100,
            height: 100,
        };

        // Now accepts reference
        let result: Result<SemanticLog> = engine.process_frame(&frame).await;
        assert!(result.is_ok());

        let log = result.unwrap();
        assert_eq!(log.source_frame_id, frame.id);

        let count = engine.database.get_log_count().await.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_process_frame_stores_correct_source_frame_id() {
        let db = Database::new_in_memory().await.unwrap();
        let engine = CaptureEngine::new(MockVision, db);

        let frame_id = Ulid::new();
        let frame = Frame {
            id: frame_id,
            timestamp: Utc::now(),
            image_data: vec![],
            width: 0,
            height: 0,
        };

        let log = engine.process_frame(&frame).await.unwrap();
        assert_eq!(log.source_frame_id, frame_id);
    }

    struct FailingVision;
    #[async_trait]
    impl VisionInference for FailingVision {
        async fn analyze_frame(&self, _frame: &Frame) -> Result<SemanticLog> {
            Err(ChronosError::Inference("VLM error".to_string()))
        }
    }

    #[tokio::test]
    async fn test_process_frame_handles_vision_error() {
        let db = Database::new_in_memory().await.unwrap();
        let engine = CaptureEngine::new(FailingVision, db);

        let frame = Frame {
            id: Ulid::new(),
            timestamp: Utc::now(),
            image_data: vec![],
            width: 0,
            height: 0,
        };

        // Tests that process_frame eventually returns the error after retries
        let result: Result<SemanticLog> = engine.process_frame(&frame).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_pipeline_processes_multiple_frames() {
        let db = Database::new_in_memory().await.unwrap();
        let vision = MockVision;
        let engine = CaptureEngine::new(vision, db);

        let (tx, rx) = mpsc::channel(10);

        let engine_handle = tokio::spawn(async move {
            engine.run_pipeline(rx).await.unwrap();
            engine
        });

        for _ in 0..5 {
            let frame = Frame {
                id: Ulid::new(),
                timestamp: Utc::now(),
                image_data: vec![],
                width: 0,
                height: 0,
            };
            tx.send(frame).await.unwrap();
        }

        drop(tx);
        let engine = engine_handle.await.unwrap();

        let count = engine.database.get_log_count().await.unwrap();
        assert_eq!(count, 5);
    }

    /// A mock vision that fails a specific number of times before succeeding.
    struct FlakyVision {
        fail_count: std::sync::Arc<std::sync::Mutex<usize>>,
        max_fails: usize,
    }

    #[async_trait]
    impl VisionInference for FlakyVision {
        async fn analyze_frame(&self, frame: &Frame) -> Result<SemanticLog> {
            let mut count = self.fail_count.lock().unwrap();
            if *count < self.max_fails {
                *count += 1;
                Err(ChronosError::Inference("Transient VLM error".to_string()))
            } else {
                Ok(SemanticLog {
                    id: Ulid::new(),
                    source_frame_id: frame.id,
                    timestamp: Utc::now(),
                    description: "Success after failure".to_string(),
                    active_application: None,
                    activity_category: None,
                    key_entities: vec![],
                    confidence_score: 1.0,
                    raw_vlm_response: "{}".to_string(),
                })
            }
        }
    }

    #[tokio::test]
    async fn test_pipeline_handles_vision_error_gracefully_and_continues() {
        let db = Database::new_in_memory().await.unwrap();

        // This vision will fail the first frame (after 3 retries) and succeed on the second.
        let fail_count = std::sync::Arc::new(std::sync::Mutex::new(0));
        let vision = FlakyVision {
            fail_count: fail_count.clone(),
            max_fails: 3, // Frame 1 will use attempts 1, 2, 3 and fail.
        };

        let engine = CaptureEngine::new(vision, db);
        let (tx, rx) = mpsc::channel(10);

        let engine_handle = tokio::spawn(async move {
            engine.run_pipeline(rx).await.unwrap();
            engine
        });

        // Send Frame 1 (should fail after retries)
        tx.send(Frame {
            id: Ulid::new(),
            timestamp: Utc::now(),
            image_data: vec![],
            width: 0,
            height: 0,
        })
        .await
        .unwrap();

        // Send Frame 2 (should succeed)
        tx.send(Frame {
            id: Ulid::new(),
            timestamp: Utc::now(),
            image_data: vec![],
            width: 0,
            height: 0,
        })
        .await
        .unwrap();

        drop(tx);
        let engine = engine_handle.await.unwrap();

        // Verify only 1 log entry exists (from Frame 2)
        let count = engine.database.get_log_count().await.unwrap();
        assert_eq!(
            count, 1,
            "Pipeline should have continued and processed the second frame"
        );
    }
}
