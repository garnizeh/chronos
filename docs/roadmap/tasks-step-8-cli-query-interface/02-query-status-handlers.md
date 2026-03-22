# Task 8.2: Query & Status Handlers

**Objective:** Implement the logic to handle the `query` and `status` CLI commands by interacting with the database.

**Mental Map / Go Parallel:** This is akin to creating your handler functions in a Go CLI tool (`RunE` in Cobra). They receive the parsed arguments, instantiate dependencies (like a DB connection pool), execute the operation, and format the output to stdout.

**Implementation Steps:**
- [x] In `crates/chronos-daemon/src/main.rs` (or a dedicated `handlers.rs` module), create `async fn handle_query(db: &Database, from: Option<String>, to: Option<String>, limit: u64) -> anyhow::Result<()>`.
- [x] In `handle_query`, connect to the SQLite database (using `Database::new(...)`), parse the dates if provided, call `Database::get_logs_by_date_range` or `Database::get_recent_logs`, and print the results clearly to stdout.
- [x] Create `async fn handle_status(db: &Database, url: &str) -> anyhow::Result<()>`.
- [x] In `handle_status`, connect to the SQLite database, call `database.get_log_count`, and print the current log count and basic status to stdout.
- [x] Run `cargo clippy -p chronos-daemon -- -D warnings` and `cargo test -p chronos-daemon`.

**Code Scaffolding:**
```rust
use crate::database::Database;
// ... (imports)

pub async fn handle_query(db: &Database, from: Option<String>, to: Option<String>, limit: u64) -> anyhow::Result<()> {
    // Instantiate DB
    // Parse dates (you may need `chrono` for parsing the string to `DateTime<Utc>`)
    // Query DB (using db.get_logs_by_date_range or db.get_recent_logs)
    // Print to stdout
    Ok(())
}

pub async fn handle_status(db: &Database, url: &str) -> anyhow::Result<()> {
    // Instantiate DB
    // Get count (using db.get_log_count)
    // Print to stdout
    Ok(())
}
```

**Conventional Commit:** `feat(chronos-daemon): implement query and status cli handlers`
