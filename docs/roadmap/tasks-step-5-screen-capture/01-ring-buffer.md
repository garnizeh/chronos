# Task 5.1: Frame Ring Buffer

## Objective
Implement a thread-safe, bounded ring buffer using `Arc<Mutex<...>>` to store captured screen frames and manage back-pressure.

## Mental Map / Go Parallel
This is conceptually similar to a buffered channel in Go `make(chan Frame, capacity)` combined with a standard slice. However, when a Go channel is full, pushing blocks. For Chronos, we explicitly want to *drop the oldest frame* to maintain real-time performance without boundless memory growth. This is like a fixed-size `container/ring` combined with lock-free semantics or explicit `sync.Mutex`.

## Implementation Steps
- [x] Create `crates/chronos-capture/src/ring_buffer.rs`.
- [x] Define a thread-safe `FrameRingBuffer` struct wrapping an `Arc<Mutex<VecDeque<Arc<Frame>>>>` with a `usize` capacity.
- [x] Implement `new(capacity: usize) -> Self` with internal `Arc::new(Mutex::new(...))` initialization.
- [x] Implement `push(&self, frame: Frame)` for concurrent access:
  - Acquire the `Mutex` lock.
  - If `len() == capacity`, call `pop_front()` to drop the oldest.
  - Call `push_back(Arc::new(frame))`.
- [x] Implement `len(&self) -> usize` and `is_empty(&self) -> bool` with internal locking.
- [x] Implement `latest(&self) -> Option<Arc<Frame>>` returning a cloned `Arc` pointer.
- [x] Write `#[cfg(test)]` block in the same file:
  - [x] **Concurrent Tests**: `test_concurrent_push` (multiple threads pushing simultaneously).
  - [x] **Unit Tests**: `test_push_within_capacity`, `test_push_drops_oldest_when_full`, `test_latest_returns_most_recent`, `test_empty_buffer`.
- [x] **Task 5.3: Thread-Safe Ring Buffer**: Finalized the `Arc<Mutex<VecDeque<Arc<Frame>>>>` implementation to avoid large clones and ensure safe cross-thread sharing.
- [x] Run `cargo fmt -p chronos-capture -- --check`.
- [x] Run `cargo test -p chronos-capture`.
- [x] Run `cargo clippy -p chronos-capture -- -D warnings`.

## Code Scaffolding
```rust
use chronos_core::models::Frame;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct FrameRingBuffer {
    buffer: Arc<Mutex<VecDeque<Arc<Frame>>>>,
    capacity: usize,
}

impl FrameRingBuffer {
    pub fn new(capacity: usize) -> Self {
        // ...
    }
    
    pub fn push(&self, frame: Frame) {
        // ...
    }
    
    pub fn latest(&self) -> Option<Arc<Frame>> {
        // ...
    }
}
```

## Conventional Commit
`feat(chronos-capture): implement bounded ring buffer for frames`
