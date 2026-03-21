# Task 6.1: Ollama Client Struct & Initialization

**Objective:** Define the `OllamaVision` struct and its initialization method to hold a configured HTTP client and the model configuration.

**Mental Map / Go Parallel:** This is equivalent to setting up a struct that holds `*http.Client` and some configuration strings. In Rust, `reqwest::Client` is the standard HTTP client, and we'll configure timeouts using it.

## Implementation Steps

- [ ] Ensure `reqwest` is configured with JSON features in `crates/chronos-inference/Cargo.toml`.
- [ ] Create `crates/chronos-inference/src/ollama.rs`.
- [ ] Define `OllamaVision` struct:
  - `client: reqwest::Client`
  - `config: VlmConfig` (from `chronos-core::models`)
- [ ] Implement `OllamaVision::new(config: VlmConfig) -> Self`:
  - Create the `reqwest::Client` applying the `config.timeout_seconds` using `.timeout()`.
- [ ] Write `#[cfg(test)]` case:
  - `test_ollama_vision_creation` — verify struct construction with default `VlmConfig` without panicking.
- [ ] Run `cargo check -p chronos-inference` and ensure it compiles.
- [ ] Stage changes and execute the commit.

## Code Scaffolding

```rust
use chronos_core::models::VlmConfig;

pub struct OllamaVision {
    client: reqwest::Client,
    config: VlmConfig,
}

impl OllamaVision {
    pub fn new(config: VlmConfig) -> Self {
        // Construct the client
        todo!()
    }
}
```

## Conventional Commit
`feat(chronos-inference): define OllamaVision struct and HTTP client setup`
