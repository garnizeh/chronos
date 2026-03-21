# Task 2.2c: Implement Capture and VLM Config Models

**Objective:** Define `CaptureConfig` and `VlmConfig` along with their default behaviors.

**Mental Map / Go Parallel:** In Go, you often write a `NewDefaultConfig()` function. In Rust, we implement the `Default` trait, which allows standard idiomatic instantiation via `CaptureConfig::default()`.

**Implementation Steps:**
- [ ] Append to `crates/chronos-core/src/models.rs`.
- [ ] Write `#[cfg(test)]` to assert that `CaptureConfig::default()` and `VlmConfig::default()` return the exact documented values.
- [ ] Define `CaptureConfig` and implement `Default`.
- [ ] Define `VlmConfig` and implement `Default`.
- [ ] Run `cargo test -p chronos-core` to verify defaults.

**Code Scaffolding:**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureConfig {
    pub interval_seconds: u64,
    pub ring_buffer_capacity: usize,
}

impl Default for CaptureConfig {
    fn default() -> Self {
        Self {
            interval_seconds: 30,
            ring_buffer_capacity: 64,
        }
    }
}

// Do the same for VlmConfig
```

**Conventional Commit:**
```
feat(chronos-core): add default configs for capture and vlm
```
