use async_trait::async_trait;
use crate::models::{Frame, SemanticLog};
use crate::error::Result;

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
