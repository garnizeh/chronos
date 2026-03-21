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

/// Trait to abstract the underlying platform-specific screen capture.
/// This allows mocking the OS interaction for 100% test coverage.
pub trait CaptureSource: Send + Sync + 'static {
    /// Captures the primary monitor's current state as an RGBA image.
    fn capture_primary(&self) -> Result<image::RgbaImage>;
}

/// The production implementation using the `xcap` crate.
pub struct XcapSource;

impl CaptureSource for XcapSource {
    fn capture_primary(&self) -> Result<image::RgbaImage> {
        let monitors = Monitor::all().map_err(|e| {
            ChronosError::Capture(format!("Failed to enumerate monitors: {}", e))
        })?;

        let primary = monitors
            .into_iter()
            .find(|m| m.is_primary().unwrap_or(false))
            .ok_or_else(|| ChronosError::Capture("No primary monitor detected".to_string()))?;

        primary
            .capture_image()
            .map_err(|e| ChronosError::Capture(format!("Failed to capture image: {}", e)))
    }
}

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
    config: CaptureConfig,
    source: std::sync::Arc<dyn CaptureSource>,
}

impl X11Capture {
    /// Creates a new capture instance with the given configuration.
    pub fn new(config: CaptureConfig) -> Self {
        Self {
            config,
            source: std::sync::Arc::new(XcapSource),
        }
    }

    /// Creates a new capture instance with a custom image source (mainly for testing).
    pub fn with_source(config: CaptureConfig, source: std::sync::Arc<dyn CaptureSource>) -> Self {
        Self { config, source }
    }

    /// Returns a reference to the capture configuration.
    pub fn config(&self) -> &CaptureConfig {
        &self.config
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

        let source = self.source.clone();

        // Spawn a dedicated OS thread (NOT a Tokio task) for blocking X11 IO.
        std::thread::spawn(move || {
            let interval = std::time::Duration::from_secs(config.interval_seconds);

            loop {
                // Check if we should shut down (non-blocking lookup on the watch channel)
                if *shutdown_rx.borrow() {
                    break;
                }

                match source.capture_primary() {
                    Ok(image) => match Self::encode_image_to_frame(image) {
                        Ok(frame) => {
                            // Send the frame to the async world. 
                            // Provide back-pressure by handling blocked/closed channels.
                            if let Err(e) = tx.blocking_send(frame) {
                                tracing::error!("Failed to send frame (receiver likely closed): {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to encode captured image: {}", e);
                        }
                    },
                    Err(e) => {
                        tracing::error!("Screen capture loop error: {}", e);
                    }
                }

                // Sleep for the defined interval
                std::thread::sleep(interval);
            }
        })
    }


    /// Encodes a raw image buffer into a structured PNG `Frame`.
    /// This is an internal helper to centralize the encoding logic and
    /// allow unit testing without a live screen capture environment.
    fn encode_image_to_frame(image: image::RgbaImage) -> Result<Frame> {
        let width = image.width();
        let height = image.height();

        // Encode to PNG purely in RAM (no SSD wear/tear - Architecture Rule)
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
    }
}

#[async_trait]
impl ImageCapture for X11Capture {
    /// Captures a single frame synchronously on demand, but delegates to `spawn_blocking`
    /// to avoid locking the Tokio executor during the X11 call.
    async fn capture_frame(&self) -> Result<Frame> {
        let source = self.source.clone();
        let handle = tokio::task::spawn_blocking(move || -> Result<Frame> {
            let image = source.capture_primary()?;
            Self::encode_image_to_frame(image)
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
    use image::{Rgba, RgbaImage};
    use std::sync::{Arc, Mutex};

    struct MockSource {
        result: Mutex<Result<RgbaImage>>,
    }

    impl CaptureSource for MockSource {
        fn capture_primary(&self) -> Result<RgbaImage> {
            let res = self.result.lock().unwrap();
            match &*res {
                Ok(_) => Ok(RgbaImage::new(2, 2)),
                Err(e) => Err(ChronosError::Capture(e.to_string())),
            }
        }
    }

    #[test]
    fn test_x11_capture_creation() {
        let config = CaptureConfig::default();
        let capture = X11Capture::new(config.clone());
        assert_eq!(capture.config().interval_seconds, config.interval_seconds);
    }

    #[tokio::test]
    async fn test_capture_frame_success() {
        let config = CaptureConfig::default();
        let mock = Arc::new(MockSource {
            result: Mutex::new(Ok(RgbaImage::new(2, 2))),
        });
        let capture = X11Capture::with_source(config, mock);

        let result = capture.capture_frame().await;
        assert!(result.is_ok());
        let frame = result.unwrap();
        assert_eq!(frame.width, 2);
    }

    #[tokio::test]
    async fn test_capture_frame_failure() {
        let config = CaptureConfig::default();
        let mock = Arc::new(MockSource {
            result: Mutex::new(Err(ChronosError::Capture("OS Error".into()))),
        });
        let capture = X11Capture::with_source(config, mock);

        let result = capture.capture_frame().await;
        assert!(result.is_err());
    }

    #[test]
    fn test_start_capture_loop_shutdown() {
        let config = CaptureConfig {
            interval_seconds: 1,
            ..Default::default()
        };
        let mock = Arc::new(MockSource {
            result: Mutex::new(Ok(RgbaImage::new(2, 2))),
        });
        let capture = X11Capture::with_source(config, mock);

        let (tx, _rx) = mpsc::channel(10);
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let handle = capture.start_capture_loop(tx, shutdown_rx);

        // Immediate shutdown
        shutdown_tx.send(true).unwrap();
        handle.join().unwrap();
    }

    #[test]
    fn test_start_capture_loop_success() {
        let config = CaptureConfig {
            interval_seconds: 0,
            ..Default::default()
        };
        let mock = Arc::new(MockSource {
            result: Mutex::new(Ok(RgbaImage::new(2, 2))),
        });
        let capture = X11Capture::with_source(config, mock);

        let (tx, mut rx) = mpsc::channel(10);
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let _handle = capture.start_capture_loop(tx, shutdown_rx);

        // Wait for at least one frame
        let frame = rx.blocking_recv().unwrap();
        assert_eq!(frame.width, 2);

        shutdown_tx.send(true).unwrap();
    }

    #[test]
    fn test_start_capture_loop_send_failure() {
        let config = CaptureConfig {
            interval_seconds: 0,
            ..Default::default()
        };
        let mock = Arc::new(MockSource {
            result: Mutex::new(Ok(RgbaImage::new(2, 2))),
        });
        let capture = X11Capture::with_source(config, mock);

        let (tx, rx) = mpsc::channel(1); // Small buffer
        drop(rx); // Close receiver immediately

        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let handle = capture.start_capture_loop(tx, shutdown_rx);
        
        // The loop should break upon failing to send
        handle.join().unwrap();
        shutdown_tx.send(true).ok(); // Clean up just in case
    }

    #[test]
    fn test_start_capture_loop_error() {
        let config = CaptureConfig {
            interval_seconds: 0, // No delay for fast test
            ..Default::default()
        };
        let mock = Arc::new(MockSource {
            result: Mutex::new(Err(ChronosError::Capture("Mock Error".into()))),
        });
        let capture = X11Capture::with_source(config, mock);

        let (tx, _rx) = mpsc::channel(10);
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let _handle = capture.start_capture_loop(tx, shutdown_rx);
        
        // Wait a bit for the loop to execute the error path
        std::thread::sleep(std::time::Duration::from_millis(10));
        shutdown_tx.send(true).unwrap();
    }

    #[test]
    fn test_encode_image_to_frame() {
        let mut img = RgbaImage::new(2, 2);
        img.put_pixel(0, 0, Rgba([255, 0, 0, 255]));

        let result = X11Capture::encode_image_to_frame(img);
        assert!(result.is_ok());

        let frame = result.unwrap();
        assert_eq!(frame.width, 2);
        assert_eq!(frame.height, 2);
        assert_eq!(&frame.image_data[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }
}
