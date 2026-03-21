---
trigger: glob
globs: **/*.rs
---

# OS Integration & Concurrency

- **Screen Capture:** Use mature crates for desktop capture, keeping the code highly modularized in case we need to swap the underlying implementation in the future.
- **Concurrency:** Isolate the capture thread (which might be blocking depending on the OS API) from the asynchronous processing threads. Use `tokio::task::spawn_blocking` for OS interactions that do not support native async.
- **Communication:** Use `tokio` channels (`mpsc`) to transit images from the capture thread to the visual processing thread.
- **Structured Logging:** Use the `tracing` crate (never `println!`) for all internal telemetry, separating log levels (INFO, DEBUG, ERROR) consistently.