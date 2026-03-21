---
name: Rust Feature TDD Loop
description: Step-by-step workflow for implementing a new module or feature in Rust using TDD and interface-driven design.
trigger: "Implement feature: [feature_name]"
---
# Workflow: Rust Feature TDD

**Step 1: Contract Definition (Traits)**

- Define the public interface using Rust `trait`.
- Document the expected behavior, inputs, and custom `Result<T, Error>` returns.
- Pause and ask the user to approve the Trait design before proceeding.

**Step 2: Mocking & Tests**

- Create a mock implementation of the Trait.
- Write unit tests (`#[cfg(test)]`) covering the happy path and expected error states.
- Ensure the tests compile (`cargo check --tests`).

**Step 3: Concrete Implementation**

- Write the actual implementation of the struct.
- Wire the implementation to satisfy the Trait.
- Run `cargo test` to verify the implementation passes the previously written tests.

**Step 4: Refactor & Lint**

- Run `cargo clippy -- -D warnings`.
- Fix any warnings or unidiomatic code automatically.
- Present the final diff to the user.
