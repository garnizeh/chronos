# chronos-inference

## Role
**Semantic Image Understanding.**
`chronos-inference` acts as the client for local Vision Language Models (VLM). It translates raw desktop `Frame` data into structured `SemanticLog` metadata using a local Ollama instance.

## Architectural Constraints
- **Localhost Only**: Strictly communicates with `localhost:11434`. No cloud APIs, no external telemetry, and no data leakage.
- **Structured Telemetry**: All inference cycles and parsing steps must be instrumented using the `tracing` crate (`#[instrument]`).
- **Fallback Resilience**: The parser must handle non-JSON or malformed outputs from the VLM gracefully, falling back to unstructured descriptions with low confidence scores rather than failing the pipeline.

## Testing Strategy
- **HTTP Mocking**: Use the `wiremock` crate to spin up a local mock HTTP server for all tests. Full verification of the `VisionInference` implementation must be possible without a live Ollama daemon running in CI.
- **100% Pragmatic Coverage**: All HTTP request construction, timeout logic, error mapping, and JSON parsing branches must be covered by unit tests.
