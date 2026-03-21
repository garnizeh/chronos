# Prompt: Generate Chronos v0.1 MVP/POC — Detailed Roadmap & Task Breakdown

> **Purpose:** Feed this prompt to the AI agent to generate the complete v0.1 implementation roadmap.  
> **Input required:** The design document at `docs/design/0001-chronos-personal-context-engine.md`

---

## Prompt

```
You are a Senior Rust Systems Engineer building "Chronos" — a local-first screen context engine.

## Your Mission

Generate a comprehensive, detailed roadmap document for **Phase 0.1 (MVP/POC)** of Chronos. The document must contain every task, subtask, file to create, struct/trait to define, test to write, and verification step needed to go from an empty `cargo workspace` to a fully functional MVP.

## Context: Read These First

1. **Design Document** — Read the FULL design document at `docs/design/0001-chronos-personal-context-engine.md`. Every architectural decision, trait boundary, crate choice, and edge case documented there is canon. Do NOT deviate from it.

2. **User Rules** — These are MANDATORY constraints that override everything else:
   - **Privacy:** NEVER send data to external APIs. All AI inference via local Ollama on localhost. Zero outbound HTTP.
   - **No Disk Images:** Screen capture frames stay in RAM only (ring buffer). No `.png`/`.jpg` on disk.
   - **SQLite Only:** Use `sqlx` with direct SQL queries. No heavy ORMs.
   - **Didactic Mode:** The user is a Senior Go engineer learning Rust. Comment the *why*, not the *what*. Draw Go→Rust parallels (traits↔interfaces, Result↔(val,err), ownership↔GC absence).
   - **TDD:** Every module must have `#[cfg(test)]` tests in the same file. Tests are teaching tools.
   - **Small Scopes:** Never write huge modules. Build → Test → Verify → Pause for feedback.
   - **Trait Boundaries:** Core logic uses traits (`ImageCapture`, `VisionInference`, etc.). Mocks for all traits.
   - **Dependency Injection:** Pass `T: Trait` or `Box<dyn Trait>` — no hardcoded implementations in core.

## What the v0.1 MVP Includes (from the Design Doc)

- Screen capture every 30s (X11 via `xcap` or `scrap`)
- VLM inference via Ollama HTTP API (Moondream2)
- Semantic logs stored in SQLite via `sqlx`
- Basic CLI (`chronos query`, `chronos status`) via `clap`
- Comprehensive unit tests with mock capture and mock vision
- Cargo workspace with 4 crates
- Linux X11 only (no Wayland yet)

## Document Structure Required

Generate the roadmap as a Markdown document with this exact structure:

### 1. Workspace Setup
- Cargo workspace layout (`Cargo.toml` at root, member crates)
- Exact crate names and their responsibilities
- Shared dependencies and feature flags
- Directory tree showing every file that will be created

### 2. Implementation Phases (ordered by dependency)
Break the MVP into **sequential phases** where each phase is a compilable, testable unit. For each phase:

#### Phase N: [Name]
- **Goal:** 1-sentence summary of what this phase achieves
- **Depends on:** Which prior phases must be complete
- **Crate(s):** Which workspace crate(s) are touched
- **Tasks:** Numbered checklist of specific implementation tasks. Each task must specify:
  - The exact file path (e.g., `crates/chronos-core/src/models.rs`)
  - The structs, traits, or functions to implement
  - The corresponding test(s) to write
  - The `cargo check` / `cargo test` verification command
- **Acceptance Criteria:** What must pass before moving to the next phase
- **Pause Point:** ✋ Stop here and wait for user review

### 3. Suggested Phases (minimum)

The roadmap MUST include at least these phases in this order:

1. **Workspace Skeleton** — Create the cargo workspace, define crate boundaries, add dependencies to each `Cargo.toml`. Verify: `cargo check` passes on empty crates.

2. **Core Domain Models** — Define `Frame`, `SemanticLog`, `CaptureConfig`, `VlmConfig`, error types. All in `chronos-core`. Include `serde` derives. Tests: instantiation, serialization round-trip.

3. **Trait Boundaries** — Define `ImageCapture`, `VisionInference` traits in `chronos-core`. Define mock implementations (`MockCapture`, `MockVision`). Tests: mock capture returns static bytes, mock vision returns hardcoded JSON.

4. **Database Layer** — SQLite setup via `sqlx`. Create migration files. `Database` struct with `insert_semantic_log`, `get_logs_by_date`, `get_log_count`. Tests: in-memory SQLite, insert/query round-trip.

5. **Screen Capture (X11)** — Implement `X11Capture` in `chronos-capture` crate. Ring buffer (`VecDeque<Frame>`). Dedicated `std::thread`. Bridge to async via `tokio::sync::mpsc`. Tests: mock-based (no real screen needed). Integration test: capture thread sends frames through channel.

6. **Ollama Vision Client** — Implement `OllamaVision` in `chronos-inference` crate. HTTP POST to `localhost:11434/api/generate`. JSON parsing with fallback. Timeout handling. Tests: mock HTTP server (e.g., `wiremock` or `mockito`), test valid/malformed/timeout responses.

7. **Pipeline Integration** — Wire capture → vision → database in `chronos-daemon`. The main async loop: receive frame from channel, send to VLM, store result. Tests: end-to-end with mocks (MockCapture → MockVision → in-memory SQLite).

8. **CLI** — `chronos query`, `chronos status`, `chronos pause`, `chronos resume` via `clap`. Query reads from SQLite. Status shows capture state + log count. Tests: CLI argument parsing, query with seeded database.

9. **Integration & Smoke Test** — Full pipeline test with all mocks. Verify: `cargo test --workspace` passes. `cargo clippy -- -D warnings` clean. `cargo fmt --check` passes.

### 4. Verification Matrix
A table mapping each component to its test strategy:

| Component | Unit Test | Integration Test | Mock Used | Verification Command |
|-----------|-----------|-----------------|-----------|---------------------|
| ... | ... | ... | ... | ... |

### 5. Definition of Done (v0.1)
Exact checklist of what "MVP complete" means. Include:
- All `cargo test --workspace` green
- All `cargo clippy -- -D warnings` clean
- All traits have mock implementations
- All modules have `#[cfg(test)]` blocks
- README with setup instructions
- `chronos status` and `chronos query` working against mock data

## Style Rules for the Document
- Use Rust code snippets to illustrate key structs/traits (not full implementations)
- Mark every pause point with ✋
- Use `[ ]` checkbox format for tasks
- Include Go→Rust comparison notes where relevant (e.g., "In Go, you'd use `database/sql` + `sqlc`; in Rust, `sqlx::query!` does compile-time SQL validation")
- Reference the design doc sections by number (e.g., "See Design §4.A")
- Keep each phase to a size that can be implemented in a single coding session (~30-60 min)

## Skills to Activate

Read and apply these skills before generating:
- `/Rust Feature TDD Loop` — Follow the TDD workflow for each module
- `/Verify Cargo Workspace` — Use the verification suite after structural changes
- `/Local SQLite & SQLx Pipeline` — Follow the SQLx workflow for database setup
- `/OS & Async Boundary Control` — Apply for the screen capture thread boundary

Additionally, follow these external skill patterns:
- `rust-pro` — Modern Rust 1.75+ patterns, workspace organization
- `rust-async-patterns` — Tokio async/await, channels, spawn_blocking
- `clean-code` — Clean Code principles (Uncle Bob) applied to Rust
- `tdd-workflow` — RED-GREEN-REFACTOR cycle for every component
- `architecture` — ADR-informed architectural decisions
- `plan-writing` — Structured task planning with dependencies and verification
- `debugger` — Systematic debugging when tests fail
- `database-design` — Schema design principles for the SQLite layer
- `api-patterns` — REST API patterns for the Ollama client
- `bash-pro` — For any shell scripts (install, run, test)

## Output

Save the generated roadmap to: `docs/roadmap/0001-phase-01-mvp-roadmap.md`
```

---

## How to Use This Prompt

1. Open a **new conversation** with the AI agent
2. Ensure the design document is accessible at `docs/design/0001-chronos-personal-context-engine.md`
3. Paste or reference this prompt
4. The agent will generate the complete roadmap document
5. Review the roadmap, iterate if needed, then begin Phase 1

## Skills Reference (for quick activation)

| Skill | Purpose in This Context |
|---|---|
| `rust-pro` | Modern Rust patterns, workspace setup, error handling |
| `rust-async-patterns` | Tokio runtime, channels, hybrid threading |
| `tdd-workflow` | RED→GREEN→REFACTOR for every module |
| `clean-code` | Naming, function size, SRP |
| `architecture` | ADR documentation, dependency analysis |
| `plan-writing` | Structured task breakdowns |
| `database-design` | SQLite schema, migration strategy |
| `api-patterns` | HTTP client design for Ollama |
| `debugger` | Systematic debugging protocol |
| `bash-pro` | Shell scripts for build/test/run |
| `/Rust Feature TDD Loop` | Project-specific TDD workflow |
| `/Verify Cargo Workspace` | Project-specific compilation suite |
| `/Local SQLite & SQLx Pipeline` | Project-specific database workflow |
| `/OS & Async Boundary Control` | Project-specific async boundary pattern |
