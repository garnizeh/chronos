# Workspace Root Setup

> **Step:** 1 | **Tasks:** 1.1 | **Crate(s):** workspace root

## Why This Matters

Cargo workspaces are the foundation of a multi-crate Rust project. The root `Cargo.toml` declares which crates exist, ensures a single `Cargo.lock` for reproducible builds, and lets you run `cargo check --workspace` across everything at once.

**Go parallel:** This is exactly `go.work` (Go 1.18+). A Go workspace says _"these modules live together"_; a Cargo workspace says _"these crates share one `target/` and one lockfile."_ The difference: Cargo workspaces also allow shared dependency versions via `[workspace.dependencies]`, which reduces version drift — Go doesn't have a direct equivalent.

## Tasks

### Task 1.1 — Create root `Cargo.toml`

**What:** Create a `Cargo.toml` at the repo root that declares the workspace with all 4 member crates.

**File:** `/Cargo.toml`

**Content:**

```toml
[workspace]
resolver = "2"
members = [
    "crates/chronos-core",
    "crates/chronos-capture",
    "crates/chronos-inference",
    "crates/chronos-daemon",
]
```

**Details:**

- **`resolver = "2"`** — Uses Rust 2021 dependency resolver, which handles feature unification correctly. Without this, you can get surprising behaviour where enabling a feature in one crate silently enables it in another. Always use `"2"` for new projects.
- **Member ordering** — List `chronos-core` first since it's the foundational crate. The order doesn't affect compilation (Cargo resolves the dependency graph), but it helps human readers understand the hierarchy.
- **No `[workspace.dependencies]` yet** — We'll add shared dependency versions in Step 1 task files for each crate. Optionally, we can centralise them later as a refactor, but starting simple is fine.

**Gotchas:**

- Do NOT add a `[package]` section to the root `Cargo.toml` — the workspace root is not itself a crate (this is a common Rust beginner mistake; Go has no equivalent confusion since `go.work` is clearly separate from `go.mod`).
- The `members` paths must match the exact directory structure you create in task 1.2+.

## Pre-Research

Before starting, ensure you understand:
- [ ] [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) — official docs
- [ ] Resolver v2 behaviour: [RFC 2957](https://rust-lang.github.io/rfcs/2957-cargo-features2.html)

## Commit Guidance

```bash
git commit -m "chore: create root Cargo.toml workspace

Step 1, Task 1.1"
```

## Acceptance Criteria

- `Cargo.toml` exists at repo root with `[workspace]` section
- No `[package]` section in root `Cargo.toml`
- All 4 member paths listed
- `resolver = "2"` set
