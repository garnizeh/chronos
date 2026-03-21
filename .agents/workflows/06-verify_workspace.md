---
description: Runs the strict compilation and testing suite across the entire Rust monorepo. Use this skill after making structural changes to any crate.
---

# Workflow: Verify Workspace

**Execution Steps:**
1. Run `cargo fmt --all -- --check` to verify strict formatting.
2. Run `cargo clippy --workspace --all-targets -- -D warnings`. If this fails, read the Clippy suggestions, fix the code automatically, and run it again.
3. Run `cargo test --workspace`.
4. Report the final status to the user. Do not proceed with new features until the workspace is fully verified.