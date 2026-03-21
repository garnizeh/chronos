# Task 7.2: Process Single Frame Method

**Objective:** Implement the `process_frame` method to handle the logic of analyzing a single frame and storing its semantic log.

**Mental Map / Go Parallel:** This is a method on your struct that orchestrates a sequence of operations: call the VLM, get the result, and save to the DB, returning early on error (`if err != nil { return err }` becomes `?` in Rust).

**Implementation Steps:**
- [ ] Implement `pub async fn process_frame(&self, frame: Frame) -> Result<SemanticLog>` on `CaptureEngine`.
- [ ] Call `self.vision.analyze_frame(&frame).await`.
- [ ] If successful, call `self.database.insert_semantic_log(&log).await`.
- [ ] Return the stored log.
- [ ] Write a test `test_process_frame_with_mocks` using `MockVision` and an in-memory DB to verify normal processing.
- [ ] Write a test `test_process_frame_stores_correct_source_frame_id` to verify IDs.
- [ ] Write a test `test_pipeline_handles_vision_error_gracefully` with a failing mock to ensure errors propagate correctly without panics.
- [ ] Run `cargo test -p chronos-daemon`.

**Code Scaffolding:**
```rust
use chronos_core::models::{Frame, SemanticLog};
use chronos_core::error::Result;

impl<V: VisionInference> CaptureEngine<V> {
    pub async fn process_frame(&self, frame: Frame) -> Result<SemanticLog> {
        // Implementation here
        todo!()
    }
}
```

**Conventional Commit:** `feat(chronos-daemon): implement process_frame for CaptureEngine`
