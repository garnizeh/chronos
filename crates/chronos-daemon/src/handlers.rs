use crate::database::Database;
use chrono::{DateTime, TimeZone, Utc};
use tracing::info;

// [JUSTIFIED GAP]: Involves OS-specific directory resolution (`dirs`) and filesystem side-effects (`create_dir_all`).
// Refactoring to a trait was considered but deferred to v0.2 to avoid premature abstraction for a simple path resolve.
pub fn get_default_db_url() -> anyhow::Result<String> {
    let mut path = dirs::data_local_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find local data directory"))?;
    path.push("chronos");
    std::fs::create_dir_all(&path)?;
    path.push("chronos.db");
    Ok(format!("sqlite://{}", path.to_string_lossy()))
}

/// Handle the 'query' command.
///
/// **Go Parallel:** This is like a Cobra subcommand Run function.
pub async fn handle_query(
    db: &Database,
    from: Option<String>,
    to: Option<String>,
    limit: u64,
) -> anyhow::Result<()> {
    let logs = if from.is_some() || to.is_some() {
        // Parse dates. For v0.1, we expect RFC3339 or simple YYYY-MM-DD.
        let from_dt = from
            .as_deref()
            .map(parse_date)
            .transpose()?
            .unwrap_or_else(|| Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap());

        let to_dt = to
            .as_deref()
            .map(parse_date)
            .transpose()?
            .unwrap_or_else(Utc::now);

        info!(
            "Querying logs from {} to {} (limit: {})",
            from_dt, to_dt, limit
        );
        db.get_logs_by_date_range(from_dt, to_dt, limit).await?
    } else {
        info!("Querying {} most recent logs", limit);
        db.get_recent_logs(limit).await?
    };

    if logs.is_empty() {
        println!("No logs found.");
    } else {
        println!("{:<25} | {:<15} | Description", "Timestamp", "Application");
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
pub async fn handle_status(db: &Database, url: &str) -> anyhow::Result<()> {
    let count = db.get_log_count().await?;

    println!("Chronos System Status:");
    println!("  Database: {}", url);
    println!("  Total Semantic Logs: {}", count);
    println!("  Version: v{}", env!("CARGO_PKG_VERSION"));

    Ok(())
}

/// Simple date parser for the CLI.
fn parse_date(s: &str) -> anyhow::Result<DateTime<Utc>> {
    // Try RFC3339 first
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&Utc));
    }

    // Try YYYY-MM-DD
    if let Some(dt) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .ok()
        .and_then(|naive| naive.and_hms_opt(0, 0, 0))
        .and_then(|hms| Utc.from_local_datetime(&hms).single())
    {
        return Ok(dt);
    }

    anyhow::bail!("Invalid date format: {}. Use YYYY-MM-DD or RFC3339.", s)
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let end = max.saturating_sub(3);
    // Find the largest valid byte boundary <= end
    let boundary = s
        .char_indices()
        .map(|(i, _)| i)
        .take_while(|&i| i <= end)
        .last()
        .unwrap_or(0);

    format!("{}...", &s[..boundary])
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_parse_date_rfc3339() {
        let s = "2023-10-27T10:00:00Z";
        let res = parse_date(s);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().year(), 2023);
    }

    #[test]
    fn test_parse_date_simple() {
        let s = "2023-10-27";
        let res = parse_date(s);
        assert!(res.is_ok());
        let dt = res.unwrap();
        assert_eq!(dt.year(), 2023);
        assert_eq!(dt.month(), 10);
        assert_eq!(dt.day(), 27);
    }

    #[test]
    fn test_parse_date_invalid() {
        let s = "invalid-date";
        let res = parse_date(s);
        assert!(res.is_err());
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello world", 5), "he...");
        assert_eq!(truncate("exactly", 7), "exactly");
        // UTF-8 safe truncation: 🦀 is 4 bytes.
        // max 7 -> end 4. boundary at 4. "🦀..."
        assert_eq!(truncate("🦀🦀🦀", 7), "🦀...");
        // max 6 -> end 3. boundary at 0. "..."
        assert_eq!(truncate("🦀🦀🦀", 6), "...");
    }

    #[tokio::test]
    async fn test_handle_status_smoke() {
        let db = Database::new_in_memory().await.unwrap();
        let res = handle_status(&db, "sqlite::memory:").await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_handle_query_smoke() {
        let db = Database::new_in_memory().await.unwrap();
        // Test most recent logs path
        let res = handle_query(&db, None, None, 10).await;
        assert!(res.is_ok());

        // Test date range path (with valid but empty range)
        let res = handle_query(
            &db,
            Some("2023-01-01".to_string()),
            Some("2023-12-31".to_string()),
            10,
        )
        .await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_handle_query_with_data() {
        let db = Database::new_in_memory().await.unwrap();
        // Insert a dummy log
        db.insert_semantic_log(&chronos_core::models::SemanticLog {
            id: ulid::Ulid::new(),
            source_frame_id: ulid::Ulid::new(),
            timestamp: Utc::now(),
            description: "Test log".to_string(),
            active_application: Some("TestApp".to_string()),
            activity_category: Some("Testing".to_string()),
            key_entities: vec!["Unit Test".to_string()],
            confidence_score: 0.95,
            raw_vlm_response: "{}".to_string(),
        })
        .await
        .unwrap();

        // Testing the successful branch that prints the table
        let res = handle_query(&db, None, None, 10).await;
        assert!(res.is_ok());
    }
}
