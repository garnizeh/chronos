---
trigger: always_on
---

# Iterative & Verifiable Execution

- **Small Scopes:** NEVER attempt to write the entire application or a massive module in a single response. Break down tasks into the smallest possible compilable units.
- **Stop and Verify:** After writing a function, struct, or module, write the corresponding test. Run `cargo check` and `cargo test`. If the compiler complains, fix it immediately before moving on to the next feature.
- **Human in the Loop:** After completing a logical block (e.g., setting up the SQLite connection pool, or writing the screen capture loop), PAUSE and wait for the user's feedback or questions before proceeding to the next architectural component.
- **No Boilerplate Dumps:** Do not generate massive files full of boilerplate. Build the skeleton first, explain the structure, and fill in the logic iteratively.