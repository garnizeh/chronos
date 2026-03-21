# Task 7.1: CaptureEngine Struct & Constructor

**Objective:** Define the `CaptureEngine` struct and its constructor to hold dependencies for the pipeline.

**Mental Map / Go Parallel:** This is equivalent to a struct taking interfaces and a DB connection in Go (e.g., `type CaptureEngine struct { vision VisionInference, db *Database }`). We use generics (`<V: VisionInference>`) to accept any implementation of the trait, allowing for easy dependency injection of mocks during testing.

**Implementation Steps:**
- [x] Create `crates/chronos-daemon/src/pipeline.rs`.
- [x] Define the `CaptureEngine<V: VisionInference>` struct containing `vision` and `database`.
- [x] Implement `CaptureEngine::new(vision: V, database: Database) -> Self`.
- [x] Add `pub mod pipeline;` to `crates/chronos-daemon/src/lib.rs`.
- [x] Write a basic `#[cfg(test)]` block to verify instantiation.
- [x] Run `cargo clippy` and `cargo test`.

**Code Scaffolding:**
```rust
use chronos_core::traits::VisionInference;
use crate::database::Database;

pub struct CaptureEngine<V: VisionInference> {
    vision: V,
    database: Database,
}

impl<V: VisionInference> CaptureEngine<V> {
    pub fn new(vision: V, database: Database) -> Self {
        Self { vision, database }
    }
}
```

**Conventional Commit:** `feat(chronos-daemon): implement CaptureEngine struct and constructor`
