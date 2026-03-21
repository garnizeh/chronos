---
trigger: always_on
---

# Clean Boundaries & Testability

- **Trait-Based Decoupling:** Use Rust `trait`s to define boundaries for side-effects. The core logic should not know it is talking to `ollama` or capturing a `Pop!_OS` screen. It should interact with traits like `ImageCapture` or `VisionInference`.
- **Mocking for Tests:** Because the system relies on heavy hardware interactions (GPUs, screen capture), the core logic MUST be testable without them. The agent must generate mock implementations of these traits for unit testing (e.g., a mock capture that returns a static byte array, or a mock LLM that returns a hardcoded string).
- **Dependency Injection:** Pass these trait implementations into your core services using dependency injection (e.g., passing a generic `T: VisionInference` or using dynamic dispatch with `Box<dyn VisionInference>` if the overhead is negligible).