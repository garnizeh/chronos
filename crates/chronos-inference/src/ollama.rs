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
    /// Internal HTTP client for Ollama API calls.
    client: reqwest::Client,
    /// Model-specific settings (host, model name, timeout).
    config: VlmConfig,
}

/// The internal data structure representing the VLM's JSON output.
/// This matches the schema we request in the `SCREENSHOT_PROMPT`.
#[derive(Deserialize, Debug)]
struct VlmJsonResponse {
    /// Concise summary of what is happening on screen.
    description: String,
    /// Detected name of the focused window/app.
    active_application: Option<String>,
    /// Activity classification for high-level sorting.
    activity_category: Option<String>,
    /// Specific technologies, names, or topics identified.
    #[serde(default)]
    key_entities: Vec<String>,
    /// The model's own estimation of its analysis accuracy (0.0 to 1.0).
    #[serde(default)]
    confidence_score: f64,
}

const SCREENSHOT_PROMPT: &str = "Analyze this screenshot. Provide a structured JSON response with the following fields: \
                  description (brief summary), active_application (name of the window in focus), \
                  activity_category (e.g., Development, Communication, Browsing), \
                  key_entities (list of relevant technologies, names, or topics), \
                  confidence_score (0.0 to 1.0).";

impl OllamaVision {
    /// Create a new OllamaVision client from configuration.
    /// Sets the HTTP timeout based on VLM configuration.
    pub fn new(config: VlmConfig) -> Result<Self> {
        // Validate timeout
        if config.timeout_seconds == 0 {
            return Err(chronos_core::error::ChronosError::Config(
                "VLM timeout_seconds must be greater than zero".to_string(),
            ));
        }

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            // [JUSTIFIED GAP] reqwest::Client::build() failures are rare (e.g., TLS backend
            // initialization error) and extremely difficult to trigger in a portable unit test.
            .map_err(|e| {
                chronos_core::error::ChronosError::Config(format!(
                    "Failed to build HTTP client: {}",
                    e
                ))
            })?;

        Ok(Self { client, config })
    }

    /// Internal helper to parse the VLM's response text.
    ///
    /// **Why the fallback?**
    /// VLMs (like Moondream) are probabilistic and might occasionally output raw
    /// text instead of valid JSON, even when requested. Instead of failing the entire
    /// pipeline, we treat raw text as a 'description' and assign a low confidence (0.3).
    /// This keeps the system resilient to minor model hallucinations.
    fn parse_vlm_response(raw: &str) -> VlmJsonResponse {
        // Try to parse as JSON first
        if let Ok(mut json) = serde_json::from_str::<VlmJsonResponse>(raw) {
            // Normalize confidence score to 0.0..=1.0
            json.confidence_score = json.confidence_score.clamp(0.0, 1.0);
            return json;
        }

        // Fallback: the VLM might have outputted raw text instead of JSON
        VlmJsonResponse {
            description: raw.to_string(),
            active_application: None,
            activity_category: None,
            key_entities: Vec::new(),
            confidence_score: 0.3, // Already within 0.0..=1.0
        }
    }
}

#[async_trait]
impl VisionInference for OllamaVision {
    async fn analyze_frame(&self, frame: &Frame) -> Result<SemanticLog> {
        // 1. Base64-encode the image data
        let base64_image = general_purpose::STANDARD.encode(&frame.image_data);

        // 2. Build the request body for Ollama
        let body = json!({
            "model": self.config.model_name,
            "prompt": SCREENSHOT_PROMPT,
            "images": [base64_image],
            "stream": false,
            "format": "json"
        });

        // 3. POST to Ollama
        // We use the `/api/generate` endpoint because it allows us to send the image
        // alongside the prompt in a single stateless request.
        let url = format!(
            "{}/api/generate",
            self.config.ollama_host.trim_end_matches('/')
        );
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
            if e.is_timeout() {
                chronos_core::error::ChronosError::Timeout(e.to_string())
            } else {
                chronos_core::error::ChronosError::Inference(format!(
                    "Failed to parse Ollama response JSON: {}",
                    e
                ))
            }
        })?;

        // 5. Parse the inner semantic JSON from the VLM
        let parsed = Self::parse_vlm_response(&ollama_res.response);

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
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn make_test_frame() -> Frame {
        Frame {
            id: Ulid::new(),
            timestamp: chrono::Utc::now(),
            image_data: vec![0, 1, 2, 3],
            width: 10,
            height: 10,
        }
    }

    #[test]
    fn test_ollama_vision_creation() {
        let config = VlmConfig::default();
        let _vision = OllamaVision::new(config).unwrap();
    }

    #[test]
    fn test_ollama_vision_invalid_timeout() {
        let config = VlmConfig {
            timeout_seconds: 0,
            ..VlmConfig::default()
        };
        let result = OllamaVision::new(config);
        assert!(matches!(
            result,
            Err(chronos_core::error::ChronosError::Config(msg)) if msg.contains("timeout_seconds must be greater than zero")
        ));
    }

    #[test]
    fn test_parse_valid_vlm_json() {
        let raw = r#"{
            "description": "User is writing Rust code",
            "active_application": "VS Code",
            "activity_category": "Development",
            "key_entities": ["Rust", "Inference"],
            "confidence_score": 0.95
        }"#;

        let parsed = OllamaVision::parse_vlm_response(raw);
        assert_eq!(parsed.description, "User is writing Rust code");
        assert_eq!(parsed.active_application, Some("VS Code".to_string()));
        assert_eq!(parsed.key_entities, vec!["Rust", "Inference"]);
        assert_eq!(parsed.confidence_score, 0.95);
    }

    #[test]
    fn test_parse_malformed_vlm_json_fallback() {
        let raw = "I see a person working on a computer.";

        let parsed = OllamaVision::parse_vlm_response(raw);
        assert_eq!(parsed.description, raw);
        assert_eq!(parsed.active_application, None);
        assert_eq!(parsed.confidence_score, 0.3);
    }

    #[test]
    fn test_parse_partial_vlm_json() {
        let raw = r#"{
            "description": "Minimal response",
            "confidence_score": 0.5
        }"#;

        let parsed = OllamaVision::parse_vlm_response(raw);
        assert_eq!(parsed.description, "Minimal response");
        assert_eq!(parsed.active_application, None);
        assert!(parsed.key_entities.is_empty());
        assert_eq!(parsed.confidence_score, 0.5);
    }

    #[test]
    fn test_parse_vlm_json_confidence_clamping() {
        // Test clamping upper bound
        let raw_high = r#"{
            "description": "High confidence",
            "confidence_score": 1.5
        }"#;
        let parsed_high = OllamaVision::parse_vlm_response(raw_high);
        assert_eq!(parsed_high.confidence_score, 1.0);

        // Test clamping lower bound
        let raw_low = r#"{
            "description": "Negative confidence",
            "confidence_score": -0.5
        }"#;
        let parsed_low = OllamaVision::parse_vlm_response(raw_low);
        assert_eq!(parsed_low.confidence_score, 0.0);
    }

    #[test]
    fn test_parse_vlm_json_missing_confidence() {
        let raw = r#"{
            "description": "Missing confidence score",
            "active_application": "Firefox"
        }"#;

        let parsed = OllamaVision::parse_vlm_response(raw);
        assert_eq!(parsed.description, "Missing confidence score");
        assert_eq!(parsed.active_application, Some("Firefox".to_string()));
        assert_eq!(parsed.confidence_score, 0.0); // Should default to 0.0
    }

    #[tokio::test]
    async fn test_analyze_frame_success() {
        let mock_server = MockServer::start().await;

        let ollama_response = json!({
            "response": r#"{
                "description": "Mocked screenshot analysis",
                "active_application": "Firefox",
                "activity_category": "Browsing",
                "key_entities": ["GitHub"],
                "confidence_score": 0.88
            }"#
        });

        let expected_body = json!({
            "model": "test-model",
            "prompt": SCREENSHOT_PROMPT,
            "images": [general_purpose::STANDARD.encode(vec![0, 1, 2, 3])],
            "stream": false,
            "format": "json"
        });

        Mock::given(method("POST"))
            .and(path("/api/generate"))
            .and(body_json(expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(ollama_response))
            .mount(&mock_server)
            .await;

        let config = VlmConfig {
            ollama_host: mock_server.uri(),
            model_name: "test-model".to_string(),
            timeout_seconds: 5,
        };
        let vision = OllamaVision::new(config).unwrap();
        let frame = make_test_frame();

        let result = vision.analyze_frame(&frame).await.unwrap();
        assert_eq!(result.description, "Mocked screenshot analysis");
        assert_eq!(result.active_application, Some("Firefox".to_string()));
        assert_eq!(result.confidence_score, 0.88);
        assert_eq!(result.source_frame_id, frame.id);
    }

    #[tokio::test]
    async fn test_analyze_frame_ollama_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/generate"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let config = VlmConfig {
            ollama_host: mock_server.uri(),
            ..VlmConfig::default()
        };
        let vision = OllamaVision::new(config).unwrap();
        let frame = make_test_frame();

        let result = vision.analyze_frame(&frame).await;
        assert!(matches!(
            result,
            Err(chronos_core::error::ChronosError::Inference(_))
        ));
    }

    #[tokio::test]
    async fn test_analyze_frame_timeout() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/generate"))
            .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(2)))
            .mount(&mock_server)
            .await;

        let config = VlmConfig {
            ollama_host: mock_server.uri(),
            timeout_seconds: 1, // Set timeout smaller than delay
            ..VlmConfig::default()
        };
        let vision = OllamaVision::new(config).unwrap();
        let frame = make_test_frame();

        let result = vision.analyze_frame(&frame).await;
        assert!(matches!(
            result,
            Err(chronos_core::error::ChronosError::Timeout(_))
        ));
    }

    #[tokio::test]
    async fn test_analyze_frame_malformed_outer_json() {
        let mock_server = MockServer::start().await;

        // Return invalid JSON that doesn't match OllamaResponse struct
        Mock::given(method("POST"))
            .and(path("/api/generate"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not a json"))
            .mount(&mock_server)
            .await;

        let config = VlmConfig {
            ollama_host: mock_server.uri(),
            ..VlmConfig::default()
        };
        let vision = OllamaVision::new(config).unwrap();
        let frame = make_test_frame();

        let result = vision.analyze_frame(&frame).await;
        assert!(matches!(
            result,
            Err(chronos_core::error::ChronosError::Inference(msg)) if msg.contains("Failed to parse Ollama response JSON")
        ));
    }

    #[tokio::test]
    async fn test_analyze_frame_connection_error() {
        // Use a localhost closed port and short timeout for fast/deterministic failure
        let config = VlmConfig {
            ollama_host: "http://127.0.0.1:54321".to_string(),
            timeout_seconds: 1,
            ..VlmConfig::default()
        };
        let vision = OllamaVision::new(config).unwrap();
        let frame = make_test_frame();

        let result = vision.analyze_frame(&frame).await;
        assert!(matches!(
            result,
            Err(chronos_core::error::ChronosError::Inference(_))
                | Err(chronos_core::error::ChronosError::Timeout(_))
        ));
    }
}
