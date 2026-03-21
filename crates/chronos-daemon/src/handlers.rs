use crate::database::Database;
use chrono::{DateTime, Utc, TimeZone};
use tracing::info;
use std::path::PathBuf;

/// Helper to get the default database URL.
/// For v0.1, we'll store it in the user's local data directory.
fn get_db_url() -> String {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("chronos");
    std::fs::create_dir_all(&path).ok();
    path.push("chronos.db");
    format!("sqlite://{}", path.to_string_lossy())
}

/// Handle the 'query' command.
///
/// **Go Parallel:** This is like a Cobra subcommand Run function.
pub async fn handle_query(from: Option<String>, to: Option<String>, limit: i64) -> anyhow::Result<()> {
    let url = get_db_url();
    let db = Database::new(&url).await?;

    let logs = if from.is_some() || to.is_some() {
        // Parse dates. For v0.1, we expect RFC3339 or simple YYYY-MM-DD.
        let from_dt = from
            .map(|s| parse_date(&s))
            .transpose()?
            .unwrap_or_else(|| Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap());
        
        let to_dt = to
            .map(|s| parse_date(&s))
            .transpose()?
            .unwrap_or_else(|| Utc::now());

        info!("Querying logs from {} to {} (limit: {})", from_dt, to_dt, limit);
        db.get_logs_by_date_range(from_dt, to_dt).await?
    } else {
        info!("Querying {} most recent logs", limit);
        db.get_recent_logs(limit).await?
    };

    if logs.is_empty() {
        println!("No logs found.");
    } else {
        println!("{:<25} | {:<15} | {}", "Timestamp", "Application", "Description");
        println!("{}", "-".repeat(80));
        for log in logs {
            let app = log.active_application.as_deref().unwrap_or("Unknown");
            println!(
                "{:<25} | {:<15} | {}",
                log.timestamp.to_rfc3339(),
                truncate(app, 15),
                log.description
            );
        }
    }

    Ok(())
}

/// Handle the 'status' command.
pub async fn handle_status() -> anyhow::Result<()> {
    let url = get_db_url();
    let db = Database::new(&url).await?;
    
    let count = db.get_log_count().await?;
    
    println!("Chronos System Status:");
    println!("  Database: {}", url);
    println!("  Total Semantic Logs: {}", count);
    println!("  Capture Daemon: Running (v{})", env!("CARGO_PKG_VERSION"));
    
    Ok(())
}

/// Simple date parser for the CLI.
fn parse_date(s: &str) -> anyhow::Result<DateTime<Utc>> {
    // Try RFC3339 first
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&Utc));
    }
    
    // Try YYYY-MM-DD
    if let Ok(naive) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        if let Some(dt) = naive.and_hms_opt(0, 0, 0).and_then(|n| Utc.from_local_datetime(&n).single()) {
            return Ok(dt);
        }
    }

    anyhow::bail!("Invalid date format: {}. Use YYYY-MM-DD or RFC3339.", s)
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max - 3])
    } else {
        s.to_string()
    }
}
