# Task 6.3: VisionInference Trait Implementation

**Objective:** Implement the `VisionInference` trait for `OllamaVision`, wiring up the HTTP POST request to Ollama with base64 encoded images.

**Mental Map / Go Parallel:** This is writing the logic that satisfies the interface method. We construct the request body, make the blocking (or async) HTTP call using `reqwest`, handle protocol errors (`?` operator), and return the mapped `Result<SemanticLog>`.

## Implementation Steps

- [ ] Ensure `base64` crate is in dependencies for `chronos-inference`.
- [ ] In `crates/chronos-inference/src/ollama.rs`, add `#[async_trait]` and `impl VisionInference for OllamaVision`.
- [ ] Implement `async fn analyze_frame(&self, frame: &Frame) -> Result<SemanticLog>`:
  - Base64-encode the `frame.image_data`.
  - Build the JSON request body targeting Ollama's `/api/generate` endpoint format (model, prompt, images, stream: false, format: json).
  - Issue the `POST` request to `{config.ollama_host}/api/generate`.
  - Extract the raw JSON text from the response.
  - Call the `parse_vlm_response` helper created in Task 6.2.
  - Map the results into a fully formed `SemanticLog` returning `Ok()`.
- [ ] Handle errors: Map `reqwest` HTTP/Timeout errors to `ChronosError::Inference` and `ChronosError::Timeout` via `Result` map hooks or `From` implementations.
- [ ] Write `#[cfg(test)]` case (optional but recommended if wiremock is added, otherwise skip HTTP tests for now per the roadmap).
- [ ] Run `cargo check -p chronos-inference`.
- [ ] Run `cargo fmt --all`.
- [ ] Stage changes and execute the commit.

## Code Scaffolding

```rust
use chronos_core::traits::VisionInference;
use chronos_core::models::{Frame, SemanticLog};
use chronos_core::error::Result;
use async_trait::async_trait;

#[async_trait]
impl VisionInference for OllamaVision {
    async fn analyze_frame(&self, frame: &Frame) -> Result<SemanticLog> {
        // Base64 encode image
        // Construct API request to Ollama
        // Post request and extract body text
        // Pass text to parsing helper
        // Build SemanticLog
        todo!()
    }
}
```

## Conventional Commit
`feat(chronos-inference): implement VisionInference trait for OllamaVision HTTP client`
