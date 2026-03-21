---
trigger: always_on
---

# Clean Boundaries & Testability

- **Trait-Based Decoupling:** Use Rust `trait`s to define boundaries for side-effects. The core logic should not know it is talking to `ollama` or capturing a `Pop!_OS` screen. It should interact with traits like `ImageCapture` or `VisionInference`.
- **Mocking for Tests:** Because the system relies on heavy hardware interactions (GPUs, screen capture), the core logic MUST be testable without them. The agent must generate mock implementations of these traits for unit testing (e.g., a mock capture that returns a static byte array, or a mock LLM that returns a hardcoded string).
- **Dependency Injection:** Pass these trait implementations into your core services using dependency injection (e.g., passing a generic `T: VisionInference` or using dynamic dispatch with `Box<dyn VisionInference>` if the overhead is negligible).

# Testing Health & Coverage Gaps

- **100% Target:** Aim for 100% test coverage for all business logic, data parsing, and concurrency coordination.
- **Architectural Gaps:** When 100% coverage is physically impossible due to environmental constraints, document the gap as an "Architectural Boundary." Acceptable gaps include:
    - **Hardware/OS Drivers:** Code that directly invokes non-mockable system APIs (e.g., `xcap::Monitor`, `tokio::main`).
    - **Infallible Operations:** Error paths for operations that cannot realistically fail given the input types (e.g., PNG encoding a valid `RgbaImage`, serializing a simple `Vec<String>`).
    - **Third-Party Faults:** Error handling for unrecoverable third-party driver failures (e.g., SQLite connection loss during an active transaction).
- **Justification:** Every gap must be identified via `cargo llvm-cov` and justified in the module's source code using `// [JUSTIFIED GAP]` comments (as per `.agents/workflows/fix-coverage.md`).
- **Automatic Enforcement:** After executing `just ci-local`, the agent MUST analyze the coverage report. If any non-justified logic coverage gap exists, the agent MUST immediately invoke the library workflow `@[/fix-coverage]` to resolve or document the gap before committing or starting a new step.