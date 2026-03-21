# Task 8.2: Query & Status Handlers

**Objective:** Implement the logic to handle the `query` and `status` CLI commands by interacting with the database.

**Mental Map / Go Parallel:** This is akin to creating your handler functions in a Go CLI tool (`RunE` in Cobra). They receive the parsed arguments, instantiate dependencies (like a DB connection pool), execute the operation, and format the output to stdout.

**Implementation Steps:**
- [x] In `crates/chronos-daemon/src/main.rs` (or a dedicated `handlers.rs` module), create `async fn handle_query(from: Option<String>, to: Option<String>, limit: i64) -> anyhow::Result<()>`.
- [x] In `handle_query`, connect to the SQLite database (using `Database::new(...)`), parse the dates if provided, call `database.query_semantic_logs`, and print the results clearly to stdout.
- [x] Create `async fn handle_status() -> anyhow::Result<()>`.
- [x] In `handle_status`, connect to the SQLite database, call `database.get_log_count`, and print the current log count and basic status to stdout.
- [x] Run `cargo clippy -p chronos-daemon -- -D warnings` and `cargo test -p chronos-daemon`.

**Code Scaffolding:**
```rust
use crate::database::Database;
// ... (imports)

pub async fn handle_query(from: Option<String>, to: Option<String>, limit: i64) -> anyhow::Result<()> {
    // Instantiate DB
    // Parse dates (you may need `chrono` for parsing the string to `DateTime<Utc>`)
    // Query DB
    // Print to stdout
    Ok(())
}

pub async fn handle_status() -> anyhow::Result<()> {
    // Instantiate DB
    // Get count
    // Print to stdout
    Ok(())
}
```

**Conventional Commit:** `feat(chronos-daemon): implement query and status cli handlers`
