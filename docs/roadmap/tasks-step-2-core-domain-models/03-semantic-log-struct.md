# Task 2.2b: Implement SemanticLog Model

**Objective:** Define the `SemanticLog` struct which represents the structured output from the Vision-Language Model (VLM).

**Mental Map / Go Parallel:** Like a Go struct with `json:"..."` tags, we rely on serde's `Serialize` and `Deserialize` to convert this struct to and from JSON, making mapping to HTTP request JSON payloads or SQLite columns seamless.

**Implementation Steps:**
- [x] Append to `crates/chronos-core/src/models.rs`.
- [x] Write `#[cfg(test)]` block to verify serialization and deserialization round trips (JSON string -> Object -> JSON string).
- [x] Define `SemanticLog` struct with proper Option types for nullable fields.
- [x] Add `#[derive(Debug, Clone, Serialize, Deserialize)]`.
- [x] Run `cargo test -p chronos-core` to verify mapping logic.

**Code Scaffolding:**
```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticLog {
    pub id: Ulid,
    pub timestamp: DateTime<Utc>,
    pub source_frame_id: Ulid,
    pub description: String,
    pub active_application: Option<String>,
    pub activity_category: Option<String>,
    pub key_entities: Vec<String>,
    pub confidence_score: f64,
    pub raw_vlm_response: String,
}
```

**Conventional Commit:**
```
feat(chronos-core): add SemanticLog serialization model
```
