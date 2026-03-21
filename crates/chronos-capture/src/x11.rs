use async_trait::async_trait;
use chrono::Utc;
use chronos_core::error::{ChronosError, Result};
use chronos_core::models::{CaptureConfig, Frame};
use chronos_core::traits::ImageCapture;
use std::io::Cursor;
use tokio::sync::mpsc;
use tokio::sync::watch;
use ulid::Ulid;
use xcap::Monitor;

/// X11-based screen capture implementation.
///
/// **Go Parallel (Didactic):** In Go, calling CGO or blocking OS APIs
/// implicitly schedules work on one of the OS threads managed by the runtime.
/// If that thread blocks, Go might spawn another OS thread. In Rust's Tokio,
/// blocking the executor starves the entire async runtime. Thus, interacting
/// with X11 window managers demands explicit thread isolation either via
/// `tokio::task::spawn_blocking` (for one-offs) or `std::thread::spawn`
/// (for an infinite loop).
pub struct X11Capture {
    pub config: CaptureConfig,
}

impl X11Capture {
    /// Creates a new capture instance with the given configuration.
    pub fn new(config: CaptureConfig) -> Self {
        Self { config }
    }

    /// Spawns a dedicated OS thread to capture the screen at the configured interval.
    /// This is the preferred way to run screen capture continuously without
    /// blocking the async executor.
    ///
    /// The `shutdown_rx` channel allows graceful termination.
    pub fn start_capture_loop(
        &self,
        tx: mpsc::Sender<Frame>,
        shutdown_rx: watch::Receiver<bool>,
    ) -> std::thread::JoinHandle<()> {
        let config = self.config.clone();

        // Spawn a dedicated OS thread (NOT a Tokio task) for blocking X11 IO.
        std::thread::spawn(move || {
            let interval = std::time::Duration::from_secs(config.interval_seconds);

            loop {
                // Check if we should shut down (non-blocking lookup on the watch channel)
                if *shutdown_rx.borrow() {
                    break;
                }

                if let Ok(monitors) = Monitor::all()
                    && let Some(primary) = monitors.first()
                    && let Ok(image) = primary.capture_image()
                {
                    let width = image.width();
                    let height = image.height();

                    // Encode to PNG purely in RAM (no SSD wear/tear - Architecture Rule)
                    let mut buffer = Vec::new();
                    if image
                        .write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)
                        .is_ok()
                    {
                        let frame = Frame {
                            id: Ulid::new(),
                            timestamp: Utc::now(),
                            image_data: buffer,
                            width,
                            height,
                        };

                        // Send the frame to the async world.
                        // Provide back-pressure by handling blocked/closed channels.
                        if tx.blocking_send(frame).is_err() {
                            break;
                        }
                    }
                }

                // Sleep for the defined interval
                std::thread::sleep(interval);
            }
        })
    }
}

#[async_trait]
impl ImageCapture for X11Capture {
    /// Captures a single frame synchronously on demand, but delegates to `spawn_blocking`
    /// to avoid locking the Tokio executor during the X11 call.
    async fn capture_frame(&self) -> Result<Frame> {
        let handle = tokio::task::spawn_blocking(move || -> Result<Frame> {
            let monitors = Monitor::all().map_err(|e| {
                ChronosError::Capture(format!("Failed to enumerate monitors: {}", e))
            })?;

            let primary = monitors
                .first()
                .ok_or_else(|| ChronosError::Capture("No monitors found".to_string()))?;

            let image = primary
                .capture_image()
                .map_err(|e| ChronosError::Capture(format!("Failed to capture image: {}", e)))?;

            let width = image.width();
            let height = image.height();

            // Encode to PNG bytes in RAM
            let mut buffer = Vec::new();
            image
                .write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)
                .map_err(|e| ChronosError::Capture(format!("Failed to encode PNG: {}", e)))?;

            Ok(Frame {
                id: Ulid::new(),
                timestamp: Utc::now(),
                image_data: buffer,
                width,
                height,
            })
        });

        match handle.await {
            Ok(inner_result) => inner_result,
            Err(e) => Err(ChronosError::Capture(format!(
                "Blocking task panicked: {}",
                e
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x11_capture_creation() {
        let config = CaptureConfig::default();
        let capture = X11Capture::new(config.clone());

        // Didactic note: in tests without hardware boundaries,
        // asserting simple configurations isolates logic from external dependencies.
        assert_eq!(capture.config.interval_seconds, config.interval_seconds);
        assert_eq!(
            capture.config.ring_buffer_capacity,
            config.ring_buffer_capacity
        );
    }
}
