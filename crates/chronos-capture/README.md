# chronos-capture

## Role
**OS-Level Screen Acquisition.**
`chronos-capture` is responsible for interacting with the operating system to acquire desktop frames. It currently supports Linux (X11) via the `xcap` crate and provides a modular interface for future Wayland or Windows support.

## Architectural Constraints
- **Privacy-First (RAM-Only)**: To prevent SSD wear and tear and ensure absolute privacy, captured pixels must reside strictly in a RAM-only buffer/ring buffer. Raw images must **NEVER** touch the physical disk.
- **Async Boundary Control**: Platform-specific capture calls are blocking by nature. To avoid starving the Tokio async executor, all capture loops must run on a dedicated OS thread (`std::thread::spawn`) or be wrapped in `tokio::task::spawn_blocking`.
- **Modularity**: All OS-level interactions are hidden behind the `ImageCapture` trait (from `chronos-core`) and an internal `CaptureSource` trait to allow seamless swapping of backends.

## Testing Strategy
- **Trait-Based Mocking**: Use dependency injection to pass a `MockSource` into the capture logic, allowing for 100% predictable unit tests without requiring a real display server (X11) in CI.
- **Concurrency Verification**: Tests must explicitly verify graceful shutdown signals (`tokio::sync::watch`) and backpressure handling in the frame channels.
