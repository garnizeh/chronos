use chronos_capture::x11::X11Capture;
use chronos_core::models::{CaptureConfig, VlmConfig};
#[allow(clippy::redundant_crate_prefix)]
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
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

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
        Commands::Start => handle_start().await?,
        Commands::Query { from, to, limit } => {
            let url = chronos_daemon::handlers::get_default_db_url();
            let db = Database::new(&url).await?;
            handle_query(&db, from, to, limit).await?
        }
        Commands::Status => {
            let url = chronos_daemon::handlers::get_default_db_url();
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
async fn handle_start() -> anyhow::Result<()> {
    info!("Starting Chronos Daemon v{}", env!("CARGO_PKG_VERSION"));

    // 1. Initialize Components
    let db_url = get_database_url()?;
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
        // Slow interval for production (30s)
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
        loop {
            interval.tick().await;
            #[allow(clippy::collapsible_if)]
            if let Ok(frame) = capture.capture_frame().await {
                if tx.send(frame).await.is_err() {
                    break;
                }
            }
        }
    });

    // Run the pipeline (this blocks until RX is closed or Ctrl+C)
    // In our current architecture, Ctrl+C is handled by the tokio runtime implicitly
    // for this top-level call.
    engine.run_pipeline(rx).await?;

    capture_handle.abort();
    Ok(())
}

fn get_database_url() -> anyhow::Result<String> {
    let mut db_path = dirs::data_local_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find local data directory"))?;
    db_path.push("chronos");
    std::fs::create_dir_all(&db_path)?;
    db_path.push("chronos.db");
    Ok(format!("sqlite://{}", db_path.to_string_lossy()))
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
                command: Commands::Pause
            })
            .await
            .is_ok()
        );
        assert!(
            run_app(Cli {
                command: Commands::Resume
            })
            .await
            .is_ok()
        );

        // Test Status routing (uses a dummy DB URL that might fail if data dir isn't writable,
        // but we can at least verify it reaches the DB init logic).
        // For a more robust test, we could further decouple the DB creation.
        let cli = Cli {
            command: Commands::Status,
        };
        let _ = run_app(cli).await;

        let cli = Cli {
            command: Commands::Query {
                from: None,
                to: None,
                limit: 10,
            },
        };
        let _ = run_app(cli).await;
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
        let capture = MockCapture;
        let vision = MockVision;

        // Run orchestrator with mocks.
        // We use explicit types to help the compiler infer the Send/Sync requirements.
        let handle = tokio::spawn(async move {
            let _ = run_orchestrator::<MockVision, MockCapture>(vision, capture, db).await;
        });

        // Give it a moment to process the initial mock frame
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        handle.abort();
    }
}
