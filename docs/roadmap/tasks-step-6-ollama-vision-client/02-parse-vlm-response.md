# Task 6.2: Parse VLM Response Logic

**Objective:** Implement an internal helper function to parse Ollama's semantic JSON response, falling back gracefully to text when the VLM hallucinates formatting.

**Mental Map / Go Parallel:** This is similar to calling `json.Unmarshal` into a struct, but with a fallback handler (`if err != nil`) that extracts the raw text. Rust's `serde_json::from_str` handles the parsing, and we match on `Result` to implement the fallback strategy gracefully.

## Implementation Steps

- [ ] In `crates/chronos-inference/src/ollama.rs`, write a private helper:
  - `fn parse_vlm_response(raw: &str, frame_desc_fallback: &str) -> (String, Option<String>, Option<String>, Vec<String>, f64)` (or return a helper struct that will map easily to `SemanticLog`).
- [ ] Primary path: Try to deserialize using `serde_json`.
- [ ] Fallback path: If parsing fails, use the raw string as `description`, set `confidence_score` to a low default (e.g., `0.3`), and leave optionals as `None`.
- [ ] Create an internal struct `VlmJsonResponse` matching the expected Ollama JSON schema (since `SemanticLog` has different fields like ULIDs inherited from Frame). Use `serde::Deserialize` on it.
- [ ] Write `#[cfg(test)]` cases:
  - `test_parse_valid_vlm_json` — feed well-formed JSON, verify fields.
  - `test_parse_malformed_vlm_json_fallback` — feed raw text/garbled JSON, verify fallback uses raw text with low confidence.
  - `test_parse_partial_vlm_json` — feed JSON missing some optional fields.
- [ ] Run `cargo clippy -p chronos-inference -- -D warnings`.
- [ ] Run `cargo test -p chronos-inference`.
- [ ] Stage changes and execute the commit.

## Code Scaffolding

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct VlmJsonResponse {
    description: String,
    active_application: Option<String>,
    activity_category: Option<String>,
    #[serde(default)]
    key_entities: Vec<String>,
    confidence_score: f64,
}

// Implement parsing helper...
```

## Conventional Commit
`feat(chronos-inference): implement json fallback parsing for vlm responses`
