# Task 9.3: Final Workspace Verification

**Objective:** Run the full verification suite (formatting, clippy, testing) and optionally perform a manual smoke test on a real X11 machine to ensure the entire workspace is healthy and ready for release.

**Mental Map / Go Parallel:** This step is comparable to running `go fmt`, `go vet`, and `go test ./...` across the entire workspace in Go, ensuring the application is production-ready. 

**Implementation Steps:**
- [ ] Run `cargo fmt --all -- --check` across the workspace and fix any issues found.
- [ ] Run `cargo clippy --workspace --all-targets -- -D warnings` and fix any new warnings.
- [ ] Run `cargo test --workspace` to ensure all tests (including the new integration test) pass.
- [ ] (Optional) Perform a manual smoke test: start Ollama (`ollama serve`), pull moondream (`ollama pull moondream`), run the chronos daemon (`cargo run -p chronos-daemon -- start`), and query the output after 30 seconds to ensure real hardware/system integration works as expected.
- [ ] Update the roadmap (`docs/roadmap/0001-milestone-01-mvp-roadmap.md`) to mark Step 9 items as complete.

**Conventional Commit:** `ci: verify entire workspace and finalize step 9`
