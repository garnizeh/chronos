# Task 3.2: Implement Mock Traits and Tests

**Objective:** Create `MockCapture` and `MockVision` structs implementing their respective traits. Write tests validating dynamic dispatch and deterministic outputs.

**Mental Map / Go Parallel:** This is exactly like writing struct stubs for Go interfaces locally to test domain logic without touching the database or external APIs. Rust's `Box<dyn Trait>` provides the same dynamic dispatch capability as Go's interface variables.

**Implementation Steps:**
- [ ] Append to `crates/chronos-core/src/traits.rs`.
- [ ] Write `#[cfg(test)]` block.
- [ ] Define `MockCapture` struct and implement `ImageCapture` (returns static PNG bytes).
- [ ] Define `MockVision` struct and implement `VisionInference` (returns static `SemanticLog`).
- [ ] Write unit tests verifying that the mock implementations satisfy the traits and can be boxed natively via `Box<dyn ImageCapture>`.
- [ ] Run `cargo test -p chronos-core`.
- [ ] **Conventional Commit:** `git add . && git commit -m "test(chronos-core): implement mock capture and vision traits"`
  > ⚠️ **CRITICAL RULE:** The AI and Developer MUST execute this `git commit` *before* moving to or beginning the execution of the next sequential task.

**Code Scaffolding:**
```rust
#[cfg(test)]
pub mod mocks {
    use super::*;
    use chrono::Utc;
    use ulid::Ulid;

    pub struct MockCapture;
    
    // ... Implement async trait for MockCapture
}
```
