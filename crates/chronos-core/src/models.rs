use chrono::{DateTime, Utc};
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
}
