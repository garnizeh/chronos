# Task 2.2a: Implement Frame Model

**Objective:** Define the `Frame` struct that represents a single captured screen image. This struct resides purely in RAM to avoid SSD wear.

**Mental Map / Go Parallel:** This is a simple data struct. Since we don't want to serialize `Frame` to disk or over the network, we explicitly *do not* implement `Serialize`/`Deserialize` (akin to not adding `json:"..."` struct tags in Go), enforcing boundary constraints at compile time.

**Implementation Steps:**
- [ ] Create file `crates/chronos-core/src/models.rs`.
- [ ] Write `#[cfg(test)]` block mapping `Frame` instantiation and verifying its properties.
- [ ] Define `Frame` struct containing ULID, timestamp, image bytes, width, and height.
- [ ] Add `#[derive(Debug, Clone)]` but **NO** `Serialize`/`Deserialize`.
- [ ] Run `cargo test -p chronos-core` to ensure it compiles and tests pass.

**Code Scaffolding:**
```rust
use chrono::{DateTime, Utc};
use ulid::Ulid;

#[derive(Debug, Clone)]
pub struct Frame {
    pub id: Ulid,
    pub timestamp: DateTime<Utc>,
    pub image_data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}
```

**Conventional Commit:**
```
feat(chronos-core): add Frame domain model
```
