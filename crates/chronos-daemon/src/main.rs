use chronos_capture::x11::X11Capture;
use chronos_core::models::{CaptureConfig, VlmConfig};
use chronos_core::traits::ImageCapture;
use chronos_daemon::cli::{Cli, Commands};
use chronos_daemon::database::Database;
use chronos_daemon::handlers::{handle_query, handle_status};
use chronos_daemon::pipeline::CaptureEngine;
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

    match cli.command {
        Commands::Start => handle_start().await?,
        Commands::Query { from, to, limit } => handle_query(from, to, limit).await?,
        Commands::Status => handle_status().await?,
        Commands::Pause => handle_pause()?,
        Commands::Resume => handle_resume()?,
    }

    Ok(())
}

/// Entry point for the persistent capture daemon.
///
/// **Go Parallel:** This wires up the "main loop" of your application,
/// similar to initializing your service dependencies and starting a server.
async fn handle_start() -> anyhow::Result<()> {
    info!("Starting Chronos Daemon v{}", env!("CARGO_PKG_VERSION"));

    // 1. Initialize Database
    let mut db_path = dirs::data_local_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find local data directory"))?;
    db_path.push("chronos");
    std::fs::create_dir_all(&db_path)?;
    db_path.push("chronos.db");
    let db_url = format!("sqlite://{}", db_path.to_string_lossy());

    info!("Connecting to database: {}", db_url);
    let db = Database::new(&db_url).await?;

    // 2. Initialize Capture Engine (X11 for v0.1)
    let capture = X11Capture::new(CaptureConfig::default());

    // 3. Initialize Vision Inference (Ollama)
    let vision = OllamaVision::new(VlmConfig::default())?;

    // 4. Create Orchestrator
    let engine = CaptureEngine::new(vision, db);

    // 5. Wire the pipeline
    let (tx, rx) = tokio::sync::mpsc::channel(64);

    // Spawn capture thread (simulating a blocking producer for now, or just a loop)
    // In a real scenario, this would be a long-running loop calling `capture.capture_frame()`
    let capture_handle = tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
        loop {
            interval.tick().await;
            if let Ok(frame) = capture.capture_frame().await {
                let send_res = tx.send(frame).await;
                if let Err(e) = send_res {
                    tracing::error!("Failed to send frame to pipeline: {}", e);
                    break;
                }
            }
        }
    });

    // 6. Run the pipeline
    info!("Pipeline active. Press Ctrl+C to stop.");
    engine.run_pipeline(rx).await?;

    capture_handle.abort();
    Ok(())
}

fn handle_pause() -> anyhow::Result<()> {
    println!("Pause command not yet implemented in v0.1. Full IPC coming in v0.2.");
    Ok(())
}

fn handle_resume() -> anyhow::Result<()> {
    println!("Resume command not yet implemented in v0.1. Full IPC coming in v0.2.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stubs() {
        assert!(handle_pause().is_ok());
        assert!(handle_resume().is_ok());
    }
}
