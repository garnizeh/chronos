# chronos-core

## Role
**The System Vocabulary.**
`chronos-core` defines the shared language of the Chronos workspace. It contains the primary data models (`Frame`, `SemanticLog`, `VlmConfig`), shared traits (`ImageCapture`, `VisionInference`), and the central `ChronosError` type.

## Architectural Constraints
- **Zero I/O**: This crate must never perform filesystem, network, or hardware interactions.
- **Pure Synchronous Logic**: Beyond defining `async-trait` markers for other crates to implement, all internal logic must be 100% synchronous.
- **Side-Effect Free**: Functions must be deterministic and pure, facilitating robust unit testing.
- **Trait Heavy**: Models must heavily derive standard Rust traits (`Debug`, `Clone`, `Serialize`, `Deserialize`) to ensure interoperability.

## Testing Strategy
- **100% Logic Coverage**: Every struct method, enum variant, and error mapping must be covered by standard `#[test]` unit tests co-located in the source files.
- **Infallible Operations**: As these models are the base of the system, any logic within `chronos-core` must be bulletproof.
