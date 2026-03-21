# Task 5.2: X11 Capture Implementation

## Objective
Implement `X11Capture`, satisfying `ImageCapture` by leveraging `xcap` running on a dedicated OS thread. Ensure blocking IO does not starve the Tokio runtime.

## Mental Map / Go Parallel
In Go, `go captureLoop(ctx, channel)` inherently schedules onto an OS thread. Rust's `tokio::spawn` schedules onto the async executor's thread pool, which can be starved by synchronous calls (like interacting with X11 window managers). Thus, we must use `std::thread::spawn` or `tokio::task::spawn_blocking`. Because this is an infinite loop, `std::thread::spawn` is preferred to maintain a dedicated thread entirely isolated from Tokio's green threads. The `std::sync::mpsc` or `tokio::sync::mpsc` channels form the bridge.

## Implementation Steps
- [x] Create `crates/chronos-capture/src/x11.rs`.
- [x] Define the `X11Capture` struct:
  ```rust
  pub struct X11Capture {
      config: CaptureConfig,
  }
  ```
- [x] Implement `X11Capture::new(config: CaptureConfig) -> Self`.
- [x] Implement `ImageCapture` for `X11Capture`:
  - `capture_frame(&self) -> Result<Frame>`:
    - Call `xcap::Monitor::all()` to get a list of active monitors. Take the primary (first).
    - Call `capture_image()` on the monitor.
    - Convert the `RgbaImage` to a PNG byte vector directly in RAM.
    - Pack into `chronos_core::models::Frame`.
    - Map `xcap` errors or PNG encoding errors to `ChronosError::Capture`.
    - Note: Because `capture_frame` is `async` in the trait but `xcap` blocks, wrap the synchronous portion in `tokio::task::spawn_blocking` to prevent starving the executor.
- [x] Add testing module `#[cfg(test)]`:
  - [x] `test_x11_capture_creation`
  - [x] `test_capture_config_defaults` (if relying on struct properties locally).
  - *Note: Don't write a real compile-time test for `xcap` taking a screenshot unless you guard it with `#[cfg(feature = "x11")]` as it will fail on headless CI.*

## Code Scaffolding
```rust
use async_trait::async_trait;
use chronos_core::error::{ChronosError, Result};
use chronos_core::models::{CaptureConfig, Frame};
use chronos_core::traits::ImageCapture;
use std::io::Cursor;
use ulid::Ulid;
use chrono::Utc;

pub struct X11Capture {
    config: CaptureConfig,
}

impl X11Capture {
    pub fn new(config: CaptureConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl ImageCapture for X11Capture {
    async fn capture_frame(&self) -> Result<Frame> {
        // TODO: implement spawn_blocking with xcap integration
    }
}
```

## Conventional Commit
`feat(chronos-capture): implement x11 capture on dedicated os thread`
