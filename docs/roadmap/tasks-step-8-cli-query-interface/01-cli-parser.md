# Task 8.1: CLI Parser & Data Structures

**Objective:** Define the `Cli` struct and `Commands` enum using the `clap` crate, and configure top-level error handling with `anyhow`.

**Mental Map / Go Parallel:** In Go, you'd define flags or use a library like `cobra` to parse the command-line arguments into a struct. In Rust, `clap` coupled with the `derive` macro achieves this declaratively, generating the help text and validating inputs at compile time.

**Implementation Steps:**
- [x] Add `anyhow` to `crates/chronos-daemon/Cargo.toml`.
- [x] Create `crates/chronos-daemon/src/cli.rs`.
- [x] Define the `Cli` struct with `#[derive(Parser)]` and `#[command(name = "chronos", ...)]`.
- [x] Define the `Commands` enum with `#[derive(Subcommand)]` including variants: `Start`, `Query`, `Status`, `Pause`, `Resume`.
- [x] Ensure `Query` has `--from`, `--to` (Optional Strings for dates) and `--limit` (defaulting to 10).
- [x] Add `pub mod cli;` to `crates/chronos-daemon/src/lib.rs`.
- [x] Write a `#[cfg(test)]` block in `cli.rs` with tests: `test_cli_parse_start`, `test_cli_parse_query_with_dates`, `test_cli_parse_query_defaults`, `test_cli_parse_status`.
- [x] Run `cargo clippy -p chronos-daemon -- -D warnings` and `cargo test -p chronos-daemon`.

**Code Scaffolding:**
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "chronos", about = "Your personal context engine", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start,
    Query {
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        to: Option<String>,
        #[arg(long, default_value = "10")]
        limit: i64,
    },
    Status,
    Pause,
    Resume,
}
```

**Conventional Commit:** `feat(chronos-daemon): implement cli parser and data structures`
