//! Behavioral contracts for the Chronos pipeline.
//!
//! This module defines the traits that decouple our core orchestration logic
//! from specific implementations of hardware capture and AI inference.

use crate::error::Result;
use crate::models::{Frame, SemanticLog};
use async_trait::async_trait;

/// The screen capture abstraction.
///
/// **Go Parallel:** This is exactly like declaring a Go `type ImageCapture interface { ... }`.
/// Any struct that implements these methods automatically fulfills this contract.
///
/// **Why `Send + Sync`?**
/// In Go, interfaces are implicitly safe to pass across goroutines. Rust, however, requires
/// explicit guarantees.
/// - `Send`: Indicates ownership of this type can be safely transferred to another thread.
/// - `Sync`: Indicates it is safe for multiple threads to hold a shared reference (`&T`) to it.
/// Because our capture components will cross `tokio` async task boundaries (which act like goroutines
/// but are M:N mapped onto OS threads), these marker traits are strictly required.
#[async_trait]
pub trait ImageCapture: Send + Sync {
    /// Captures a single frame from the system's screen.
    /// Returns raw image bytes safely wrapped in our `Frame` domain model.
    async fn capture_frame(&self) -> Result<Frame>;

    /// Returns the configured capture interval in seconds.
    fn capture_interval_seconds(&self) -> u64 {
        30
    }
}

/// The vision-language model abstraction.
///
/// **Why abstract this?**
/// This trait isolates our core pipeline from knowing whether it's talking to a local `Ollama` instance,
/// a mock for testing, or a future cloud-based VLM. Clean architectural boundaries!
#[async_trait]
pub trait VisionInference: Send + Sync {
    /// Sends a frame's image data to the Vision-Language Model and parses
    /// the response into a structured `SemanticLog`.
    ///
    /// Notice how we take a shared reference `&Frame` here. We don't need to consume (take ownership of)
    /// the frame because the image data only needs to be read to be encoded and sent over the network.
    async fn analyze_frame(&self, frame: &Frame) -> Result<SemanticLog>;
}

#[cfg(any(test, feature = "mocks"))]
pub mod mocks {
    use super::*;
    use chrono::Utc;
    use ulid::Ulid;

    /// A test double that returns a static 1x1 pixel PNG.
    /// No real screen capture — works in CI, headless, everywhere.
    ///
    /// **Go Parallel:** This is a stub implementation used in testing, equivalent
    /// to creating a struct `mockCapture{}` that implements the `ImageCapture`
    /// interface purely for unit test deterministic behavior.
    pub struct MockCapture;

    #[async_trait]
    impl ImageCapture for MockCapture {
        async fn capture_frame(&self) -> Result<Frame> {
            Ok(Frame {
                id: Ulid::new(),
                timestamp: Utc::now(),
                // PNG magic bytes for a minimal valid-looking header
                image_data: vec![0x89, 0x50, 0x4E, 0x47],
                width: 1,
                height: 1,
            })
        }

        // [JUSTIFIED GAP]: Default implementation used.
    }

    /// A test double that returns a hardcoded semantic log.
    /// Simulates a VLM that always sees "User editing code in VSCode".
    pub struct MockVision;

    #[async_trait]
    impl VisionInference for MockVision {
        async fn analyze_frame(&self, frame: &Frame) -> Result<SemanticLog> {
            Ok(SemanticLog {
                id: Ulid::new(),
                timestamp: frame.timestamp, // In sync with frame timestamp
                source_frame_id: frame.id,
                description: "User editing code in VSCode".to_string(),
                active_application: Some("Visual Studio Code".to_string()),
                activity_category: Some("Development".to_string()),
                key_entities: vec!["Rust".to_string(), "main.rs".to_string()],
                confidence_score: 0.95,
                raw_vlm_response: r#"{"description":"User editing code"}"#.to_string(),
            })
        }
    }

    // Internal tests for the mocks — only run when testing chronos-core itself
    #[cfg(test)]
    mod internal_tests {
        use super::*;

        // Testing the mock capture implementation itself
        #[tokio::test]
        async fn test_mock_capture_returns_frame() {
            let capture = MockCapture;
            // Unwrap is perfectly fine within a test scope because panic = fail
            let frame = capture.capture_frame().await.unwrap();
            assert_eq!(frame.image_data, vec![0x89, 0x50, 0x4E, 0x47]);
            assert_eq!(frame.width, 1);
            assert_eq!(frame.height, 1);
        }

        // Testing the mock vision implementation
        #[tokio::test]
        async fn test_mock_vision_returns_semantic_log() {
            let capture = MockCapture;
            let frame = capture.capture_frame().await.unwrap();

            let vision = MockVision;
            let log = vision.analyze_frame(&frame).await.unwrap();

            // Ensure that the log properly tracked which frame it was created from
            assert_eq!(log.source_frame_id, frame.id);
            assert_eq!(log.description, "User editing code in VSCode");
            assert_eq!(log.confidence_score, 0.95);
        }

        // Verifying uniquely generated identifiers for frames
        #[tokio::test]
        async fn test_mock_capture_unique_ids() {
            let capture = MockCapture;
            let f1 = capture.capture_frame().await.unwrap();
            let f2 = capture.capture_frame().await.unwrap();
            assert_ne!(f1.id, f2.id); // Identifiers must not conflict
        }

        // Proving that Rust's dynamic dispatch works smoothly for Traits
        #[tokio::test]
        async fn test_trait_object_dispatch() {
            // Here we prove dynamic dispatch works natively with our Send + Sync trait
            // This is Rust's equivalent of an interface slice in Go
            let capture: Box<dyn ImageCapture> = Box::new(MockCapture);
            let frame = capture.capture_frame().await.unwrap();

            let vision: Box<dyn VisionInference> = Box::new(MockVision);
            let log = vision.analyze_frame(&frame).await.unwrap();

            assert_eq!(log.source_frame_id, frame.id);
        }
    }
}
