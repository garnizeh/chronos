# Task 6.1: Ollama Client Struct & Initialization

**Objective:** Define the `OllamaVision` struct and its initialization method to hold a configured HTTP client and the model configuration.

**Mental Map / Go Parallel:** This is equivalent to setting up a struct that holds `*http.Client` and some configuration strings. In Rust, `reqwest::Client` is the standard HTTP client, and we'll configure timeouts using it.

## Implementation Steps

- [x] Ensure `reqwest` is configured with JSON features in `crates/chronos-inference/Cargo.toml`.
- [x] Create `crates/chronos-inference/src/ollama.rs`.
- [x] Define `OllamaVision` struct:
  - `client: reqwest::Client`
  - `config: VlmConfig` (from `chronos-core::models`)
- [x] Implement `OllamaVision::new(config: VlmConfig) -> Result<Self>`:
  - Create the `reqwest::Client` applying the `config.timeout_seconds` using `.timeout()`.
  - Propagate errors using `map_err` and `?` to avoid `.expect()` in production code.
- [x] Write `#[cfg(test)]` case:
  - `test_ollama_vision_creation` — verify struct construction with default `VlmConfig` using `.unwrap()` in the test.
- [x] Run `cargo check -p chronos-inference` and ensure it compiles.
- [x] Stage changes and execute the commit.

## Code Scaffolding

```rust
use chronos_core::models::VlmConfig;
use chronos_core::error::Result;

pub struct OllamaVision {
    client: reqwest::Client,
    config: VlmConfig,
}

impl OllamaVision {
    pub fn new(config: VlmConfig) -> Result<Self> {
        // Construct the client and return Result
        todo!()
    }
}
```

## Conventional Commit
`feat(chronos-inference): define OllamaVision struct and Result-based HTTP client setup`
