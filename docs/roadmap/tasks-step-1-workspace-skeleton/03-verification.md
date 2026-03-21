# Verification & Cleanup

> **Step:** 1 | **Tasks:** 1.10 – 1.13 | **Crate(s):** workspace-wide

## Why This Matters

These tasks validate that everything created in tasks 1.1–1.9 is correct. Running the full Cargo toolchain (`check`, `fmt`, `clippy`) catches issues _before_ we start writing real logic in Step 2.

This is the pattern we'll repeat at the end of every phase: **build → format → lint**. The CI pipeline (`.github/workflows/ci.yml`) runs exactly these steps, so passing locally means passing in CI.

**Go parallel:** This is `go build ./... && gofmt -d . && go vet ./...`. Same idea — verify the skeleton compiles, follows style, and has no obvious issues.

## Tasks

### Task 1.10 — Verify `.gitignore`

**What:** Confirm the `.gitignore` already covers Rust-specific entries.

This was done in Step 0. Just verify that `/target` is present (the workspace will generate a single `target/` at the root). No changes needed unless something is missing.

**Check:**
```bash
grep -q '/target' .gitignore && echo "OK" || echo "MISSING: add /target"
```

---

### Task 1.11 — `cargo check --workspace`

**What:** Compile-check all crates without producing binaries.

```bash
cargo check --workspace
```

**What to expect:**
- First run will download and compile all dependencies (~1-2 minutes)
- Subsequent runs use cache and take seconds
- Exit code 0 = success

**Common failures:**
- Missing `src/lib.rs` or `src/main.rs` → _"can't find crate root"_
- Typo in path dependency → _"failed to load manifest"_
- Feature flag mismatch → _"feature `X` not found"_

---

### Task 1.12 — `cargo fmt --all -- --check`

**What:** Verify that all Rust files follow standard formatting.

```bash
cargo fmt --all -- --check
```

Since we only have stub files with comments, this should pass trivially. The `--check` flag means _"don't modify files, just exit with error if formatting differs."_

**If it fails:** Run `cargo fmt --all` to auto-fix, then commit the formatted result.

---

### Task 1.13 — `cargo clippy --workspace --all-targets -- -D warnings`

**What:** Run the Rust linter across all crates, treating warnings as errors.

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

**What Clippy catches:** unused imports, redundant closures, performance anti-patterns, potential bugs. It's like `go vet` but much more comprehensive (500+ lint rules).

**`-D warnings`** turns all warnings into hard errors. This matches our CI config and prevents lint debt from accumulating.

**Common false positives in stubs:**
- Empty files might trigger nothing (good)
- If you added `use` statements preemptively, Clippy may flag them as unused

## Commit Guidance

Verification tasks don't produce new files, but if Task 1.10 reveals a missing `.gitignore` entry, or if `fmt` requires changes:

```bash
git commit -m "chore: verify workspace compiles cleanly

Step 1, Tasks 1.10–1.13
- cargo check --workspace: OK
- cargo fmt --all -- --check: OK  
- cargo clippy --workspace -- -D warnings: OK"
```

If everything passes without changes, this commit is optional (the verification is implicit in the CI run on the branch).

## Acceptance Criteria

- [ ] `cargo check --workspace` → exit 0
- [ ] `cargo fmt --all -- --check` → exit 0 (no formatting diff)
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` → exit 0 (no warnings)
- [ ] All 4 crates visible: `cargo metadata --format-version=1 | jq '.workspace_members'`
