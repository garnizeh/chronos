---
description: Defines the Rust workspace structure and boundaries for Project Chronos.
globs: **/*.rs
---
# Workspace Architecture & Boundaries

This project uses a Cargo Workspace. Never put all logic into a single crate. Respect the following boundaries:

- `chronos-capture`: Strictly for OS-level screen capture and IO. No database logic here.
- `chronos-vision`: Strictly for local VLM inference (Ollama/candle).
- `chronos-synth`: The LLM logic for summarizing logs.
- `chronos-db`: The ONLY crate allowed to use `sqlx` and SQLite. 

**Terminal Commands:**
When running checks or tests, always use workspace-aware flags. 
Use `cargo check --workspace` to check the whole project, or `cargo test -p <crate_name>` to test a specific module.