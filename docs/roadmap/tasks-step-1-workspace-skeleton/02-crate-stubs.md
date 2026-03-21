# Crate Stubs

> **Step:** 1 | **Tasks:** 1.2 – 1.9 | **Crate(s):** all 4 members

## Why This Matters

Each crate in the workspace serves a distinct architectural boundary (see rule `07-workspace-routing.md`). In this task group we create the minimal files needed for Cargo to recognise each crate — a `Cargo.toml` with its dependencies and a stub `src/lib.rs` or `src/main.rs`.

The dependencies declared here are the ones we **know Step 2+ will need**. Adding them now ensures `cargo check` validates that all external crates resolve correctly, and that cross-crate paths (`chronos-core = { path = "..." }`) work.

**Go parallel:** This is like creating `go.mod` + an empty `main.go` (or `package.go`) for each module. In Go, `go build ./...` would verify imports resolve. In Rust, `cargo check --workspace` does the same.

## Tasks

### Task 1.2 — `chronos-core/Cargo.toml`

**What:** Create the manifest for the foundational shared types crate.

**File:** `crates/chronos-core/Cargo.toml`

```toml
[package]
name = "chronos-core"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
thiserror = "2"
ulid = { version = "1.1", features = ["serde"] }
```

**Why these dependencies:**

| Crate | Purpose | Go equivalent |
|---|---|---|
| `serde` + `serde_json` | Serialization framework — like `encoding/json` but compile-time derived, zero-cost | `encoding/json` + struct tags |
| `chrono` | Date/time — like `time.Time` but with timezone-aware types and `serde` integration | `time` package |
| `thiserror` | Derive macro for `std::error::Error` — eliminates boilerplate | `errors.New()` + `fmt.Errorf()` |
| `ulid` | ULID generation — like `oklog/ulid` | `github.com/oklog/ulid` |

**Gotchas:**
- `serde` needs `features = ["derive"]` to enable `#[derive(Serialize, Deserialize)]`. Without it, you get a confusing "cannot find derive macro" error.
- `thiserror = "2"` — v2 is the current major. If you see v1 examples online, the API is identical but v2 has better diagnostics.

---

### Task 1.3 — `chronos-core/src/lib.rs`

**What:** Create a minimal lib.rs that compiles.

**File:** `crates/chronos-core/src/lib.rs`

```rust
// chronos-core: Shared domain models, error types, and trait abstractions.
// Modules will be added in Step 2.
```

Just an empty file with a comment. Modules (`mod error;`, `mod models;`, etc.) will be added in Step 2.

---

### Task 1.4 — `chronos-capture/Cargo.toml`

**What:** Manifest for the screen capture crate.

**File:** `crates/chronos-capture/Cargo.toml`

```toml
[package]
name = "chronos-capture"
version = "0.1.0"
edition = "2021"

[dependencies]
chronos-core = { path = "../chronos-core" }
xcap = "0.0.14"
tokio = { version = "1", features = ["sync"] }
```

**Why these dependencies:**

| Crate | Purpose | Notes |
|---|---|---|
| `chronos-core` | Access to `Frame`, error types, `ImageCapture` trait | Path dependency — changes propagate instantly |
| `xcap` | Cross-platform screen capture (X11/Wayland/macOS/Windows) | Check [crates.io](https://crates.io/crates/xcap) for latest version |
| `tokio` with `sync` only | Just `mpsc` channels for the ring buffer — NOT the full runtime | Keeping it minimal; `chronos-daemon` owns the runtime |

**Gotchas:**
- `tokio` features: only `"sync"` here. Do NOT add `"full"` — this crate should not spawn its own async runtime. The daemon will drive the runtime. This is a deliberate boundary (see rule `06-clean-boundaries.md`).
- `xcap` version: the roadmap says `0.0.13`, but check for the latest on crates.io. The API may have minor changes between patch versions since it's pre-1.0.

---

### Task 1.5 — `chronos-capture/src/lib.rs`

**What:** Empty lib stub.

**File:** `crates/chronos-capture/src/lib.rs`

```rust
// chronos-capture: OS-level screen capture with ring buffer.
// Implementation comes in Step 4.
```

---

### Task 1.6 — `chronos-inference/Cargo.toml`

**What:** Manifest for the VLM inference crate.

**File:** `crates/chronos-inference/Cargo.toml`

```toml
[package]
name = "chronos-inference"
version = "0.1.0"
edition = "2021"

[dependencies]
chronos-core = { path = "../chronos-core" }
reqwest = { version = "0.12", features = ["json"] }
serde_json = "1"
base64 = "0.22"
tokio = { version = "1", features = ["full"] }
```

**Why these dependencies:**

| Crate | Purpose | Go equivalent |
|---|---|---|
| `reqwest` | HTTP client for Ollama REST API — like `net/http` but async and higher-level | `net/http` + `http.Client` |
| `base64` | Encode frame bytes to base64 for Ollama's `/api/generate` | `encoding/base64` |
| `tokio` `full` | Needs the full runtime for async HTTP calls with timeouts | `context.WithTimeout()` |

**Gotchas:**
- `reqwest` needs `features = ["json"]` for `.json()` request/response methods. Without it, you'd have to manually serialize.
- `tokio` `full` is appropriate here because this crate makes async HTTP calls. This is different from `chronos-capture` which only needs channels.

---

### Task 1.7 — `chronos-inference/src/lib.rs`

**What:** Empty lib stub.

**File:** `crates/chronos-inference/src/lib.rs`

```rust
// chronos-inference: Local VLM inference via Ollama HTTP API.
// Implementation comes in Step 5.
```

---

### Task 1.8 — `chronos-daemon/Cargo.toml`

**What:** Manifest for the main binary that orchestrates everything.

**File:** `crates/chronos-daemon/Cargo.toml`

```toml
[package]
name = "chronos-daemon"
version = "0.1.0"
edition = "2021"

[dependencies]
chronos-core = { path = "../chronos-core" }
chronos-capture = { path = "../chronos-capture" }
chronos-inference = { path = "../chronos-inference" }
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
clap = { version = "4", features = ["derive"] }
```

**Why these dependencies:**

| Crate | Purpose | Go equivalent |
|---|---|---|
| All `chronos-*` | Wires together all internal crates | Internal packages in Go |
| `tokio` `full` | Owns the async runtime; runs capture loop, inference, and DB writes concurrently | `goroutines` + `select` |
| `sqlx` | Async SQLite access — compile-time checked SQL queries | `database/sql` + `mattn/go-sqlite3` |
| `clap` | CLI argument parsing — like `cobra` or `flag` | `spf13/cobra` or `flag` |

**Gotchas:**
- `sqlx` requires `runtime-tokio` to match our async runtime. Using `runtime-async-std` would cause a panic at startup.
- `sqlx` `sqlite` feature pulls in `libsqlite3-sys` which compiles SQLite from source. First build will be slow (~30s). Subsequent builds use cache.
- This is the ONLY crate that should depend on `sqlx` (see rule `07-workspace-routing.md`).

---

### Task 1.9 — `chronos-daemon/src/main.rs`

**What:** Minimal binary entry point.

**File:** `crates/chronos-daemon/src/main.rs`

```rust
fn main() {
    println!("chronos v0.1");
}
```

This is intentionally trivial. The real `main` (with `#[tokio::main]`, `clap`, database init) comes in Step 7. For now we just need `cargo run -p chronos-daemon` to work.

## Pre-Research

Before starting, ensure you understand:
- [ ] [Cargo `path` dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-path-dependencies) — how crates reference siblings
- [ ] [Tokio feature flags](https://docs.rs/tokio/latest/tokio/#feature-flags) — why `sync` vs `full` matters
- [ ] [xcap crate](https://crates.io/crates/xcap) — check the latest version and changelog
- [ ] [sqlx feature matrix](https://github.com/launchbadge/sqlx#cargo-feature-flags) — runtime + database combos

## Commit Guidance

These 8 tasks are split into 4 commits — one per crate (Cargo.toml + src stub together):

```bash
git commit -m "feat(chronos-core): scaffold crate with initial dependencies

Step 1, Tasks 1.2–1.3"

git commit -m "feat(chronos-capture): scaffold crate with xcap and tokio sync

Step 1, Tasks 1.4–1.5"

git commit -m "feat(chronos-inference): scaffold crate with reqwest and base64

Step 1, Tasks 1.6–1.7"

git commit -m "feat(chronos-daemon): scaffold binary crate with sqlx and clap

Step 1, Tasks 1.8–1.9"
```

**Rationale:** A `Cargo.toml` without its `src/lib.rs` won't compile, so they must be committed together. But each crate is an independent logical unit of work.

## Acceptance Criteria

- All 4 directories exist under `crates/`
- Each has a `Cargo.toml` and a `src/lib.rs` (or `src/main.rs` for daemon)
- `cargo check --workspace` starts resolving (may download deps on first run)
- Cross-crate `path` dependencies resolve correctly
