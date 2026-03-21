# Task 4.4: Integration Tests for Database Layer

## Objective
Verify the database implementation using in-memory SQLite tests.

## Mental Map / Go Parallel
In Go, you'd often use a real SQLite file or a mock for testing `database/sql`. In Rust with `sqlx`, we can use `:memory:` to run isolated, fast integration tests that exercise the real SQL migrations and queries.

## Implementation Steps
- [ ] In `crates/chronos-daemon/src/database.rs` (inside `#[cfg(test)]` block):
  - [ ] Write `test_insert_and_query_round_trip`:
    - [ ] Create an in-memory database.
    - [ ] Insert a `SemanticLog`.
    - [ ] Query it back and assert all fields match.
  - [ ] Write `test_get_logs_by_date_range`:
    - [ ] Insert logs at different times.
    - [ ] Query a range and verify only the expected logs are returned.
  - [ ] Write `test_get_recent_logs_respects_limit`:
    - [ ] Insert 10 logs.
    - [ ] Query with limit 3 and verify the most 3 recent are returned.
  - [ ] Write `test_empty_database_returns_zero_count`.
- [ ] Run `cargo test -p chronos-daemon` and ensure all pass.
- [ ] Run `cargo clippy -p chronos-daemon -- -D warnings` to ensure no lint warnings.

## Conventional Commit
`test(chronos-daemon): add integration tests for database layer`
