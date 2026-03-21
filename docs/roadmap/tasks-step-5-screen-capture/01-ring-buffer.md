# Task 5.1: Frame Ring Buffer

## Objective
Implement a thread-safe, bounded ring buffer using `Arc<Mutex<...>>` to store captured screen frames and manage back-pressure.

## Mental Map / Go Parallel
This is conceptually similar to a buffered channel in Go `make(chan Frame, capacity)` combined with a standard slice. However, when a Go channel is full, pushing blocks. For Chronos, we explicitly want to *drop the oldest frame* to maintain real-time performance without boundless memory growth. This is like a fixed-size `container/ring` combined with lock-free semantics or explicit `sync.Mutex`.

## Implementation Steps
- [x] Create `crates/chronos-capture/src/ring_buffer.rs`.
- [x] Define the `FrameRingBuffer` struct wrapping a `std::collections::VecDeque<Frame>` and a `usize` capacity.
- [x] Implement `new(capacity: usize) -> Self`.
- [x] Implement `push(&mut self, frame: Frame)`:
  - If `len() == capacity`, call `pop_front()` to drop the oldest.
  - Call `push_back(frame)`.
- [x] Implement `len(&self) -> usize` and `is_empty(&self) -> bool`.
- [x] Implement `latest(&self) -> Option<&Frame>` returning `back()`.
- [x] Write `#[cfg(test)]` block in the same file:
  - [x] `test_push_within_capacity`
  - [x] `test_push_drops_oldest_when_full`
  - [x] `test_latest_returns_most_recent`
  - [x] `test_empty_buffer`
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
