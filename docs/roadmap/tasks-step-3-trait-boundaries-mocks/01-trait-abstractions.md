# Task 3.1: Define Trait Abstractions

**Objective:** Define the `ImageCapture` and `VisionInference` traits that act as boundaries for all external I/O interactions.

**Mental Map / Go Parallel:** In Go, interfaces are satisfied implicitly and are automatically safe to pass across goroutines. In Rust, we define explicit `trait`s. To ensure they can be safely sent across Tokio threads, we must add `Send + Sync` bounds. Since async methods in traits are a bit tricky before Rust 1.75, we'll use the `async_trait` crate (or native async if supported, but `async-trait` is a widely accepted pragmatic standard).

**Implementation Steps:**
- [x] Add `async-trait = "0.1"` dependency to `crates/chronos-core/Cargo.toml`.
- [x] Create `crates/chronos-core/src/traits.rs`.
- [x] Define the `ImageCapture` trait returning `Result<Frame>`.
- [x] Define the `VisionInference` trait accepting `&Frame` and returning `Result<SemanticLog>`.
- [x] Run `cargo check -p chronos-core` to verify compilation.
- [x] **Conventional Commit:** `git commit -m "feat(chronos-core): define ImageCapture and VisionInference traits"`
  > ⚠️ **CRITICAL RULE:** The AI and Developer MUST execute this `git commit` *before* moving to or beginning the execution of the next sequential task.

**Code Scaffolding:**
```rust
use async_trait::async_trait;
use crate::models::{Frame, SemanticLog};
use crate::error::Result;

#[async_trait]
pub trait ImageCapture: Send + Sync {
    async fn capture_frame(&self) -> Result<Frame>;
}

#[async_trait]
pub trait VisionInference: Send + Sync {
    async fn analyze_frame(&self, frame: &Frame) -> Result<SemanticLog>;
}
```
