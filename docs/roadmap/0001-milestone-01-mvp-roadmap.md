# Chronos v0.1 — Milestone 1: MVP/POC Roadmap & Task Breakdown

> **Source of truth:** [`docs/design/0001-chronos-personal-context-engine.md`](../design/0001-chronos-personal-context-engine.md)  
> **Prompt spec:** [`docs/prompt/0001-milestone-01-mvp.md`](../prompt/0001-milestone-01-mvp.md)  
> **Status:** In Progress — Step 3

---

## Table of Contents

1. [Workspace Setup](#1-workspace-setup)
2. [Implementation Steps](#2-implementation-steps)
   - [Step 0: Repository Bootstrapping](#step-0-repository-bootstrapping)
   - [Step 1: Workspace Skeleton](#step-1-workspace-skeleton)
   - [Step 2: Core Domain Models](#step-2-core-domain-models)
   - [Step 3: Trait Boundaries & Mocks](#step-3-trait-boundaries--mocks)
   - [Step 4: Database Layer](#step-4-database-layer)
   - [Step 5: Screen Capture (X11)](#step-5-screen-capture-x11)
   - [Step 6: Ollama Vision Client](#step-6-ollama-vision-client)
   - [Step 7: Pipeline Integration (Daemon)](#step-7-pipeline-integration-daemon)
   - [Step 8: CLI](#step-8-cli)
   - [Step 9: Integration & Smoke Test](#step-9-integration--smoke-test)
3. [Verification Matrix](#3-verification-matrix)
4. [Definition of Done (v0.1)](#4-definition-of-done-v01)

---

## 1. Workspace Setup

### Crate Layout (see Design §3.F)

Chronos uses a `cargo workspace` with **4 member crates**, each with a single clear responsibility. This mirrors how Go modules isolate packages — but in Rust, workspace crates share a single `Cargo.lock` and can be compiled together or independently.

| Crate | Responsibility | Key Dependencies |
|---|---|---|
| `chronos-core` | Domain models, traits, error types, shared config | `serde`, `serde_json`, `chrono`, `thiserror`, `ulid` |
| `chronos-capture` | X11 screen capture, ring buffer, OS thread management | `xcap`, `tokio` (channel only), `chronos-core` |
| `chronos-inference` | Ollama HTTP client, VLM request/response, JSON parsing | `reqwest`, `serde_json`, `base64`, `tokio`, `chronos-core` |
| `chronos-daemon` | Main binary — pipeline orchestration, CLI, SQLite, async runtime | `tokio`, `sqlx`, `clap`, `chronos-core`, `chronos-capture`, `chronos-inference` |

> **Go parallel:** Think of each crate as a Go module in a multi-module repo. `chronos-core` is your `pkg/` directory — shared types with zero external coupling. The daemon is your `cmd/chronos/` entry point.

### Directory Tree (target state at MVP completion)

```
chronos/
├── Cargo.toml                              # [workspace] members
├── Cargo.lock
├── .gitignore
├── README.md
│
├── crates/
│   ├── chronos-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                      # Re-exports
│   │       ├── models.rs                   # Frame, SemanticLog, CaptureConfig, VlmConfig
│   │       ├── error.rs                    # ChronosError enum (thiserror)
│   │       └── traits.rs                   # ImageCapture, VisionInference + mocks
│   │
│   ├── chronos-capture/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                      # Re-exports
│   │       ├── x11.rs                      # X11Capture: ImageCapture impl
│   │       └── ring_buffer.rs              # VecDeque<Frame> wrapper
│   │
│   ├── chronos-inference/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                      # Re-exports
│   │       └── ollama.rs                   # OllamaVision: VisionInference impl
│   │
│   └── chronos-daemon/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs                     # Tokio entrypoint, clap CLI
│           ├── cli.rs                      # Clap command definitions
│           ├── pipeline.rs                 # Capture → Vision → DB loop
│           └── database.rs                 # SQLite via sqlx: migrations, insert, query
│
├── migrations/
│   └── 001_create_semantic_logs.sql        # Initial schema
│
├── docs/                                   # (existing)
│   ├── design/
│   ├── idea/
│   ├── prompt/
│   ├── roadmap/                            # ← this document
│   └── reference/
│
└── .agents/                                # (existing)
    ├── rules/
    └── workflows/
```

---

## 2. Implementation Steps

Each phase is a self-contained, compilable, testable unit. Follow the `/Rust Feature TDD Loop` workflow (Step 1 → 4) for every module, and run `/Verify Cargo Workspace` after structural changes.

---

### Step 0: Repository Bootstrapping ✅

**Goal:** Initialise the git repository, create foundational project files (README, LICENSE, .gitignore), and set up a minimal GitHub Actions CI pipeline so that every future phase is automatically validated on push.

**Depends on:** Nothing (absolute starting point)

**Crate(s):** None — pure repository scaffolding

**Tasks:**

- [x] **0.1** Initialise git repository: `git init`
- [x] **0.2** Create `.gitignore` — standard Rust ignores plus project-specific entries:
  - `/target` — build artefacts
  - `*.swp`, `*.swo` — editor temp files
  - `.env` — local environment overrides
  - `*.db`, `*.db-journal` — SQLite files (never committed)
- [x] **0.3** Create `LICENSE` — MIT license
- [x] **0.4** Create `README.md` with initial content:
  - Project name and one-line description
  - Privacy statement: *"100% local-first. All AI inference runs on your machine via Ollama. No data ever leaves localhost."*
  - CI badge (GitHub Actions) + CodeRabbit reviews badge
  - "Under construction" notice
  - Link to design document
  - Prerequisites: Rust 1.94+, Cargo 1.94+, Ollama 0.17.4+
- [x] **0.5** Create `.github/workflows/ci.yml` — GitHub Actions CI pipeline:
  - **Triggers:** `push` to `main`, all `pull_request`s
  - **Job: `check`** (runs on `ubuntu-latest`):
    1. Checkout code
    2. Install Rust stable toolchain with `clippy`, `rustfmt`, and `llvm-tools-preview` components
    3. Install `cargo-llvm-cov` (via `taiki-e/install-action`)
    4. Cache `~/.cargo` and `target/` (via `Swatinem/rust-cache`)
    5. `cargo fmt --all -- --check`
    6. `cargo clippy --workspace --all-targets -- -D warnings`
    7. `cargo llvm-cov --workspace --lcov --output-path lcov.info` (tests + coverage)
    8. Upload `lcov.info` to Codecov (via `codecov/codecov-action@v5`)
  - **Note:** CI will only pass once Step 1 creates the workspace+crates. Step 0 intentionally commits this file first so the pipeline exists from day one.
- [x] **0.5b** Create `.github/workflows/release.yml` — automated releases via `release-please`:
  - **Triggers:** `push` to `main`
  - **Permissions:** `contents: write`, `pull-requests: write`
  - Uses `googleapis/release-please-action@v4` with **manifest-based config**:
    - `config-file: release-please-config.json` — declares `cargo-workspace` plugin + 4 crate packages
    - `manifest-file: .release-please-manifest.json` — tracks per-crate versions (all start at `0.0.0`)
  - Parses conventional commits (`feat:`, `fix:`, etc.) to auto-create Release PRs with bumped `Cargo.toml` versions, `CHANGELOG.md`, and GitHub Releases on merge.
- [x] **0.5c** Create `.agents/rules/08-conventional-commits.md` — always-on agent rule enforcing:
  - Conventional commit format (`<type>(<scope>): <description>`)
  - Allowed types: `feat`, `fix`, `chore`, `docs`, `test`, `refactor`, `perf`, `ci`
  - Breaking change syntax (`feat!:` or `BREAKING CHANGE:` footer)
  - Scope convention: use crate name (e.g., `feat(chronos-core): ...`)
  - Examples of good and bad commit messages
- [x] **0.5d** *(bonus)* Create `.github/workflows/auto-author-assign.yml` — auto-assigns PR author on open/reopen
- [x] **0.6** Initial commit + subsequent commits:
  - `chore: bootstrap repository (phase 0)` — README, LICENSE, .gitignore, CI, docs, .agents
  - Follow-up commits: CI badge fix, version requirements, release-please manifest config, CodeRabbit badge

> **Go parallel:** This is equivalent to `go mod init` + creating your `Makefile` with `lint`, `test`, `vet` targets, plus wiring up CI. The key insight: committing CI _before_ any code means every Pull Request is validated from PR #1.

**Acceptance Criteria:** ✅ All met
- ✅ `git log` shows 8 commits
- ✅ `.gitignore`, `LICENSE`, `README.md` exist at the repo root
- ✅ `.github/workflows/ci.yml` exists with fmt + clippy + coverage + codecov upload
- ✅ `.github/workflows/release.yml` exists with release-please (manifest-based config + `cargo-workspace` plugin)
- ✅ `release-please-config.json` + `.release-please-manifest.json` exist at the repo root
- ✅ `.agents/rules/08-conventional-commits.md` exists with always-on trigger
- ✅ `git status` is clean (no untracked files)

**✅ Step 0 complete — Reviewed 2025-03-20. Proceeding to Step 1.**

---

### Step 1: Workspace Skeleton ✅

> 📋 **Detailed tasks:** [`tasks-step-1-workspace-skeleton/`](tasks-step-1-workspace-skeleton/)

**Goal:** Create the cargo workspace with 4 empty crates. Verify that `cargo check --workspace` compiles cleanly.

**Depends on:** Step 0 complete

**Crate(s):** All — workspace root + 4 members

**Tasks:**

- [x] **1.1** Create root `Cargo.toml` with `[workspace]` definition listing all 4 member crates
- [x] **1.2** Create `crates/chronos-core/Cargo.toml` with initial dependencies:
  - `serde = { version = "1", features = ["derive"] }`
  - `serde_json = "1"`
  - `chrono = { version = "0.4", features = ["serde"] }`
  - `thiserror = "2"`
  - `ulid = { version = "1.1", features = ["serde"] }`
- [x] **1.3** Create `crates/chronos-core/src/lib.rs` with empty module declarations
- [x] **1.4** Create `crates/chronos-capture/Cargo.toml` with dependencies:
  - `chronos-core = { path = "../chronos-core" }`
  - `xcap = "0.0.13"` (or latest)
  - `tokio = { version = "1", features = ["sync"] }`  *(channel only — no full runtime)*
- [x] **1.5** Create `crates/chronos-capture/src/lib.rs` stub
- [x] **1.6** Create `crates/chronos-inference/Cargo.toml` with dependencies:
  - `chronos-core = { path = "../chronos-core" }`
  - `reqwest = { version = "0.12", features = ["json"] }`
  - `serde_json = "1"`
  - `base64 = "0.22"`
  - `tokio = { version = "1", features = ["full"] }`
- [x] **1.7** Create `crates/chronos-inference/src/lib.rs` stub
- [x] **1.8** Create `crates/chronos-daemon/Cargo.toml` with dependencies:
  - `chronos-core = { path = "../chronos-core" }`
  - `chronos-capture = { path = "../chronos-capture" }`
  - `chronos-inference = { path = "../chronos-inference" }`
  - `tokio = { version = "1", features = ["full"] }`
  - `sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }`
  - `clap = { version = "4", features = ["derive"] }`
- [x] **1.9** Create `crates/chronos-daemon/src/main.rs` with a `fn main() { println!("chronos v0.1"); }`
- [x] **1.10** Create `.gitignore` (standard Rust: `/target`, `*.swp`, etc.)
- [x] **1.11** Run: `cargo check --workspace`
- [x] **1.12** Run: `cargo fmt --all -- --check`
- [x] **1.13** Run: `cargo clippy --workspace --all-targets -- -D warnings`

**Acceptance Criteria:** ✅ All met
- ✅ `cargo check --workspace` → success (exit 0)
- ✅ `cargo clippy --workspace --all-targets -- -D warnings` → clean
- ✅ `cargo fmt --all -- --check` → no formatting issues
- ✅ All 4 crates visible in workspace members

> **Go parallel:** This is equivalent to `go mod init` + creating empty `package` files so `go build ./...` works. The `[workspace]` in Cargo.toml is like a Go workspace (`go.work`).

**✅ Step 1 complete — Proceeding to Step 2.**

---

### Step 2: Core Domain Models ✅

> 📋 **Detailed tasks:** [`tasks-step-2-core-domain-models/`](tasks-step-2-core-domain-models/)

**Goal:** Define all shared data types: `Frame`, `SemanticLog`, `CaptureConfig`, `VlmConfig`, and the domain error enum. Validate with serialization round-trip tests.

**Depends on:** Step 1 complete

**Crate(s):** `chronos-core`

**Tasks:**

- [x] **2.1** Create `crates/chronos-core/src/error.rs`:
  - Define `ChronosError` enum using `thiserror::Error`:
    - `Capture(String)` — screen capture failures
    - `Inference(String)` — VLM communication errors
    - `Database(String)` — SQL/storage errors
    - `Config(String)` — configuration issues
    - `Timeout(String)` — operation timeout
  - Implement `From<sqlx::Error>`, `From<reqwest::Error>` conversions
  - Define type alias: `pub type Result<T> = std::result::Result<T, ChronosError>;`
  - Tests: verify error display strings, verify `From` conversions

  > **Go parallel:** In Go you'd define `var ErrCapture = errors.New("capture")` and wrap with `fmt.Errorf(...)`. Rust's `thiserror` auto-generates `Display` and `Error` — like Go's `errors.New()` but with exhaustive pattern matching via `match`.

- [x] **2.2** Create `crates/chronos-core/src/models.rs`:
  - `Frame` struct (see Design §3.B):
    ```rust
    pub struct Frame {
        pub id: Ulid,
        pub timestamp: DateTime<Utc>,
        pub image_data: Vec<u8>,  // Raw PNG bytes (in RAM only!)
        pub width: u32,
        pub height: u32,
    }
    ```
    - Derive: `Debug`, `Clone`
    - **No `Serialize`** — frames never leave RAM, never touch disk
  - `SemanticLog` struct (see Design §3.D, Table schema):
    ```rust
    pub struct SemanticLog {
        pub id: Ulid,
        pub timestamp: DateTime<Utc>,
        pub source_frame_id: Ulid,
        pub description: String,
        pub active_application: Option<String>,
        pub activity_category: Option<String>,
        pub key_entities: Vec<String>,
        pub confidence_score: f64,
        pub raw_vlm_response: String,
    }
    ```
    - Derive: `Debug`, `Clone`, `Serialize`, `Deserialize`
    - `key_entities` stored as JSON array string in SQLite
  - `CaptureConfig` struct:
    ```rust
    pub struct CaptureConfig {
        pub interval_seconds: u64,     // Default: 30
        pub ring_buffer_capacity: usize, // Default: 64
    }
    ```
    - Derive: `Debug`, `Clone`, `Serialize`, `Deserialize`
    - `Default` implementation with v0.1 defaults
  - `VlmConfig` struct:
    ```rust
    pub struct VlmConfig {
        pub ollama_host: String,       // Default: "http://localhost:11434"
        pub model_name: String,        // Default: "moondream"
        pub timeout_seconds: u64,      // Default: 60
    }
    ```
    - Derive: `Debug`, `Clone`, `Serialize`, `Deserialize`
    - `Default` implementation
  - Tests (`#[cfg(test)]` in same file):
    - Frame creation with known bytes
    - SemanticLog serialization round-trip (`serde_json::to_string` → `from_str`)
    - CaptureConfig default values assertion
    - VlmConfig default values assertion

- [x] **2.3** Update `crates/chronos-core/src/lib.rs` to declare and re-export modules:
  ```rust
  pub mod error;
  pub mod models;
  ```

- [x] **2.4** Run: `cargo test -p chronos-core`
- [x] **2.5** Run: `cargo clippy -p chronos-core -- -D warnings`

**Acceptance Criteria:**
- All model structs compile correctly
- Serialization/deserialization round-trip tests pass
- Default configs return documented values
- `cargo test -p chronos-core` → all green

**✅ Step 2 complete — Proceeding to Step 3.**

---

### Step 3: Trait Boundaries & Mocks

> 📋 **Detailed tasks:** [`tasks-step-3-trait-boundaries-mocks/`](tasks-step-3-trait-boundaries-mocks/)

**Goal:** Define the `ImageCapture` and `VisionInference` trait abstractions with full mock implementations. This is the decoupling layer that makes the entire system testable without hardware. (See Design §3.A, §6)

**Depends on:** Step 2 complete

**Crate(s):** `chronos-core`

**Tasks:**

- [x] **3.1** Create `crates/chronos-core/src/traits.rs`:
  - `ImageCapture` trait:
    ```rust
    /// The screen capture abstraction.
    /// In Go terms: this is an interface. Any struct that implements
    /// these methods satisfies the contract.
    #[async_trait]
    pub trait ImageCapture: Send + Sync {
        /// Capture a single frame from the screen.
        /// Returns raw image bytes wrapped in a Frame.
        async fn capture_frame(&self) -> Result<Frame>;
    }
    ```
  - `VisionInference` trait:
    ```rust
    /// The vision-language model abstraction.
    /// Makes the core pipeline agnostic to whether it talks to
    /// Ollama, a mock, or any future VLM backend.
    #[async_trait]
    pub trait VisionInference: Send + Sync {
        /// Send a frame's image data to a VLM and get a semantic description.
        async fn analyze_frame(&self, frame: &Frame) -> Result<SemanticLog>;
    }
    ```
  - Add `async-trait = "0.1"` dependency to `chronos-core/Cargo.toml`
  - **Note:** `Send + Sync` bounds are required because these traits cross async task boundaries. In Go, interfaces are implicitly goroutine-safe; in Rust, we must declare it.

- [x] **3.2** Add mock implementations in the same file (behind `#[cfg(test)]` or public for integration tests):
  - `MockCapture`:
    ```rust
    /// A test double that returns a static 1x1 pixel PNG.
    /// No real screen capture — works in CI, headless, everywhere.
    pub struct MockCapture;
    
    #[async_trait]
    impl ImageCapture for MockCapture {
        async fn capture_frame(&self) -> Result<Frame> {
            Ok(Frame {
                id: Ulid::new(),
                timestamp: Utc::now(),
                image_data: vec![0x89, 0x50, 0x4E, 0x47], // PNG magic bytes
                width: 1,
                height: 1,
            })
        }
    }
    ```
  - `MockVision`:
    ```rust
    /// A test double that returns a hardcoded semantic log.
    /// Simulates a VLM that always sees "User editing code in VSCode".
    pub struct MockVision;
    
    #[async_trait]
    impl VisionInference for MockVision {
        async fn analyze_frame(&self, frame: &Frame) -> Result<SemanticLog> {
            Ok(SemanticLog {
                id: Ulid::new(),
                timestamp: frame.timestamp,
                source_frame_id: frame.id,
                description: "User editing code in VSCode".to_string(),
                active_application: Some("Visual Studio Code".to_string()),
                activity_category: Some("Development".to_string()),
                key_entities: vec!["Rust".to_string(), "main.rs".to_string()],
                confidence_score: 0.95,
                raw_vlm_response: r#"{"description":"User editing code"}"#.to_string(),
            })
        }
    }
    ```

- [x] **3.3** Write tests (`#[cfg(test)]` in `traits.rs`):
  - `test_mock_capture_returns_frame` — verify MockCapture produces a valid Frame with PNG magic bytes
  - `test_mock_vision_returns_semantic_log` — verify MockVision produces a valid SemanticLog with expected fields
  - `test_mock_vision_preserves_frame_id` — verify `source_frame_id` matches the input frame's `id`
  - `test_mock_capture_unique_ids` — verify two captures produce different ULIDs
  - `test_trait_object_dispatch` — create a `Box<dyn ImageCapture>` from MockCapture, call it, verify it works (proves dynamic dispatch is viable)

  > **Go parallel:** These tests are equivalent to testing a Go interface with a stub implementation. The `Box<dyn Trait>` test proves Rust's dynamic dispatch works like Go's implicit interface satisfaction.

- [x] **3.4** Update `crates/chronos-core/src/lib.rs` to export `traits` module
- [x] **3.5** Run: `cargo test -p chronos-core`
- [x] **3.6** Run: `cargo clippy -p chronos-core -- -D warnings`

**Acceptance Criteria:**
- Both traits compile with `async_trait`
- All mock tests pass
- `Box<dyn ImageCapture>` and `Box<dyn VisionInference>` compile (dynamic dispatch works)
- No `unwrap()` in any non-test code

**✋ Pause Point — Wait for user review before proceeding to Step 4.**

---

### Step 4: Database Layer

**Goal:** Set up SQLite persistence via `sqlx`. Create the `semantic_logs` table schema, implement insert/query operations, and verify with in-memory SQLite tests. Follow the `/Local SQLite & SQLx Pipeline` workflow.

**Depends on:** Step 2 complete (models)

**Crate(s):** `chronos-daemon` (database module lives here as it's the only binary that touches storage)

**Tasks:**

- [ ] **4.1** Create `migrations/001_create_semantic_logs.sql`:
  ```sql
  -- UP: Create the semantic_logs table (see Design §3.D)
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

  -- Index for time-range queries (the most common query pattern)
  CREATE INDEX IF NOT EXISTS idx_semantic_logs_timestamp ON semantic_logs(timestamp);

  -- Index for filtering by application
  CREATE INDEX IF NOT EXISTS idx_semantic_logs_app ON semantic_logs(active_application);
  ```

  > **Go parallel:** In Go, you'd use `golang-migrate` or `goose` for SQL migrations. Rust's `sqlx` has a built-in `sqlx::migrate!()` macro that embeds migrations at compile time — like `go:embed` for SQL files.

- [ ] **4.2** Create `crates/chronos-daemon/src/database.rs`:
  - `Database` struct holding a `SqlitePool`:
    ```rust
    pub struct Database {
        pool: SqlitePool,
    }
    ```
  - `Database::new(database_url: &str) -> Result<Self>` — connects and runs migrations
  - `Database::new_in_memory() -> Result<Self>` — for testing (`:memory:`)
  - `Database::insert_semantic_log(&self, log: &SemanticLog) -> Result<()>` — INSERT with `sqlx::query`
  - `Database::get_logs_by_date_range(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<SemanticLog>>` — SELECT with timestamp range
  - `Database::get_log_count(&self) -> Result<i64>` — COUNT for status display
  - `Database::get_recent_logs(&self, limit: i64) -> Result<Vec<SemanticLog>>` — SELECT ORDER BY timestamp DESC LIMIT N

  > **Go parallel:** This is your `database/sql` + `sqlc` pattern. But `sqlx` validates SQL at compile time via `query!` macros — like `sqlc generate` but integrated into the build. For runtime queries we'll use `sqlx::query` (non-macro) with `.bind()` to avoid needing a live database at compile time.

- [ ] **4.3** Write tests (`#[cfg(test)]` in `database.rs`):
  - `test_insert_and_query_round_trip` — insert a SemanticLog, query it back, verify all fields match
  - `test_get_logs_by_date_range` — insert logs at different timestamps, query a range, verify filtering
  - `test_get_log_count` — insert N logs, verify count returns N
  - `test_get_recent_logs_respects_limit` — insert 10 logs, query with limit 3, verify 3 returned in descending order
  - `test_empty_database_returns_zero_count` — verify empty DB returns count 0
  - All tests use `Database::new_in_memory()` — no real file I/O

- [ ] **4.4** Run: `cargo test -p chronos-daemon`
- [ ] **4.5** Run: `cargo clippy -p chronos-daemon -- -D warnings`

**Acceptance Criteria:**
- Migration creates the table successfully in-memory
- All CRUD operations pass round-trip tests
- `key_entities` correctly serialized as JSON string ↔ `Vec<String>`
- No `.unwrap()` in non-test code
- `cargo test -p chronos-daemon` → all green

**✋ Pause Point — Wait for user review before proceeding to Step 5.**

---

### Step 5: Screen Capture (X11)

**Goal:** Implement the `X11Capture` struct that captures the primary monitor via `xcap` on a dedicated OS thread, stores frames in a ring buffer, and bridges to the async world via `tokio::sync::mpsc`. Follow the `/OS & Async Boundary Control` workflow.

**Depends on:** Step 3 complete (traits)

**Crate(s):** `chronos-capture`

**Tasks:**

- [ ] **5.1** Create `crates/chronos-capture/src/ring_buffer.rs`:
  - `FrameRingBuffer` struct wrapping `VecDeque<Frame>` with a max capacity:
    ```rust
    pub struct FrameRingBuffer {
        buffer: VecDeque<Frame>,
        capacity: usize,
    }
    ```
  - Methods:
    - `new(capacity: usize) -> Self`
    - `push(&mut self, frame: Frame)` — pushes frame; if buffer is full, drops the oldest (see Design §3.B, "back-pressure")
    - `len(&self) -> usize`
    - `is_empty(&self) -> bool`
    - `latest(&self) -> Option<&Frame>` — peek at most recent
  - Tests:
    - `test_push_within_capacity` — push N < capacity, verify len = N
    - `test_push_drops_oldest_when_full` — push capacity+1, verify len = capacity and oldest frame is gone
    - `test_latest_returns_most_recent` — push 3, verify latest matches last pushed
    - `test_empty_buffer` — verify `is_empty()` on new buffer, `latest()` returns `None`

  > **Go parallel:** This is a fixed-size channel with buffer, like `make(chan Frame, 64)`, but explicit about what happens on overflow. Go's buffered channel blocks; our ring buffer drops the oldest — a design decision for back-pressure (Design §3.B).

- [ ] **5.2** Create `crates/chronos-capture/src/x11.rs`:
  - `X11Capture` struct implementing `ImageCapture`:
    ```rust
    pub struct X11Capture {
        config: CaptureConfig,
    }
    ```
  - `X11Capture::new(config: CaptureConfig) -> Self`
  - `impl ImageCapture for X11Capture`:
    - `capture_frame()` — use `xcap::Monitor::all()` to get primary monitor, call `capture_image()`, convert to PNG bytes, wrap in `Frame`
    - Wrap the blocking `xcap` call in `tokio::task::spawn_blocking` since `xcap` uses OS-level X11 calls (see Design §3.C for thread isolation)
  - `X11Capture::start_capture_loop()` — spawns a dedicated `std::thread` that:
    1. Captures a frame every `config.interval_seconds`
    2. Sends the frame through a `tokio::sync::mpsc::Sender<Frame>`
    3. Respects a shutdown signal via `tokio::sync::watch` or a shared `AtomicBool`
  - Error mapping: X11 errors → `ChronosError::Capture`

  > **Go parallel:** This is equivalent to starting a goroutine with `go captureLoop(ctx, ch)`. The key difference: Rust requires `std::thread` (not just async) because `xcap`'s X11 calls are blocking and would starve the Tokio executor. In Go, the runtime multiplexes goroutines onto OS threads automatically — Rust gives you explicit control.

- [ ] **5.3** Write tests (`#[cfg(test)]` in `x11.rs`):
  - **Unit tests (no real X11 needed):**
    - `test_x11_capture_creation` — verify struct construction with default config
    - `test_capture_config_defaults` — verify interval and buffer size defaults
  - **Note:** Real X11 capture tests require a display server. We rely on `MockCapture` from Step 3 for pipeline testing. An optional integration test gated behind `#[cfg(feature = "x11-integration")]` can live here for manual verification on a real machine.

- [ ] **5.4** Update `crates/chronos-capture/src/lib.rs`:
  ```rust
  pub mod ring_buffer;
  pub mod x11;
  ```

- [ ] **5.5** Run: `cargo test -p chronos-capture`
- [ ] **5.6** Run: `cargo clippy -p chronos-capture -- -D warnings`

**Acceptance Criteria:**
- Ring buffer tests pass (push, overflow, latest)
- X11Capture compiles and implements `ImageCapture` trait
- Capture loop uses `std::thread` (not Tokio task) for blocking X11 calls
- Frame data flows through `mpsc::Sender` channel
- No `.unwrap()` in non-test code

**✋ Pause Point — Wait for user review before proceeding to Step 6.**

---

### Step 6: Ollama Vision Client

**Goal:** Implement the `OllamaVision` struct that sends base64-encoded frames to a local Ollama instance and parses the VLM's JSON response into a `SemanticLog`. (See Design §3.C)

**Depends on:** Step 3 complete (traits)

**Crate(s):** `chronos-inference`

**Tasks:**

- [ ] **6.1** Create `crates/chronos-inference/src/ollama.rs`:
  - `OllamaVision` struct:
    ```rust
    pub struct OllamaVision {
        client: reqwest::Client,
        config: VlmConfig,
    }
    ```
  - `OllamaVision::new(config: VlmConfig) -> Self` — creates `reqwest::Client` with configured timeout
  - `impl VisionInference for OllamaVision`:
    - `analyze_frame(frame)` — (see Design §5.A for prompt + format):
      1. Base64-encode `frame.image_data`
      2. Build JSON request body for `/api/generate`:
         ```json
         {
           "model": "moondream",
           "prompt": "Analyze this screenshot...",
           "images": ["<base64>"],
           "stream": false,
           "format": "json"
         }
         ```
      3. POST to `{config.ollama_host}/api/generate`
      4. Parse the response JSON to extract `"response"` field
      5. Parse the VLM's response text as structured JSON:
         ```json
         {
           "description": "...",
           "active_application": "...",
           "activity_category": "...",
           "key_entities": ["..."],
           "confidence_score": 0.85
         }
         ```
      6. Map fields into `SemanticLog`
      7. **Fallback:** If the VLM response is not valid JSON, store the raw text as `description` with low confidence (Design §5.A — "regex fallback parser")
  - Error handling:
    - HTTP errors → `ChronosError::Inference`
    - Timeout → `ChronosError::Timeout`
    - Malformed JSON → fallback to raw text, log warning

  > **Go parallel:** This is equivalent to `http.Post()` with JSON encoding via `encoding/json`. In Rust, `reqwest` is the standard HTTP client (like Go's `net/http`), and `serde_json` replaces `json.Marshal/Unmarshal`. The key difference: Rust's `?` operator replaces Go's `if err != nil { return err }` pattern.

- [ ] **6.2** Add internal helper: `parse_vlm_response(raw: &str) -> (SemanticLog fields)`:
  - Primary path: `serde_json::from_str` the VLM's JSON
  - Fallback path: if JSON parsing fails, use the raw text as `description`, set `confidence_score = 0.3`, leave optional fields as `None`

- [ ] **6.3** Write tests (`#[cfg(test)]` in `ollama.rs`):
  - `test_parse_valid_vlm_json` — feed known good JSON, verify all fields map correctly
  - `test_parse_malformed_vlm_json_fallback` — feed garbled text, verify fallback to raw description with low confidence
  - `test_parse_partial_vlm_json` — feed JSON missing optional fields, verify `None` handling
  - `test_ollama_vision_creation` — verify struct construction with default config
  - (Optional, if `wiremock` or `mockito` is added):
    - `test_ollama_http_success` — mock HTTP server returns valid Ollama response, verify SemanticLog
    - `test_ollama_http_timeout` — mock server delays, verify `ChronosError::Timeout`
    - `test_ollama_http_500` — mock server returns 500, verify `ChronosError::Inference`

  > For v0.1, JSON parsing tests are sufficient. Full HTTP mocking with `wiremock` can be added as a follow-up or gated behind a `test-http` feature.

- [ ] **6.4** Update `crates/chronos-inference/src/lib.rs`:
  ```rust
  pub mod ollama;
  ```

- [ ] **6.5** Run: `cargo test -p chronos-inference`
- [ ] **6.6** Run: `cargo clippy -p chronos-inference -- -D warnings`

**Acceptance Criteria:**
- JSON parsing (valid, malformed, partial) tests all pass
- `OllamaVision` implements `VisionInference` trait
- Fallback path handles non-JSON VLM responses gracefully
- Timeout configured via `VlmConfig.timeout_seconds`
- No `.unwrap()` in non-test code
- No outbound HTTP to anything other than `localhost` (privacy constraint)

**✋ Pause Point — Wait for user review before proceeding to Step 7.**

---

### Step 7: Pipeline Integration (Daemon)

**Goal:** Wire the full pipeline: Capture → Vision → Database. Implement the main async loop in the daemon crate that receives frames from a channel, sends them to the VLM, and stores results in SQLite. (See Design §3.A, §5.B)

**Depends on:** Steps 4, 5, and 6 complete

**Crate(s):** `chronos-daemon`

**Tasks:**

- [ ] **7.1** Create `crates/chronos-daemon/src/pipeline.rs`:
  - `CaptureEngine` struct (see Design §3.A):
    ```rust
    /// The core orchestrator. Receives frames, analyzes them, stores results.
    /// Generic over its dependencies — accepts any ImageCapture and VisionInference.
    /// 
    /// Go parallel: This is like a struct that takes interfaces in Go:
    /// type CaptureEngine struct {
    ///     vision VisionInference
    ///     db     *Database
    /// }
    pub struct CaptureEngine<V: VisionInference> {
        vision: V,
        database: Database,
    }
    ```
  - `CaptureEngine::new(vision: V, database: Database) -> Self`
  - `CaptureEngine::process_frame(&self, frame: Frame) -> Result<SemanticLog>`:
    1. Call `self.vision.analyze_frame(&frame)` to get semantic description
    2. Call `self.database.insert_semantic_log(&log)` to persist
    3. Return the stored log
  - `CaptureEngine::run_pipeline(&self, rx: mpsc::Receiver<Frame>) -> Result<()>`:
    1. Loop on `rx.recv().await`
    2. For each frame, call `process_frame()`
    3. Log errors but don't crash (Design §5.B — "backoff on errors")
    4. Respect a shutdown signal (e.g., `tokio::signal::ctrl_c()`)

  > **Go parallel:** This is your `for frame := range ch { ... }` loop inside a goroutine. The `mpsc::Receiver` is Rust's equivalent of `<-chan Frame`. The generic `<V: VisionInference>` is like Go's interface constraint on a struct field.

- [ ] **7.2** Write tests (`#[cfg(test)]` in `pipeline.rs`):
  - `test_process_frame_with_mocks` — use `MockVision` + in-memory DB, process one frame, verify log stored and returned
  - `test_pipeline_processes_multiple_frames` — send 5 frames through channel with MockCapture → MockVision → in-memory DB, verify 5 logs stored
  - `test_pipeline_handles_vision_error_gracefully` — create a `FailingMockVision` that returns `Err`, verify pipeline doesn't crash, continues processing subsequent frames
  - `test_process_frame_stores_correct_source_frame_id` — verify the stored log's `source_frame_id` matches the input frame's `id`

- [ ] **7.3** Run: `cargo test -p chronos-daemon`
- [ ] **7.4** Run: `cargo clippy -p chronos-daemon -- -D warnings`

**Acceptance Criteria:**
- End-to-end mock pipeline test works (MockCapture → MockVision → in-memory SQLite)
- Pipeline gracefully handles VLM errors without crashing
- Frame IDs flow correctly through the pipeline
- `cargo test -p chronos-daemon` → all green

**✋ Pause Point — Wait for user review before proceeding to Step 8.**

---

### Step 8: CLI

**Goal:** Implement the CLI interface using `clap`: `chronos query`, `chronos status`, `chronos pause`, `chronos resume`. The CLI reads from SQLite and reports system state. (See Design §3.F — "chronos-daemon" crate responsibilities)

**Depends on:** Step 4 complete (database), Step 7 for pipeline integration

**Crate(s):** `chronos-daemon`

**Tasks:**

- [ ] **8.1** Create `crates/chronos-daemon/src/cli.rs`:
  - Define `Cli` struct with `#[derive(Parser)]`:
    ```rust
    #[derive(Parser)]
    #[command(name = "chronos", about = "Your personal context engine")]
    pub struct Cli {
        #[command(subcommand)]
        pub command: Commands,
    }
    
    #[derive(Subcommand)]
    pub enum Commands {
        /// Start the capture daemon
        Start,
        
        /// Query semantic logs
        Query {
            /// Filter logs from this date (YYYY-MM-DD)
            #[arg(long)]
            from: Option<String>,
            
            /// Filter logs to this date (YYYY-MM-DD)
            #[arg(long)]
            to: Option<String>,
            
            /// Maximum number of results
            #[arg(long, default_value = "10")]
            limit: i64,
        },
        
        /// Show system status
        Status,
        
        /// Pause screen capture
        Pause,
        
        /// Resume screen capture
        Resume,
    }
    ```

  > **Go parallel:** In Go you'd use `cobra` or `flag`. Rust's `clap` with `derive` is the equivalent — but it generates help text and validates arguments at compile time.

- [ ] **8.2** Implement command handlers in `main.rs`:
  - `handle_start()` — initialize Database, X11Capture, OllamaVision, wire the pipeline (from Step 7), run the async loop
  - `handle_query(from, to, limit)` — connect to SQLite, query logs by date range, print results formatted to stdout
  - `handle_status()` — connect to SQLite, print log count + capture state (for v0.1: basic stats from DB)
  - `handle_pause()` / `handle_resume()` — for v0.1, these can write/delete a sentinel file or print a "not yet implemented" message. Full IPC comes in v0.2.

- [ ] **8.3** Update `crates/chronos-daemon/src/main.rs`:
  ```rust
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
  ```
  - Add `anyhow = "1"` to `chronos-daemon/Cargo.toml` for top-level error handling

- [ ] **8.4** Write tests (`#[cfg(test)]` in `cli.rs`):
  - `test_cli_parse_start` — verify `chronos start` parses to `Commands::Start`
  - `test_cli_parse_query_with_dates` — verify `chronos query --from 2025-01-01 --to 2025-01-31` parses correctly
  - `test_cli_parse_query_defaults` — verify `chronos query` with no args uses default limit
  - `test_cli_parse_status` — verify `chronos status` parses correctly

- [ ] **8.5** Run: `cargo test -p chronos-daemon`
- [ ] **8.6** Run: `cargo clippy -p chronos-daemon -- -D warnings`
- [ ] **8.7** Manual verification: `cargo run -p chronos-daemon -- --help`

**Acceptance Criteria:**
- All CLI subcommands parse correctly
- `chronos query` reads from SQLite and formats output
- `chronos status` shows log count
- `--help` produces clean usage documentation
- No hardcoded file paths (use XDG data directory or configurable path)

**✋ Pause Point — Wait for user review before proceeding to Step 9.**

---

### Step 9: Integration & Smoke Test

**Goal:** Full workspace verification. All tests green, all lints clean, all formatting correct. Run the `/Verify Cargo Workspace` workflow. Write a basic README.

**Depends on:** All prior phases complete

**Crate(s):** All

**Tasks:**

- [ ] **9.1** Run the full verification suite (per `/Verify Cargo Workspace`):
  ```bash
  cargo fmt --all -- --check
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace
  ```
  - Fix any issues that arise

- [ ] **9.2** Create root `README.md`:
  - Project name, one-line description
  - Privacy statement ("100% local, never sends data externally")
  - Prerequisites: Rust 1.75+, Ollama installed with Moondream model
  - Build instructions: `cargo build --workspace`
  - Quick start: `cargo run -p chronos-daemon -- start`
  - Query example: `cargo run -p chronos-daemon -- query --from 2025-01-01`
  - Status: v0.1 MVP — Linux X11 only
  - Link to design document

- [ ] **9.3** Write an end-to-end integration test (in `chronos-daemon`):
  - `test_full_pipeline_mock_end_to_end`:
    1. Create `MockCapture` and `MockVision` from `chronos-core`
    2. Create in-memory `Database`
    3. Create `CaptureEngine` with mocks
    4. Send 3 frames through the pipeline
    5. Verify 3 logs stored in database
    6. Query by date range, verify results
    7. Check log count = 3

- [ ] **9.4** Final verification:
  ```bash
  cargo fmt --all -- --check        # ✅
  cargo clippy --workspace --all-targets -- -D warnings  # ✅
  cargo test --workspace            # ✅
  ```

- [ ] **9.5** (Optional) Manual smoke test on a real X11 machine:
  1. Start Ollama: `ollama serve`
  2. Ensure model is pulled: `ollama pull moondream`
  3. Run: `cargo run -p chronos-daemon -- start`
  4. Wait 30+ seconds, then Ctrl+C
  5. Run: `cargo run -p chronos-daemon -- status`
  6. Run: `cargo run -p chronos-daemon -- query`
  7. Verify output shows captured semantic logs

**Acceptance Criteria:**
- `cargo test --workspace` → all green
- `cargo clippy --workspace --all-targets -- -D warnings` → clean
- `cargo fmt --all -- --check` → no formatting issues
- README exists with setup instructions
- End-to-end integration test with mocks passes

**✋ Final Pause — MVP v0.1 is complete. Ready for user review.**

---

## 3. Verification Matrix

| Component | Unit Tests | Integration Tests | Mock Used | Verification Command |
|---|---|---|---|---|
| `error.rs` (Core) | Display strings, From conversions | — | — | `cargo test -p chronos-core` |
| `models.rs` (Core) | Serde round-trip, defaults | — | — | `cargo test -p chronos-core` |
| `traits.rs` (Core) | Mock capture/vision, trait objects | — | `MockCapture`, `MockVision` | `cargo test -p chronos-core` |
| `ring_buffer.rs` (Capture) | Push, overflow, latest | — | — | `cargo test -p chronos-capture` |
| `x11.rs` (Capture) | Struct creation | Manual (real X11) | `MockCapture` (in pipeline tests) | `cargo test -p chronos-capture` |
| `ollama.rs` (Inference) | JSON parsing (valid/invalid/partial) | HTTP mock (optional) | `MockVision` (in pipeline tests) | `cargo test -p chronos-inference` |
| `database.rs` (Daemon) | Insert, query, count, range, limit | In-memory SQLite | — | `cargo test -p chronos-daemon` |
| `pipeline.rs` (Daemon) | Single frame, multi frame, error handling | Full mock pipeline | `MockCapture`, `MockVision`, in-memory DB | `cargo test -p chronos-daemon` |
| `cli.rs` (Daemon) | Argument parsing for all subcommands | Manual (run `--help`) | — | `cargo test -p chronos-daemon` |
| **Full Workspace** | — | End-to-end mock pipeline | All mocks | `cargo test --workspace` |
| **Lint** | — | — | — | `cargo clippy --workspace --all-targets -- -D warnings` |
| **Format** | — | — | — | `cargo fmt --all -- --check` |

---

## 4. Definition of Done (v0.1)

The MVP is complete when **all** of the following are true:

- [ ] `cargo test --workspace` → **all tests green** (zero failures)
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` → **zero warnings**
- [ ] `cargo fmt --all -- --check` → **no formatting issues**
- [ ] All traits (`ImageCapture`, `VisionInference`) have mock implementations
- [ ] All modules have `#[cfg(test)]` blocks with meaningful tests
- [ ] No `.unwrap()` in non-test production code
- [ ] No outbound HTTP to anything other than `localhost` (privacy guarantee)
- [ ] No raw image files (`.png`, `.jpg`) written to disk (frames stay in RAM ring buffer)
- [ ] `chronos status` works against real or mock data
- [ ] `chronos query` works against real or mock data
- [ ] `README.md` exists with build + usage instructions
- [ ] End-to-end integration test passes with all mocks
- [ ] All code comments explain *why*, not *what*, with Go→Rust parallels where relevant

---

> *"Build small. Test everything. Trust the borrow checker. Pause for humans."*
