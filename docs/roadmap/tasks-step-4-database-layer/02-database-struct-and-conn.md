# Task 4.2: Database Struct and Connection

## Objective
Initialize the `Database` struct and handle connection pooling with SQLite.

## Mental Map / Go Parallel
This is your `database/sql` pattern. `sqlx` provides a `Pool<Sqlite>` which is thread-safe and can be shared across the app. We'll wrap it in a `Database` struct.

## Implementation Steps
- [x] In `crates/chronos-daemon/src/database.rs`:
  - [x] Define the `Database` struct:
    ```rust
    pub struct Database {
        pool: sqlx::SqlitePool,
    }
    ```
  - [x] Implement `Database::new(database_url: &str) -> Result<Self>`:
    - [x] Create connection pool using `SqlitePoolOptions`.
    - [x] Run migrations using `sqlx::migrate!().run(&pool).await`.
  - [x] Implement `Database::new_in_memory() -> Result<Self>` for testing.
- [x] Add `sqlx` with `sqlite` and `runtime-tokio` features to `chronos-daemon/Cargo.toml`.
- [x] Add `chronos-core` as a dependency to access `Result` and `ChronosError`.

## Code Scaffolding
```rust
use chronos_core::error::Result;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(url: &str) -> Result<Self> {
        // TODO: Implementation
    }

    pub async fn new_in_memory() -> Result<Self> {
        // TODO: Implementation for tests
    }
}
```

## Conventional Commit
`feat(chronos-daemon): implement database connection pool and migrations`
