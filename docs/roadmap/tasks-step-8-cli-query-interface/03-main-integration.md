# Task 8.3: Main Integration & Lifecycle Handlers

**Objective:** Implement the remaining command handlers, refactor the `main` function to route based on the CLI subcommand, and bind the pipeline to the `start` command.

**Mental Map / Go Parallel:** In Go, your `main()` would call `Execute()` on the root command, routing the execution flow. Here, we match on the parsed `enum` to execute the appropriate asynchronous handler, leveraging `tokio` for the async runtime.

**Implementation Steps:**
- [ ] In `crates/chronos-daemon/src/main.rs`, update the `main` function to parse the `Cli` arguments and route to the corresponding handlers (using a `match` on `cli.command`).
- [ ] Create `async fn handle_start() -> anyhow::Result<()>`. Move the existing daemon initialization logic (DB, Capture, Vision, Engine, and `run_pipeline`) into this function.
- [ ] Create stub handlers for `handle_pause()` and `handle_resume()` that print a "Not yet implemented in MVP" message for now.
- [ ] Update `main` signature to return `anyhow::Result<()>`.
- [ ] Run `cargo clippy -p chronos-daemon -- -D warnings` and `cargo test -p chronos-daemon`.
- [ ] Perform a manual check: Run `cargo run -p chronos-daemon -- --help`, `cargo run -p chronos-daemon -- status`.

**Code Scaffolding:**
```rust
use clap::Parser;
use crate::cli::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Start => handle_start().await?,
        Commands::Query { from, to, limit } => handle_query(from, to, limit).await?,
        Commands::Status => handle_status().await?,
        Commands::Pause => handle_pause()?,
        Commands::Resume => handle_resume()?,
    }
    
    Ok(())
}

async fn handle_start() -> anyhow::Result<()> {
    // Pipeline initialization logic goes here
    Ok(())
}

fn handle_pause() -> anyhow::Result<()> {
    println!("Pause command not yet implemented in v0.1");
    Ok(())
}

fn handle_resume() -> anyhow::Result<()> {
    println!("Resume command not yet implemented in v0.1");
    Ok(())
}
```

**Conventional Commit:** `feat(chronos-daemon): integrate cli commands and update main lifecycle`
