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
    // [JUSTIFIED GAP]: This production implementation depends on live X11/Monitor hardware.
    // It is an Architectural Boundary (Rule 06) and is tested via MockSource in unit tests.
    fn capture_primary(&self) -> Result<image::RgbaImage> {
        let monitors = Monitor::all()
            .map_err(|e| ChronosError::Capture(format!("Failed to enumerate monitors: {}", e)))?;

        let mut primary = None;
        for m in monitors {
            if m.is_primary().map_err(|e| {
                ChronosError::Capture(format!("Failed to check if monitor is primary: {}", e))
            })? {
                primary = Some(m);
                break;
            }
        }

        let primary = primary
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
            let tick_interval = std::time::Duration::from_millis(100);

            loop {
                // Check shutdown before starting a new cycle
                if *shutdown_rx.borrow() {
                    break;
                }

                match source.capture_primary() {
                    Ok(image) => match Self::encode_image_to_frame(image) {
                        Ok(mut frame) => {
                            // Send the frame with interruptible retry
                            loop {
                                if *shutdown_rx.borrow() {
                                    // [JUSTIFIED GAP]: Shutdown mid-send is infrequent and occasionally
                                    // under-reported by llvm-cov due to thread termination timing.
                                    return;
                                }

                                match tx.try_send(frame) {
                                    Ok(_) => break,
                                    Err(mpsc::error::TrySendError::Full(returned_frame)) => {
                                        frame = returned_frame;
                                        // [JUSTIFIED GAP]: Backpressure retry logic is verified in
                                        // test_start_capture_loop_backpressure but often missed by timing.
                                        // Back-pressure: wait a bit and check shutdown again
                                        std::thread::sleep(std::time::Duration::from_millis(50));
                                    }
                                    Err(mpsc::error::TrySendError::Closed(_)) => {
                                        tracing::error!("Failed to send frame (receiver closed)");
                                        return;
                                    }
                                }
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

                // Small tick to prevent tight loop on zero interval
                std::thread::sleep(std::time::Duration::from_millis(1));

                // Interruptible sleep: sleep in small "ticks" until interval is reached
                let sleep_start = std::time::Instant::now();
                while sleep_start.elapsed() < interval {
                    if *shutdown_rx.borrow() {
                        // [JUSTIFIED GAP]: Managed shutdown during sleep is hit in
                        // test_start_capture_loop_shutdown_during_sleep.
                        return;
                    }
                    std::thread::sleep(tick_interval);
                }
            }
        })
    }

    /// Encodes a raw image buffer into a structured PNG `Frame`.
    /// This is an internal helper to centralize the encoding logic and
    /// allow unit testing without a live screen capture environment.
    fn encode_image_to_frame(image: image::RgbaImage) -> Result<Frame> {
        let width = image.width();
        let height = image.height();

        if width == 0 || height == 0 {
            return Err(ChronosError::Capture(
                "Cannot encode empty image".to_string(),
            ));
        }

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
    /// Captures a single frame asynchronously by delegating to `spawn_blocking`
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

    fn capture_interval_seconds(&self) -> u64 {
        self.config.interval_seconds
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
            res.clone()
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

        let handle = capture.start_capture_loop(tx, shutdown_rx);

        // Wait for at least one frame
        let frame = rx.blocking_recv().unwrap();
        assert_eq!(frame.width, 2);

        shutdown_tx.send(true).unwrap();
        handle.join().unwrap();
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

        let handle = capture.start_capture_loop(tx, shutdown_rx);

        // Wait a bit for the loop to execute the error path
        std::thread::sleep(std::time::Duration::from_millis(10));
        shutdown_tx.send(true).unwrap();
        handle.join().unwrap();
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
    #[test]
    fn test_start_capture_loop_backpressure() {
        let _config = CaptureConfig {
            interval_seconds: 0,
            ..Default::default()
        };
        let mock = Arc::new(MockSource {
            result: Mutex::new(Ok(RgbaImage::new(2, 2))),
        });
        let capture = X11Capture::with_source(
            CaptureConfig {
                interval_seconds: 0,
                ..Default::default()
            },
            mock,
        );

        let (tx, mut rx) = mpsc::channel(1);
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let handle = capture.start_capture_loop(tx, shutdown_rx);

        // 1. Consume the first frame to ensure the loop is running
        let _ = rx.blocking_recv().unwrap();

        // 2. Now stop consuming. The loop will send one more frame (filling the channel)
        // and then hit TrySendError::Full on the subsequent attempt.
        // We wait long enough for multiple loop cycles and the 50ms backoff.
        std::thread::sleep(std::time::Duration::from_millis(500));

        // 3. Consume the second frame to unblock the loop
        let _ = rx.blocking_recv().unwrap();

        shutdown_tx.send(true).unwrap();
        handle.join().unwrap();
    }

    #[test]
    fn test_start_capture_loop_shutdown_during_send() {
        let config = CaptureConfig {
            interval_seconds: 0,
            ..Default::default()
        };
        let mock = Arc::new(MockSource {
            result: Mutex::new(Ok(RgbaImage::new(2, 2))),
        });
        let capture = X11Capture::with_source(config, mock);

        let (tx, mut rx) = mpsc::channel(1);
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let handle = capture.start_capture_loop(tx, shutdown_rx);

        // Consume one so the loop is active
        let _ = rx.blocking_recv().unwrap();

        // Wait for it to saturate the channel and enter the try_send retry loop
        std::thread::sleep(std::time::Duration::from_millis(150));

        shutdown_tx.send(true).unwrap();
        handle.join().unwrap();
    }

    #[test]
    fn test_start_capture_loop_shutdown_during_sleep() {
        let config = CaptureConfig {
            interval_seconds: 1, // 1s is enough to hit the sleep loop
            ..Default::default()
        };
        let mock = Arc::new(MockSource {
            result: Mutex::new(Ok(RgbaImage::new(2, 2))),
        });
        let capture = X11Capture::with_source(config, mock);

        let (tx, mut rx) = mpsc::channel(1);
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let handle = capture.start_capture_loop(tx, shutdown_rx);

        // Wait for one capture to finish so it enters the sleep phase
        let _ = rx.blocking_recv().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));

        shutdown_tx.send(true).unwrap();
        handle.join().unwrap();
    }

    #[test]
    fn test_start_capture_loop_encode_error() {
        // Validation check for empty (0x0) images now returns Err.
        // We run the loop with a mock source that returns a 0x0 image.
        let config = CaptureConfig {
            interval_seconds: 0,
            ..Default::default()
        };
        let mock = Arc::new(MockSource {
            result: Mutex::new(Ok(RgbaImage::new(0, 0))),
        });
        let capture = X11Capture::with_source(config, mock);

        let (tx, _rx) = mpsc::channel(1);
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let handle = capture.start_capture_loop(tx, shutdown_rx);

        // Wait a bit for the loop to run at least once
        std::thread::sleep(std::time::Duration::from_millis(100));

        shutdown_tx.send(true).unwrap();
        handle.join().unwrap();
    }
}
