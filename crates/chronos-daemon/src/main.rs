use chronos_capture::x11::X11Capture;
use chronos_core::models::{CaptureConfig, VlmConfig};
use chronos_daemon::{
    cli::{Cli, Commands},
    database::Database,
    handlers::{handle_query, handle_status},
    pipeline::CaptureEngine,
};
use chronos_inference::ollama::OllamaVision;
use clap::Parser;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging (using try_init to avoid panicking in tests where a subscriber is already set)
    let _ = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .try_init();

    let cli = Cli::parse();
    run_app(cli).await
}

/// Core application router.
///
/// **Go Parallel (Didactic):** This is the equivalent of a `Serve()` or `Run()`
/// function that takes a configuration (the parsed CLI) and dispatches it.
/// Separating this from `main()` allows us to unit test the routing logic.
pub async fn run_app(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Start => handle_start(cli.db_url).await?,
        Commands::Query { from, to, limit } => {
            let url = match cli.db_url {
                Some(url) => url,
                None => chronos_daemon::handlers::get_default_db_url()?,
            };
            let db = Database::new(&url).await?;
            handle_query(&db, from, to, limit).await?
        }
        Commands::Status => {
            let url = match cli.db_url {
                Some(url) => url,
                None => chronos_daemon::handlers::get_default_db_url()?,
            };
            let db = Database::new(&url).await?;
            handle_status(&db, &url).await?
        }
        Commands::Pause => handle_pause(),
        Commands::Resume => handle_resume(),
    }

    Ok(())
}

/// Entry point for the persistent capture daemon.
///
/// **Go Parallel:** This wires up the "main loop" of your application,
/// similar to initializing your service dependencies and starting a server.
async fn handle_start(db_url_override: Option<String>) -> anyhow::Result<()> {
    info!("Starting Chronos Daemon v{}", env!("CARGO_PKG_VERSION"));

    // 1. Initialize Components
    let db_url = match db_url_override {
        Some(url) => url,
        None => get_database_url()?,
    };
    info!("Connecting to database: {db_url}");
    let db = Database::new(&db_url).await?;

    let capture = X11Capture::new(CaptureConfig::default());
    let vision = OllamaVision::new(VlmConfig::default())?;

    // 2. Run Orchestrator
    info!("Pipeline active. Press Ctrl+C to stop.");
    run_orchestrator(vision, capture, db).await
}

/// Decoupled orchestration logic for the capture daemon.
///
/// **Go Parallel:** This is the equivalent of a `StartServer(deps)` function
/// in Go that wires up the dependencies and enters the main loop.
///
/// # Errors
/// Returns an error if the pipeline fails or the capture loop is interrupted unexpectedly.
pub async fn run_orchestrator<V, C>(vision: V, capture: C, db: Database) -> anyhow::Result<()>
where
    V: chronos_core::traits::VisionInference + Send + Sync + 'static,
    C: chronos_core::traits::ImageCapture + Send + Sync + 'static,
{
    // Create Orchestrator
    let engine = CaptureEngine::new(vision, db);

    // Wire the pipeline
    let (tx, rx) = tokio::sync::mpsc::channel(64);

    // Spawn capture thread
    let capture_handle = tokio::spawn(async move {
        // We use the configured interval from the capture implementation.
        // Note: We call capture.capture_frame() manually here instead of start_capture_loop()
        // because we are already inside a managed async orchestrator and want to
        // maintain direct control over the pipeline wiring.
        // Guard against zero duration to prevent busy-looping if configured incorrectly.
        let interval_secs = std::cmp::max(capture.capture_interval_seconds(), 1);
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(interval_secs));
        loop {
            interval.tick().await;
            match capture.capture_frame().await {
                Ok(frame) => {
                    if tx.send(frame).await.is_err() {
                        break;
                    }
                }
                Err(e) => {
                    tracing::error!("failed to capture frame: {e}");
                    // Transient error or display temporarily locked; continue searching for frames
                }
            }
        }
    });

    // Run the pipeline (this blocks until RX is closed or Ctrl+C)
    // We use a select! to handle graceful shutdown signals.
    let mut pipeline_handle = tokio::spawn(async move { engine.run_pipeline(rx).await });

    tokio::select! {
        res = &mut pipeline_handle => {
            // Pipeline finished on its own (rx closed)
            res??;
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Shutdown signal received, closing capture loop...");
            capture_handle.abort();
            // Await the pipeline to finish processing remaining frames in the queue
            pipeline_handle.await??;
            info!("Pipeline drained. Shutdown complete.");
        }
    }

    Ok(())
}

fn get_database_url() -> anyhow::Result<String> {
    chronos_daemon::handlers::get_default_db_url()
}

fn handle_pause() {
    println!("Pause command not yet implemented in v0.1. Full IPC coming in v0.2.");
}

fn handle_resume() {
    println!("Resume command not yet implemented in v0.1. Full IPC coming in v0.2.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_app_routing() {
        // Test Pause/Resume routing
        assert!(
            run_app(Cli {
                db_url: None,
                command: Commands::Pause
            })
            .await
            .is_ok()
        );
        assert!(
            run_app(Cli {
                db_url: None,
                command: Commands::Resume
            })
            .await
            .is_ok()
        );

        // Test Status routing (uses a dummy DB URL that might fail if data dir isn't writable,
        // but we can at least verify it reaches the DB init logic).
        // For a more robust test, we could further decouple the DB creation.
        // Test Status routing with hermetic DB
        let cli = Cli {
            db_url: Some("sqlite::memory:".to_string()),
            command: Commands::Status,
        };
        assert!(run_app(cli).await.is_ok());

        // Test Query routing with hermetic DB
        let cli = Cli {
            db_url: Some("sqlite::memory:".to_string()),
            command: Commands::Query {
                from: None,
                to: None,
                limit: 10,
            },
        };
        assert!(run_app(cli).await.is_ok());
    }

    #[test]
    fn test_get_database_url() {
        let url = get_database_url().unwrap();
        assert!(url.starts_with("sqlite://"));
        assert!(url.contains("chronos.db"));
    }

    #[tokio::test]
    async fn test_run_orchestrator_wiring() {
        use chronos_core::traits::mocks::{MockCapture, MockVision};

        let db = Database::new_in_memory().await.unwrap();

        // Pause time AFTER DB initialization to avoid connection pool timeouts
        tokio::time::pause();

        let capture = MockCapture;
        let vision = MockVision;

        // Run orchestrator with mocks.
        // We use explicit types to help the compiler infer the Send/Sync requirements.
        let handle = tokio::spawn(async move {
            let _ = run_orchestrator::<MockVision, MockCapture>(vision, capture, db).await;
        });

        // Advance time to trigger the first capture tick immediately in virtual time
        tokio::time::advance(std::time::Duration::from_millis(500)).await;

        handle.abort();
    }

    #[tokio::test]
    async fn test_run_orchestrator_capture_error() {
        use chronos_core::traits::ImageCapture;
        use chronos_core::traits::mocks::MockVision;

        struct MockCaptureError;
        #[async_trait::async_trait]
        impl ImageCapture for MockCaptureError {
            async fn capture_frame(
                &self,
            ) -> chronos_core::error::Result<chronos_core::models::Frame> {
                Err(chronos_core::error::ChronosError::Capture(
                    "Mock failure".to_string(),
                ))
            }

            // [JUSTIFIED GAP]: Default implementation used.
        }

        let db = Database::new_in_memory().await.unwrap();
        let capture = MockCaptureError;
        let vision = MockVision;

        let handle = tokio::spawn(async move {
            let _ = run_orchestrator::<MockVision, MockCaptureError>(vision, capture, db).await;
        });

        // Give it a moment to run and hit the error log
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        handle.abort();
    }
}
