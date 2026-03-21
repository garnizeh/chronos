# Task 7.3: Async Pipeline Loop

**Objective:** Implement the `run_pipeline` method to continuously process frames arriving from a channel.

**Mental Map / Go Parallel:** This is your `for frame := range ch { ... }` loop running inside a goroutine. In Rust, we loop over a `Receiver` with `while let Some(frame) = rx.recv().await { ... }`, handling errors inside the loop to avoid crashing the whole pipeline.

**Implementation Steps:**
- [ ] Implement `pub async fn run_pipeline(&self, mut rx: tokio::sync::mpsc::Receiver<Frame>) -> Result<()>` on `CaptureEngine`.
- [ ] Inside `run_pipeline`, loop over `rx.recv().await`.
- [ ] Call `process_frame` for each frame. Warn or log errors but do not break the loop on VLM or DB failures (implement backoff/resilience).
- [ ] Ensure the loop exits gracefully if the channel is dropped.
- [ ] Write a test `test_pipeline_processes_multiple_frames` sending e.g., 5 frames through the channel and asserting they are processed.
- [ ] Run `cargo fmt --all`, `cargo clippy -p chronos-daemon -- -D warnings` and `cargo test --workspace`.
- [ ] Read and enforce any `// [JUSTIFIED GAP]` coverage exceptions if any hardware bound needs an exception. Make sure coverage passes.

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
