---
trigger: glob
globs: **/*.rs
---

# Rust Pragmatic & Idiomatic Style Guide

- **Pragmatism:** Write simple and straightforward Rust code. Avoid premature abstractions, complex macros, or over-engineering (such as unnecessary traits). Prioritize clarity over "cleverness."
- **Core Ecosystem:** Prefer using the standard library (`std`) whenever possible. When external crates are strictly necessary, use community standards and lean tools. Avoid monolithic frameworks.
- **Error Handling:** Use the `thiserror` crate to create custom errors without boilerplate, and the `?` operator for continuous propagation. Never use `.unwrap()` or `.expect()` in production code; always handle the `Result` explicitly.
- **Formatting and Linting:** All code must strictly adhere to `cargo fmt`. The generated code must not emit any warnings under `cargo clippy -- -D warnings`.
- **Testing:** Generate unit tests in the same source code file using the `#[cfg(test)] mod tests { ... }` convention.