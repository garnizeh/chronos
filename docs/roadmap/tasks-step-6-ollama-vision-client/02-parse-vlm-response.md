# Task 6.2: Parse VLM Response Logic

**Objective:** Implement an internal helper function to parse Ollama's semantic JSON response, falling back gracefully to text when the VLM hallucinates formatting.

**Mental Map / Go Parallel:** This is similar to calling `json.Unmarshal` into a struct, but with a fallback handler (`if err != nil`) that extracts the raw text. Rust's `serde_json::from_str` handles the parsing, and we match on `Result` to implement the fallback strategy gracefully.

## Implementation Steps

- [x] In `crates/chronos-inference/src/ollama.rs`, write a private helper:
  - `fn parse_vlm_response(raw: &str) -> VlmJsonResponse` (Associated function).
- [x] Primary path: Try to deserialize using `serde_json`.
- [x] Fallback path: If parsing fails, use the raw string as `description`, set `confidence_score` to a low default (e.g., `0.3`), and leave optionals as `None`.
- [x] Create an internal struct `VlmJsonResponse` matching the expected Ollama JSON schema (since `SemanticLog` has different fields like ULIDs inherited from Frame). Use `serde::Deserialize` on it.
- [x] Write `#[cfg(test)]` cases:
  - `test_parse_valid_vlm_json` — feed well-formed JSON, verify fields.
  - `test_parse_malformed_vlm_json_fallback` — feed raw text/garbled JSON, verify fallback uses raw text with low confidence.
  - `test_parse_partial_vlm_json` — feed JSON missing some optional fields.
- [x] Run `cargo clippy -p chronos-inference -- -D warnings`.
- [x] Run `cargo test -p chronos-inference`.
- [x] Stage changes and execute the commit.

## Code Scaffolding

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct VlmJsonResponse {
    description: String,
    active_application: Option<String>,
    activity_category: Option<String>,
    #[serde(default)]
    key_entities: Vec<String>,
    confidence_score: f64,
}

impl OllamaVision {
    fn parse_vlm_response(raw: &str) -> VlmJsonResponse {
        // Associated function logic...
        todo!()
    }
}
```

## Conventional Commit
`feat(chronos-inference): implement associated json fallback parsing for vlm responses`
