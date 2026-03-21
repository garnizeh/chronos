# Task 4.3: CRUD Operations for Semantic Logs

## Objective
Implement methods to insert and query semantic logs from the SQLite database.

## Mental Map / Go Parallel
This is your `database/sql` + `sqlc` pattern. But `sqlx` validates SQL at compile time via `query!` macros — like `sqlc generate` but integrated into the build. For runtime queries we'll use `sqlx::query` (non-macro) with `.bind()` to avoid needing a live database at compile time.

## Implementation Steps
- [ ] In `crates/chronos-daemon/src/database.rs`:
  - [ ] Implement `insert_semantic_log(&self, log: &SemanticLog) -> Result<()>`:
    - [ ] Serialize `key_entities` to a JSON string.
    - [ ] Execute `INSERT INTO semantic_logs (...) VALUES (...)`.
  - [ ] Implement `get_logs_by_date_range(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<SemanticLog>>`:
    - [ ] Execute `SELECT * FROM semantic_logs WHERE timestamp BETWEEN ? AND ?`.
    - [ ] Map rows back to `SemanticLog` structs.
  - [ ] Implement `get_recent_logs(&self, limit: i64) -> Result<Vec<SemanticLog>>`:
    - [ ] Execute `SELECT * FROM semantic_logs ORDER BY timestamp DESC LIMIT ?`.
  - [ ] Implement `get_log_count(&self) -> Result<i64>` for status reporting.
- [ ] Ensure all models used are imported from `chronos_core::models`.

## Code Scaffolding
```rust
use chronos_core::models::SemanticLog;
use chrono::{DateTime, Utc};

impl Database {
    pub async fn insert_semantic_log(&self, log: &SemanticLog) -> Result<()> {
        // TODO: Implementation
    }

    pub async fn get_logs_by_date_range(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>
    ) -> Result<Vec<SemanticLog>> {
        // TODO: Implementation
    }

    pub async fn get_recent_logs(&self, limit: i64) -> Result<Vec<SemanticLog>> {
        // TODO: Implementation
    }

    pub async fn get_log_count(&self) -> Result<i64> {
        // TODO: Implementation
    }
}
```

## Conventional Commit
`feat(chronos-daemon): implement crud operations for semantic logs`
