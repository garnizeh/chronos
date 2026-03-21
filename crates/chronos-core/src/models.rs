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
    pub id: Ulid,
    pub timestamp: DateTime<Utc>,
    pub image_data: Vec<u8>,
    pub width: u32,
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
    pub id: Ulid,
    pub timestamp: DateTime<Utc>,
    pub source_frame_id: Ulid,
    pub description: String,
    pub active_application: Option<String>,
    pub activity_category: Option<String>,
    pub key_entities: Vec<String>,
    pub confidence_score: f64,
    pub raw_vlm_response: String,
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
        let deserialized: SemanticLog = serde_json::from_str(&json_str).expect("Failed to deserialize SemanticLog");

        // Assert all fields match after round-trip
        assert_eq!(log, deserialized);
        assert_eq!(deserialized.key_entities.len(), 2);
    }
}
