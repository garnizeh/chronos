# Task 7.1: CaptureEngine Struct & Constructor

**Objective:** Define the `CaptureEngine` struct and its constructor to hold dependencies for the pipeline.

**Mental Map / Go Parallel:** This is equivalent to a struct taking interfaces and a DB connection in Go (e.g., `type CaptureEngine struct { vision VisionInference, db *Database }`). We use generics (`<V: VisionInference>`) to accept any implementation of the trait, allowing for easy dependency injection of mocks during testing.

**Implementation Steps:**
- [ ] Create `crates/chronos-daemon/src/pipeline.rs`.
- [ ] Define the `CaptureEngine<V: VisionInference>` struct containing `vision` and `database`.
- [ ] Implement `CaptureEngine::new(vision: V, database: Database) -> Self`.
- [ ] Add `pub mod pipeline;` to `crates/chronos-daemon/src/lib.rs` (if it exists, else we use it in `main.rs`) or ensure it's accessible.
- [ ] Write a basic `#[cfg(test)]` block to verify the struct can be instantiated with `MockVision` and an in-memory `Database`.
- [ ] Run `cargo clippy -p chronos-daemon -- -D warnings` and `cargo test -p chronos-daemon`.

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
