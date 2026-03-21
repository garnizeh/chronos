# Task 6.4: Module Exports and Verification

**Objective:** Expose the `ollama` module to the rest of the workspace and run full verifications for the `chronos-inference` crate.

**Mental Map / Go Parallel:** This is the equivalent of making a package public and ensuring `go test ./...` and `golangci-lint` pass cleanly. Adding the module to `lib.rs` integrates it into the crate compile phase strictly.

## Implementation Steps

- [ ] Update `crates/chronos-inference/src/lib.rs` to declare and re-export the module:
  ```rust
  pub mod ollama;
  ```
- [ ] Run `cargo fmt --all -- --check`. Format if needed.
- [ ] Run `cargo clippy -p chronos-inference -- -D warnings`. Ensure ZERO warnings.
- [ ] Run `cargo test -p chronos-inference`. Ensure ALL tests pass.
- [ ] Open the main roadmap document (`docs/roadmap/0001-milestone-01-mvp-roadmap.md`) and mark all Step 6 checklist items as complete (`[x]`).
- [ ] Review `cargo llvm-cov` to verify coverage on the new parsing and struct initialization methods (excluding actual HTTP paths). If there are gaps without `[JUSTIFIED GAP]` comments, fix them via `@[/fix-coverage]`.
- [ ] Stage changes and execute the commit.

## Conventional Commit
`chore(chronos-inference): export ollama module and finalize step 6 verification`
