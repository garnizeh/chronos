use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose};
use chronos_core::error::Result;
use chronos_core::models::{Frame, SemanticLog, VlmConfig};
use chronos_core::traits::VisionInference;
use serde::Deserialize;
use serde_json::json;
use std::time::Duration;
use ulid::Ulid;

/// The Ollama vision-language model client.
/// Uses `reqwest` to communicate with a local Ollama instance.
pub struct OllamaVision {
    client: reqwest::Client,
    config: VlmConfig,
}

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
    /// Create a new OllamaVision client from configuration.
    /// Sets the HTTP timeout based on VLM configuration.
    pub fn new(config: VlmConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to build reqwest client");

        Self { client, config }
    }

    /// Internal helper to parse the VLM's response text.
    /// If the response is valid JSON, it maps it to the expected fields.
    /// If not, it falls back to using the entire text as a description with low confidence.
    fn parse_vlm_response(&self, raw: &str) -> VlmJsonResponse {
        // Try to parse as JSON first
        if let Ok(json) = serde_json::from_str::<VlmJsonResponse>(raw) {
            return json;
        }

        // Fallback: the VLM might have outputted raw text instead of JSON
        VlmJsonResponse {
            description: raw.to_string(),
            active_application: None,
            activity_category: None,
            key_entities: Vec::new(),
            confidence_score: 0.3, // Low confidence for unstructured fallback
        }
    }
}

#[async_trait]
impl VisionInference for OllamaVision {
    async fn analyze_frame(&self, frame: &Frame) -> Result<SemanticLog> {
        // 1. Base64-encode the image data
        let base64_image = general_purpose::STANDARD.encode(&frame.image_data);

        // 2. Build the request body for Ollama
        let prompt = "Analyze this screenshot. Provide a structured JSON response with the following fields: \
                      description (brief summary), active_application (name of the window in focus), \
                      activity_category (e.g., Development, Communication, Browsing), \
                      key_entities (list of relevant technologies, names, or topics), \
                      confidence_score (0.0 to 1.0).";

        let body = json!({
            "model": self.config.model_name,
            "prompt": prompt,
            "images": [base64_image],
            "stream": false,
            "format": "json"
        });

        // 3. POST to Ollama
        let url = format!("{}/api/generate", self.config.ollama_host);
        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    chronos_core::error::ChronosError::Timeout(e.to_string())
                } else {
                    chronos_core::error::ChronosError::Inference(e.to_string())
                }
            })?;

        let status = response.status();
        if !status.is_success() {
            return Err(chronos_core::error::ChronosError::Inference(format!(
                "Ollama returned error status: {}",
                status
            )));
        }

        // 4. Parse Ollama's outer JSON response
        #[derive(Deserialize)]
        struct OllamaResponse {
            response: String,
        }

        let ollama_res: OllamaResponse = response.json().await.map_err(|e| {
            chronos_core::error::ChronosError::Inference(format!(
                "Failed to parse Ollama response JSON: {}",
                e
            ))
        })?;

        // 5. Parse the inner semantic JSON from the VLM
        let parsed = self.parse_vlm_response(&ollama_res.response);

        // 6. Map to SemanticLog
        Ok(SemanticLog {
            id: Ulid::new(),
            timestamp: chrono::Utc::now(),
            source_frame_id: frame.id,
            description: parsed.description,
            active_application: parsed.active_application,
            activity_category: parsed.activity_category,
            key_entities: parsed.key_entities,
            confidence_score: parsed.confidence_score,
            raw_vlm_response: ollama_res.response,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chronos_core::models::VlmConfig;

    #[test]
    fn test_ollama_vision_creation() {
        let config = VlmConfig::default();
        let _vision = OllamaVision::new(config);
    }

    #[test]
    fn test_parse_valid_vlm_json() {
        let vision = OllamaVision::new(VlmConfig::default());
        let raw = r#"{
            "description": "User is writing Rust code",
            "active_application": "VS Code",
            "activity_category": "Development",
            "key_entities": ["Rust", "Inference"],
            "confidence_score": 0.95
        }"#;

        let parsed = vision.parse_vlm_response(raw);
        assert_eq!(parsed.description, "User is writing Rust code");
        assert_eq!(parsed.active_application, Some("VS Code".to_string()));
        assert_eq!(parsed.key_entities, vec!["Rust", "Inference"]);
        assert_eq!(parsed.confidence_score, 0.95);
    }

    #[test]
    fn test_parse_malformed_vlm_json_fallback() {
        let vision = OllamaVision::new(VlmConfig::default());
        let raw = "I see a person working on a computer.";

        let parsed = vision.parse_vlm_response(raw);
        assert_eq!(parsed.description, raw);
        assert_eq!(parsed.active_application, None);
        assert_eq!(parsed.confidence_score, 0.3);
    }

    #[test]
    fn test_parse_partial_vlm_json() {
        let vision = OllamaVision::new(VlmConfig::default());
        let raw = r#"{
            "description": "Minimal response",
            "confidence_score": 0.5
        }"#;

        let parsed = vision.parse_vlm_response(raw);
        assert_eq!(parsed.description, "Minimal response");
        assert_eq!(parsed.active_application, None);
        assert!(parsed.key_entities.is_empty());
        assert_eq!(parsed.confidence_score, 0.5);
    }
}
