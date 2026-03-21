# Task 7.3: Async Pipeline Loop

**Objective:** Implement the `run_pipeline` method to continuously process frames arriving from a channel.

**Mental Map / Go Parallel:** This is your `for frame := range ch { ... }` loop running inside a goroutine. In Rust, we loop over a `Receiver` with `while let Some(frame) = rx.recv().await { ... }`, handling errors inside the loop to avoid crashing the whole pipeline.

**Implementation Steps:**
- [x] Implement `pub async fn run_pipeline(&self, mut rx: tokio::sync::mpsc::Receiver<Frame>) -> Result<()>` on `CaptureEngine`.
- [x] Inside `run_pipeline`, loop over `rx.recv().await` and pass borrowed references to `process_frame`.
- [x] Call `process_frame` for each frame with resilient error handling (exponential backoff retry for transient failures).
- [x] Ensure the loop exits gracefully if the channel is dropped.
- [x] Write a test `test_pipeline_processes_multiple_frames`.
- [x] Run `cargo fmt --all`, `cargo clippy`, and `cargo test --workspace`.
- [x] Read and enforce any `// [JUSTIFIED GAP]` coverage exceptions (coverage verified at >99%).

**Code Scaffolding:**
```rust
use tokio::sync::mpsc;
use chronos_core::models::Frame;
use chronos_core::error::Result;

impl<V: VisionInference> CaptureEngine<V> {
    pub async fn run_pipeline(&self, mut rx: mpsc::Receiver<Frame>) -> Result<()> {
        while let Some(frame) = rx.recv().await {
            // Process and handle errors without crashing
        }
        Ok(())
    }
}
```

**Conventional Commit:** `feat(chronos-daemon): implement async run_pipeline loop`
