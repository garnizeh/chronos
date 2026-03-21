use chronos_core::models::Frame;
use std::collections::VecDeque;

/// A bounded ring buffer for managing captured frames in memory.
///
/// **Go Parallel (Didactic):** In Go, if you want a fixed-size queue that deliberately
/// drops the oldest item when full (rather than blocking the sender like a standard
/// buffered `chan`), you would typically use a slice with a `sync.Mutex` and manually
/// manage the indices, or use the `container/ring` package.
///
/// In Rust, `std::collections::VecDeque` is the standard double-ended queue.
/// We wrap it in this struct to safely enforce our specific back-pressure policy:
/// "When memory is full, drop the oldest frame, never block the capture."
pub struct FrameRingBuffer {
    buffer: VecDeque<Frame>,
    capacity: usize,
}

impl FrameRingBuffer {
    /// Creates a new empty ring buffer with the specified maximum capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Pushes a new frame into the buffer.
    /// If the buffer is already at capacity, the oldest frame (at the front)
    /// is silently dropped to make room for the newest frame (at the back).
    pub fn push(&mut self, frame: Frame) {
        if self.buffer.len() == self.capacity {
            // Buffer is full. Pop the oldest to enforce our memory limit.
            self.buffer.pop_front();
        }
        self.buffer.push_back(frame);
    }

    /// Returns the current number of frames in the buffer.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns `true` if the buffer contains no frames.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Returns a reference to the most recently added frame (the back of the queue).
    /// Returns `None` if the buffer is empty.
    ///
    /// Note: It returns `Option<&Frame>`, which in Go maps roughly to returning
    /// `(*Frame, bool)`. The borrow checker ensures the returned reference
    /// cannot outlive the buffer itself.
    pub fn latest(&self) -> Option<&Frame> {
        self.buffer.back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use ulid::Ulid;

    /// Helper to generate a dummy frame for testing.
    fn create_dummy_frame(width: u32) -> Frame {
        Frame {
            id: Ulid::new(),
            timestamp: Utc::now(),
            image_data: vec![], // Empty image for these tests
            width,
            height: 1080,
        }
    }

    #[test]
    fn test_push_within_capacity() {
        let mut rb = FrameRingBuffer::new(5);

        rb.push(create_dummy_frame(100));
        rb.push(create_dummy_frame(200));

        assert_eq!(rb.len(), 2);
        assert!(!rb.is_empty());
    }

    #[test]
    fn test_push_drops_oldest_when_full() {
        let mut rb = FrameRingBuffer::new(2);

        let frame1 = create_dummy_frame(100);
        let frame2 = create_dummy_frame(200);
        let frame3 = create_dummy_frame(300); // This should evict frame1

        rb.push(frame1.clone());
        rb.push(frame2.clone());
        rb.push(frame3.clone());

        assert_eq!(rb.len(), 2, "Capacity should not exceed 2");

        // The front of the internal buffer should now be frame2, and back frame3.
        // Let's verify by popping manually from the internal deque,
        // though `latest` gives us the back.
        let latest = rb.latest().expect("Buffer should not be empty");
        assert_eq!(latest.width, 300, "Latest frame should be frame3");
    }

    #[test]
    fn test_latest_returns_most_recent() {
        let mut rb = FrameRingBuffer::new(3);

        rb.push(create_dummy_frame(100));
        assert_eq!(rb.latest().unwrap().width, 100);

        rb.push(create_dummy_frame(200));
        assert_eq!(rb.latest().unwrap().width, 200);
    }

    #[test]
    fn test_empty_buffer() {
        let rb = FrameRingBuffer::new(10);
        assert!(rb.is_empty());
        assert_eq!(rb.len(), 0);
        assert!(rb.latest().is_none());
    }
}
