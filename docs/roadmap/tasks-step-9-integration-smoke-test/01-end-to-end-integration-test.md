# Task 9.1: End-to-End Integration Test

**Objective:** Write a full end-to-end integration test in `chronos-daemon` using `MockCapture` and `MockVision` to verify the complete pipeline without hardware dependencies.

**Mental Map / Go Parallel:** This is similar to a full package-level integration test in Go using `httptest` or `net/http/httptest` to stub out external endpoints while verifying database inserts and channels. In Rust, we use our mocked trait implementations.

**Implementation Steps:**
- [x] In `crates/chronos-daemon/src/pipeline.rs` (or a dedicated integration test file), create `test_full_pipeline_mock_end_to_end`.
- [x] Initialize `MockCapture`, `MockVision`, and an in-memory `Database` (`sqlite::memory:`).
- [x] Create the `CaptureEngine` with these dependencies.
- [x] Run the pipeline or simulate sending multiple frames (e.g., 3 frames) through it.
- [x] Query the database to verify 3 logs were successfully stored.
- [x] Query the database by date range and verify the results match expectations.
- [x] Verify `database.get_log_count()` returns 3.
- [x] Run `cargo test -p chronos-daemon` to ensure the new test passes.

**Code Scaffolding:**
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use chronos_core::traits::mocks::{MockCapture, MockVision};
    // ... imports

    #[tokio::test]
    async fn test_full_pipeline_mock_end_to_end() {
        // Setup mocks and db
        // ...
        
        // Assertions
        // ...
    }
}
```

**Conventional Commit:** `test(chronos-daemon): add end-to-end mock pipeline test`
