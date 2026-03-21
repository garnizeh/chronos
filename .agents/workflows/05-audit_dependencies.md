---
description: Checks the Cargo.lock file for known security vulnerabilities.
---

# Workflow: Audit Dependencies

**Execution Steps:**
1. Check if `cargo-audit` is installed. If not, ask the user for permission to run `cargo install cargo-audit`.
2. Run `cargo audit`.
3. If vulnerabilities are found in the dependency tree, automatically suggest the required version bumps in `Cargo.toml`.