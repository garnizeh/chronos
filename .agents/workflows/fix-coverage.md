---
description: achieve maximum pragmatic test coverage for the Chronos monorepo
---

Act as a Senior QA Automation Engineer and Rust Expert. Your objective is to achieve maximum pragmatic test coverage for the Chronos monorepo without compromising the project's architecture or introducing brittle, non-deterministic tests.

## Execution Steps

1. **Run Coverage**: Execute `cargo llvm-cov --workspace --lcov --output-path lcov.info` (or parse the existing `lcov.info`/HTML report) to map the current test coverage.

2. **Analyze Gaps**: Identify all lines with 0 hits (Count: 0), missed branches (especially `?` early returns), and untested `Err(_)` states.

3. **Autonomous Fixes**: For pure business logic, internal state machines, or data transformations, autonomously write the missing `#[test]` or `#[tokio::test]` blocks. Use existing mock structures (e.g., `MockSource`) to trigger the missing branches.

## Architectural Guardrails (CRITICAL)

If an uncovered block of code falls into any of the following categories, you MUST STOP and refuse to write the test automatically:

- Raw OS/Hardware APIs (e.g., X11, Wayland, raw file system I/O).
- Hardcoded time/delays that would make the test suite slow.
- Third-party daemon dependencies (e.g., requiring a live Ollama instance).

## Conflict Resolution Protocol

When you hit a guardrail or an architectural conflict, you must halt execution and output a structured report to the user containing:

- **The Problem**: Explain exactly what is uncovered and why it violates the guardrails (e.g., "Testing `capture_primary` requires a live X11 server").
- **Detailed Documentation (MANDATORY)**: Document the gap directly in the source code using a comment (e.g., `// [JUSTIFIED GAP]: ...`) immediately before the untested block. Explain the technical reason and why it is acceptable.
- **Option A (Refactor for Inversion of Control)**: Propose abstracting the heavy dependency behind a Rust trait so it can be mocked.
- **Option B (Exempt from Coverage)**: Propose ignoring the function using the `#[coverage(off)]` attribute (if using nightly/specific flags) or simply accepting it as an intentional gap.
- **Option C (Integration Test)**: Propose moving this specific check to an isolated integration test that only runs in a specific CI environment.

Await the user's decision before proceeding with the specific file.
