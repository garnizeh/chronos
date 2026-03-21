use crate::database::Database;
use chronos_core::error::Result;
use chronos_core::models::{Frame, SemanticLog};
use chronos_core::traits::VisionInference;
use tokio::sync::mpsc;
use tracing::{error, info};

/// The core orchestrator. Receives frames, analyzes them, stores results.
/// Generic over its dependencies — accepts any VisionInference implementation.
///
/// **Go Parallel:** This is like a struct that takes interfaces in Go:
/// ```go
/// type CaptureEngine struct {
///     vision VisionInference
///     db     *Database
/// }
/// ```
pub struct CaptureEngine<V: VisionInference> {
    vision: V,
    database: Database,
}

impl<V: VisionInference> CaptureEngine<V> {
    /// Creates a new `CaptureEngine` with the given vision inference engine and database.
    pub fn new(vision: V, database: Database) -> Self {
        Self { vision, database }
    }

    /// Processes a single frame by analyzing it with the VLM and storing the result in the database.
    ///
    /// **Go Parallel:** This is a method on your struct that orchestrates a sequence
    /// of operations: call the VLM, get the result, and save to the DB.
    pub async fn process_frame(&self, frame: Frame) -> Result<SemanticLog> {
        let log = self.vision.analyze_frame(&frame).await?;
        self.database.insert_semantic_log(&log).await?;
        Ok(log)
    }

    /// Runs the main pipeline loop, receiving frames from the channel and processing them.
    /// This method blocks until the channel is closed.
    ///
    /// **Go Parallel:** This is your `for frame := range ch { ... }` loop.
    pub async fn run_pipeline(&self, mut rx: mpsc::Receiver<Frame>) -> Result<()> {
        info!("Starting pipeline loop");
        while let Some(frame) = rx.recv().await {
            let frame_id = frame.id;
            if let Err(e) = self.process_frame(frame).await {
                // Log the error but don't crash the loop.
                // Design §5.B: Resilience via graceful error handling.
                error!("Error processing frame {}: {:?}", frame_id, e);
            }
        }
        info!("Pipeline loop ended (channel closed)");
        Ok(())
    }
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

        let result: Result<SemanticLog> = engine.process_frame(frame.clone()).await;
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

        let log = engine.process_frame(frame).await.unwrap();
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

        let result: Result<SemanticLog> = engine.process_frame(frame).await;
        assert!(result.is_err());
        match result {
            Err(ChronosError::Inference(msg)) => assert_eq!(msg, "VLM error"),
            _ => panic!("Expected Inference error"),
        }
    }

    #[tokio::test]
    async fn test_pipeline_processes_multiple_frames() {
        let db = Database::new_in_memory().await.unwrap();
        let vision = MockVision;
        let engine = CaptureEngine::new(vision, db);

        let (tx, rx) = mpsc::channel(10);

        // Spawn the pipeline in the background
        let engine_handle = tokio::spawn(async move {
            engine.run_pipeline(rx).await.unwrap();
            engine
        });

        // Send 5 frames
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

        // Drop sender to close channel and end loop
        drop(tx);

        let engine = engine_handle.await.unwrap();

        // Verify 5 logs stored
        let count = engine.database.get_log_count().await.unwrap();
        assert_eq!(count, 5);
    }

    #[tokio::test]
    async fn test_pipeline_handles_vision_error_gracefully() {
        let db = Database::new_in_memory().await.unwrap();
        let vision = FailingVision;
        let engine = CaptureEngine::new(vision, db);

        let (tx, rx) = mpsc::channel(10);
        let engine_handle = tokio::spawn(async move {
            engine.run_pipeline(rx).await.unwrap();
        });

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
        engine_handle.await.unwrap();

        // Pipeline should not crash, even if process_frame failed
    }
}
