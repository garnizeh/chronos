# Task 4.1: SQL Migrations

## Objective
Create the initial SQLite schema for persisting semantic logs.

## Mental Map / Go Parallel
In Go, you'd use `golang-migrate` or `goose` for SQL migrations. Rust's `sqlx` has a built-in `sqlx::migrate!()` macro that embeds migrations at compile time — like `go:embed` for SQL files.

## Implementation Steps
- [x] Create directory `migrations/` at the workspace root if it doesn't exist.
- [x] Create file `migrations/001_create_semantic_logs.sql` with the following schema:
  ```sql
  -- UP: Create the semantic_logs table
  CREATE TABLE IF NOT EXISTS semantic_logs (
      id              TEXT PRIMARY KEY NOT NULL,    -- ULID as text
      timestamp       TEXT NOT NULL,                -- ISO 8601
      source_frame_id TEXT NOT NULL,                -- ULID of the originating frame
      description     TEXT NOT NULL,                -- VLM-generated description
      active_application TEXT,                      -- Detected active window
      activity_category  TEXT,                      -- Classified activity type
      key_entities    TEXT NOT NULL DEFAULT '[]',   -- JSON array of strings
      confidence_score REAL NOT NULL DEFAULT 0.0,   -- 0.0 to 1.0
      raw_vlm_response TEXT NOT NULL,               -- Full VLM JSON response
      created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
  );

  -- Index for time-range queries
  CREATE INDEX IF NOT EXISTS idx_semantic_logs_timestamp ON semantic_logs(timestamp);

  -- Index for filtering by application
  CREATE INDEX IF NOT EXISTS idx_semantic_logs_app ON semantic_logs(active_application);
  ```
- [ ] Run `sqlx migrate run` (or equivalent via `just` if available) to verify the SQL syntax.

## Conventional Commit
`feat(chronos-daemon): add initial sqlite migration for semantic logs`
