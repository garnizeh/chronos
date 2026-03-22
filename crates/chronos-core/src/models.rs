use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

/// Represents a single captured frame from the screen.
///
/// **Go Parallel (Didactic):** This is a simple data struct. Since we don't want
/// to serialize `Frame` to disk or over the network, we explicitly *do not* add
/// `Serialize` or `Deserialize` derivations (akin to omitting `json:"..."` tags in Go).
/// This enforces our architecture constraint at compile time: Frames live purely in RAM.
#[derive(Debug, Clone)]
pub struct Frame {
    /// Unique identifier for this specific capture event.
    pub id: Ulid,
    /// Exact moment the frame was captured.
    pub timestamp: DateTime<Utc>,
    /// Raw uncompressed image bytes (e.g., RGBA).
    pub image_data: Vec<u8>,
    /// Screen width in pixels at capture time.
    pub width: u32,
    /// Screen height in pixels at capture time.
    pub height: u32,
}

/// Represents the structured output from the Vision-Language Model (VLM).
///
/// **Go Parallel (Didactic):** In Go, to seamlessly convert a struct to and from JSON
/// (e.g., for an HTTP API or storing in a single DB column), we use `json:"..."` struct
/// tags along with `encoding/json`. In Rust, the `serde` crate provides macros
/// (`#[derive(Serialize, Deserialize)]`) that generate high-performance serialization
/// logic at compile time, achieving the exact same objective with zero runtime reflection cost.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SemanticLog {
    /// Unique identifier for this log entry.
    pub id: Ulid,
    /// Moment the event was recorded.
    pub timestamp: DateTime<Utc>,
    /// Link to the original frame that generated this log.
    pub source_frame_id: Ulid,
    /// Human-readable summary of the screen content.
    pub description: String,
    /// Name of the foreground application (if detectable).
    pub active_application: Option<String>,
    /// High-level activity type (e.g., "Development", "Leisure").
    pub activity_category: Option<String>,
    /// List of specific items or keywords identified.
    pub key_entities: Vec<String>,
    /// Model certainty score from 0.0 to 1.0.
    pub confidence_score: f64,
    /// Full, unparsed JSON output from the VLM for auditability.
    pub raw_vlm_response: String,
}

/// Configuration for the screen capture timing and memory limit.
///
/// **Go Parallel:** In Go, this would often be instantiated via a `NewDefaultCaptureConfig()`
/// function. Rust uses the standard `Default` trait, allowing `CaptureConfig::default()`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CaptureConfig {
    /// Seconds between consecutive screen captures.
    pub interval_seconds: u64,
    /// Maximum number of frames to hold in the RAM ring buffer.
    pub ring_buffer_capacity: usize,
}

impl Default for CaptureConfig {
    fn default() -> Self {
        Self {
            interval_seconds: 30,
            ring_buffer_capacity: 64,
        }
    }
}

/// Configuration for the local VLM engine (Ollama).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VlmConfig {
    /// URL of the local Ollama API server.
    pub ollama_host: String,
    /// Name of the vision model to use (e.g., "moondream").
    pub model_name: String,
    /// Maximum wait time for model inference before failing.
    pub timeout_seconds: u64,
}

impl Default for VlmConfig {
    fn default() -> Self {
        Self {
            ollama_host: "http://localhost:11434".to_string(),
            model_name: "moondream".to_string(),
            timeout_seconds: 60,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_instantiation() {
        let now = Utc::now();
        let frame = Frame {
            id: Ulid::new(),
            timestamp: now,
            image_data: vec![0xFF, 0x00, 0xFF], // Mock RGB bytes
            width: 1920,
            height: 1080,
        };

        assert_eq!(frame.width, 1920);
        assert_eq!(frame.height, 1080);
        assert_eq!(frame.image_data.len(), 3);
        assert_eq!(frame.timestamp, now);

        // Ulids are sortable and non-empty
        assert!(!frame.id.to_string().is_empty());
    }

    #[test]
    fn test_semantic_log_serialization_round_trip() {
        let now = Utc::now();
        let log = SemanticLog {
            id: Ulid::new(),
            timestamp: now,
            source_frame_id: Ulid::new(),
            description: "User is coding in Rust".to_string(),
            active_application: Some("Code".to_string()),
            activity_category: Some("Development".to_string()),
            key_entities: vec!["Rust".to_string(), "VSCode".to_string()],
            confidence_score: 0.95,
            raw_vlm_response: r#"{"description": "User is coding in Rust"}"#.to_string(),
        };

        // Serialize to JSON string (Go `json.Marshal`)
        let json_str = serde_json::to_string(&log).expect("Failed to serialize SemanticLog");

        // Deserialize back to struct (Go `json.Unmarshal`)
        let deserialized: SemanticLog =
            serde_json::from_str(&json_str).expect("Failed to deserialize SemanticLog");

        // Assert all fields match after round-trip
        assert_eq!(log, deserialized);
        assert_eq!(deserialized.key_entities.len(), 2);
    }

    #[test]
    fn test_capture_config_defaults() {
        let config = CaptureConfig::default();
        assert_eq!(config.interval_seconds, 30);
        assert_eq!(config.ring_buffer_capacity, 64);
    }

    #[test]
    fn test_vlm_config_defaults() {
        let config = VlmConfig::default();
        assert_eq!(config.ollama_host, "http://localhost:11434");
        assert_eq!(config.model_name, "moondream");
        assert_eq!(config.timeout_seconds, 60);
    }
}
