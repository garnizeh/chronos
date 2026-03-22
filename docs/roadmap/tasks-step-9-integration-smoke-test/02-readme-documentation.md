# Task 9.2: README Documentation

**Objective:** Create the root `README.md` file introducing the project, its privacy guarantees, and usage instructions for the v0.1 MVP.

**Mental Map / Go Parallel:** Much like writing a `README.md` for a new Go CLI tool, this needs to answer the "What, Why, and How" of the project instantly.

**Implementation Steps:**
- [x] In the root repository directory, edit the existing `README.md` or create it if missing.
- [x] Add the project name and description.
- [x] Add the privacy statement: "100% local, never sends data externally."
- [x] Add prerequisites: Rust 1.75+ and Ollama with `moondream` model pulled.
- [x] Add build instructions (`cargo build --workspace`).
- [x] Add Quick Start examples (e.g. `cargo run -p chronos-daemon -- start`).
- [x] Add a Query example (e.g. `cargo run -p chronos-daemon -- query --from 2025-01-01`).
- [x] Run `cargo fmt --all -- --check` to make sure formatting is clean.

**Conventional Commit:** `docs: add comprehensive readme for v0.1 mvp`
