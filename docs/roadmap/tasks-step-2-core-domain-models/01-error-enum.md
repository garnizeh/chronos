# Task 2.1: Implement ChronosError

**Objective:** Create the central error enumeration that will be used across the entire workspace to represent domain errors. Define a custom `Result` alias.

**Mental Map / Go Parallel:** In Go, you define custom errors with `errors.New("capture error")` or wrap them using `fmt.Errorf`. Rust's `thiserror` crate provides a declarative macro to auto-generate the `Display` and `std::error::Error` implementations, allowing exhaustive pattern matching via `match`.

**Implementation Steps:**
- [x] Create file `crates/chronos-core/src/error.rs`.
- [x] Write `#[cfg(test)]` block with tests to verify error formatting and `From` conversions *before* implementing the logic.
- [x] Define the `ChronosError` enum with `Capture`, `Inference`, `Database`, `Config`, and `Timeout` variants.
- [x] Implement `From<sqlx::Error>` and `From<reqwest::Error>` for automatic error conversion.
- [x] Define `pub type Result<T> = std::result::Result<T, ChronosError>;`.
- [x] Run `cargo test -p chronos-core` to verify.

**Code Scaffolding:**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChronosError {
    #[error("Capture error: {0}")]
    Capture(String),
    
    // Add Inference, Database, Config, Timeout
    
    // Example of automatic From conversion for external crate errors:
    // #[error("Database error: {0}")]
    // DatabaseExt(#[from] sqlx::Error),
}

pub type Result<T> = std::result::Result<T, ChronosError>;
```

**Conventional Commit:**
```
feat(chronos-core): implement central ChronosError enum
```
