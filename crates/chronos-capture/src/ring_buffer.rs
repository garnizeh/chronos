use chronos_core::models::Frame;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// A thread-safe, bounded ring buffer for managing captured frames in memory.
///
/// **Go Parallel (Didactic):** In Go, this is exactly like a struct with a `sync.Mutex`
/// and a slice. However, instead of passing pointers (`*Frame`) which requires manual
/// heap management, we use `Arc<Frame>` (Atomic Reference Count). This ensures that
/// image data isn't copied when moving between threads, similar to how Go handles
/// pointers to large structs under the hood.
/// **Thread-Safety & Locking Strategy:**
/// Methods like `push` and `latest` use `self.buffer.lock().unwrap()`.
/// Using `unwrap()` here is an **intentional design choice**: if a thread panics
/// while holding the lock, the `Mutex` becomes "poisoned."
///
/// In this high-frequency capture context, a poisoned buffer represents an
/// unrecoverable internal state. We prefer a clean panic (failing fast) over
/// attempting to operate on potentially corrupted data.
///
/// *Callers should note:* If recovery is required, one could use `lock().map_err()`
/// or `PoisonError::into_inner()` to attempt to retrieve the data, but for
/// Chronos, we treat this as a fatal system failure.
#[derive(Clone)]
pub struct FrameRingBuffer {
    /// The underlying synchronized storage. We use `Arc` to allow cloning the
    /// buffer handle across threads, and `Mutex` to guard the `VecDeque`.
    /// Each frame inside is also wrapped in an `Arc` to avoid copying image data.
    buffer: Arc<Mutex<VecDeque<Arc<Frame>>>>,
    /// Maximum number of frames the buffer is allowed to hold before evicting the oldest.
    capacity: usize,
}

impl FrameRingBuffer {
    /// Creates a new empty ring buffer with the specified maximum capacity.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is 0, as a zero-capacity buffer would break internal
    /// eviction logic.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "FrameRingBuffer capacity must be at least 1");
        Self {
            buffer: Arc::new(Mutex::new(VecDeque::with_capacity(capacity))),
            capacity,
        }
    }

    /// Pushes a new frame into the buffer.
    /// If the buffer is already at capacity, the oldest frame is dropped.
    ///
    /// Notice how this takes `&self` but allows mutation? That's "Interior Mutability"
    /// in Rust. The `Mutex` provides the safety check at runtime, allowing multiple
    /// threads to push concurrently.
    pub fn push(&self, frame: Frame) {
        let mut guard = self.buffer.lock().unwrap();
        while guard.len() >= self.capacity {
            guard.pop_front();
        }
        guard.push_back(Arc::new(frame));
    }

    /// Returns the current number of frames in the buffer.
    pub fn len(&self) -> usize {
        self.buffer.lock().unwrap().len()
    }

    /// Returns `true` if the buffer contains no frames.
    pub fn is_empty(&self) -> bool {
        self.buffer.lock().unwrap().is_empty()
    }

    /// Returns a shared reference (`Arc`) to the most recently added frame.
    ///
    /// By returning `Arc<Frame>`, we avoid cloning the large `image_data` Vec.
    /// Multiple consumers can hold an `Arc` to the same frame simultaneously.
    pub fn latest(&self) -> Option<Arc<Frame>> {
        let guard = self.buffer.lock().unwrap();
        guard.back().cloned()
    }

    /// Returns a snapshot of all frames currently in the buffer.
    pub fn to_vec(&self) -> Vec<Arc<Frame>> {
        let guard = self.buffer.lock().unwrap();
        guard.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::thread;
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
        let rb = FrameRingBuffer::new(5);

        rb.push(create_dummy_frame(100));
        rb.push(create_dummy_frame(200));

        assert_eq!(rb.len(), 2);
        assert!(!rb.is_empty());
    }

    #[test]
    fn test_push_drops_oldest_when_full() {
        let rb = FrameRingBuffer::new(2);

        let frame1 = create_dummy_frame(100);
        let frame2 = create_dummy_frame(200);
        let frame3 = create_dummy_frame(300); // This should evict frame1

        rb.push(frame1);
        rb.push(frame2);
        rb.push(frame3);

        assert_eq!(rb.len(), 2, "Capacity should not exceed 2");

        let frames = rb.to_vec();
        assert_eq!(frames.len(), 2);

        // The frames remaining should be frame2 and frame3
        assert_eq!(frames[0].width, 200, "Oldest should now be frame2");
        assert_eq!(frames[1].width, 300, "Newest should be frame3");

        // Explicitly verify frame1 is gone and frame2 is still there
        assert!(
            frames.iter().all(|f| f.width != 100),
            "frame1 should have been evicted"
        );
        assert!(
            frames.iter().any(|f| f.width == 200),
            "frame2 should still be present"
        );
    }

    #[test]
    fn test_latest_returns_most_recent() {
        let rb = FrameRingBuffer::new(3);

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

    #[test]
    fn test_concurrent_push() {
        let rb = FrameRingBuffer::new(100);
        let rb_clone = rb.clone();

        let handle = thread::spawn(move || {
            for i in 0..50 {
                rb_clone.push(create_dummy_frame(i));
            }
        });

        for i in 50..100 {
            rb.push(create_dummy_frame(i));
        }

        handle.join().unwrap();
        assert_eq!(rb.len(), 100);
    }

    #[test]
    #[should_panic(expected = "FrameRingBuffer capacity must be at least 1")]
    fn test_new_with_zero_capacity_panics() {
        let _ = FrameRingBuffer::new(0);
    }
}
