use chrono::{DateTime, Utc};
use chronos_core::models::SemanticLog;
use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

/// The Database struct encapsulates the SQLite connection pool.
/// In Go, this would be equivalent to a struct holding a `*sql.DB`.
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Create a new Database instance connecting to the given URL.
    /// This also runs any pending migrations automatically.
    pub async fn new(url: &str) -> Result<Self, chronos_core::error::ChronosError> {
        let pool = SqlitePoolOptions::new()
            // TODO(provisional): Using 5 connections for SQLite.
            // Rationale: Keeps resource footprint low for a background daemon while allowing concurrent CLI queries.
            // Trigger: Scaling to 10+ concurrent dashboard users or heavy background analytics.
            // Direction: Move to a config-based pool size or dynamic adjustment.
            .max_connections(5)
            .connect(url)
            .await
            .map_err(|e: sqlx::Error| chronos_core::error::ChronosError::Database(e.to_string()))?;

        // Run migrations. Path is relative to the crate root.
        sqlx::migrate!("../../migrations")
            .run(&pool)
            .await
            .map_err(|e: sqlx::migrate::MigrateError| {
                chronos_core::error::ChronosError::Database(e.to_string())
            })?;

        Ok(Self { pool })
    }

    /// Create an in-memory database for testing purposes.
    /// Useful for isolated, fast integration tests.
    /// Using a shared cache ensure all pool connections see the same data.
    pub async fn new_in_memory() -> Result<Self, chronos_core::error::ChronosError> {
        // sqlx uses ":memory:" for in-memory SQLite, but shared cache is better for pools
        Self::new("sqlite:file::memory:?mode=memory&cache=shared").await
    }

    /// Insert a new SemanticLog into the database.
    pub async fn insert_semantic_log(
        &self,
        log: &SemanticLog,
    ) -> Result<(), chronos_core::error::ChronosError> {
        let key_entities_json =
            serde_json::to_string(&log.key_entities).map_err(|e: serde_json::Error| {
                chronos_core::error::ChronosError::Database(e.to_string())
            })?;

        sqlx::query(
            "INSERT INTO semantic_logs (
                id, timestamp, source_frame_id, description, 
                active_application, activity_category, key_entities, 
                confidence_score, raw_vlm_response
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(log.id.to_string())
        .bind(log.timestamp.to_rfc3339())
        .bind(log.source_frame_id.to_string())
        .bind(&log.description)
        .bind(&log.active_application)
        .bind(&log.activity_category)
        .bind(key_entities_json)
        .bind(log.confidence_score)
        .bind(&log.raw_vlm_response)
        .execute(&self.pool)
        .await
        .map_err(|e: sqlx::Error| chronos_core::error::ChronosError::Database(e.to_string()))?;

        Ok(())
    }

    /// Retrieve logs within a specific date range.
    pub async fn get_logs_by_date_range(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<SemanticLog>, chronos_core::error::ChronosError> {
        let rows = sqlx::query_as::<_, SemanticLogRow>(
            "SELECT * FROM semantic_logs WHERE timestamp BETWEEN ? AND ? ORDER BY timestamp ASC",
        )
        .bind(from.to_rfc3339())
        .bind(to.to_rfc3339())
        .fetch_all(&self.pool)
        .await
        .map_err(|e: sqlx::Error| chronos_core::error::ChronosError::Database(e.to_string()))?;

        let mut logs = Vec::new();
        for row in rows {
            logs.push(row.try_into()?);
        }
        Ok(logs)
    }

    /// Get the total count of logs in the database.
    pub async fn get_log_count(&self) -> Result<i64, chronos_core::error::ChronosError> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM semantic_logs")
            .fetch_one(&self.pool)
            .await
            .map_err(|e: sqlx::Error| chronos_core::error::ChronosError::Database(e.to_string()))?;

        Ok(count.0)
    }

    /// Get the most recent logs, up to the specified limit.
    pub async fn get_recent_logs(
        &self,
        limit: i64,
    ) -> Result<Vec<SemanticLog>, chronos_core::error::ChronosError> {
        let rows = sqlx::query_as::<_, SemanticLogRow>(
            "SELECT * FROM semantic_logs ORDER BY timestamp DESC LIMIT ?",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e: sqlx::Error| chronos_core::error::ChronosError::Database(e.to_string()))?;

        let mut logs = Vec::new();
        for row in rows {
            logs.push(row.try_into()?);
        }
        Ok(logs)
    }
}

/// Internal helper struct to map SQL rows to domain models.
/// sqlx's `FromRow` needs specific types that match the DB schema.
#[derive(sqlx::FromRow)]
struct SemanticLogRow {
    id: String,
    timestamp: String,
    source_frame_id: String,
    description: String,
    active_application: Option<String>,
    activity_category: Option<String>,
    key_entities: String,
    confidence_score: f64,
    raw_vlm_response: String,
}

impl TryFrom<SemanticLogRow> for SemanticLog {
    type Error = chronos_core::error::ChronosError;

    fn try_from(row: SemanticLogRow) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            id: row.id.parse().map_err(|e| {
                chronos_core::error::ChronosError::Database(format!("Invalid ID: {}", e))
            })?,
            timestamp: DateTime::parse_from_rfc3339(&row.timestamp)
                .map_err(|e| {
                    chronos_core::error::ChronosError::Database(format!("Invalid timestamp: {}", e))
                })?
                .with_timezone(&Utc),
            source_frame_id: row.source_frame_id.parse().map_err(|e| {
                chronos_core::error::ChronosError::Database(format!(
                    "Invalid source_frame_id: {}",
                    e
                ))
            })?,
            description: row.description,
            active_application: row.active_application,
            activity_category: row.activity_category,
            key_entities: serde_json::from_str(&row.key_entities).map_err(|e| {
                chronos_core::error::ChronosError::Database(format!("Invalid key_entities: {}", e))
            })?,
            confidence_score: row.confidence_score,
            raw_vlm_response: row.raw_vlm_response,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chronos_core::models::SemanticLog;
    use ulid::Ulid;

    #[tokio::test]
    async fn test_new_in_memory() {
        let db = Database::new_in_memory().await;
        assert!(
            db.is_ok(),
            "Should be able to create an in-memory database: {:?}",
            db.err()
        );
    }

    #[tokio::test]
    async fn test_insert_and_query_round_trip() {
        let db = Database::new_in_memory().await.unwrap();
        let log = SemanticLog {
            id: Ulid::new(),
            timestamp: Utc::now(),
            source_frame_id: Ulid::new(),
            description: "Test description".to_string(),
            active_application: Some("Test App".to_string()),
            activity_category: Some("Test Category".to_string()),
            key_entities: vec!["Entity1".to_string(), "Entity2".to_string()],
            confidence_score: 0.9,
            raw_vlm_response: "{}".to_string(),
        };

        db.insert_semantic_log(&log).await.unwrap();

        let count = db.get_log_count().await.unwrap();
        assert_eq!(count, 1);

        let recent = db.get_recent_logs(1).await.unwrap();
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].id, log.id);
        assert_eq!(recent[0].description, log.description);
        assert_eq!(recent[0].key_entities, log.key_entities);
    }

    #[tokio::test]
    async fn test_get_logs_by_date_range() {
        let db = Database::new_in_memory().await.unwrap();
        let now = Utc::now();

        let log1 = SemanticLog {
            id: Ulid::new(),
            timestamp: now - chrono::Duration::try_minutes(10).unwrap(),
            source_frame_id: Ulid::new(),
            description: "Old log".to_string(),
            active_application: None,
            activity_category: None,
            key_entities: vec![],
            confidence_score: 1.0,
            raw_vlm_response: "".to_string(),
        };

        let log2 = SemanticLog {
            id: Ulid::new(),
            timestamp: now,
            source_frame_id: Ulid::new(),
            description: "New log".to_string(),
            active_application: None,
            activity_category: None,
            key_entities: vec![],
            confidence_score: 1.0,
            raw_vlm_response: "".to_string(),
        };

        db.insert_semantic_log(&log1).await.unwrap();
        db.insert_semantic_log(&log2).await.unwrap();

        let range_logs = db
            .get_logs_by_date_range(
                now - chrono::Duration::try_minutes(5).unwrap(),
                now + chrono::Duration::try_minutes(5).unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(range_logs.len(), 1);
        assert_eq!(range_logs[0].id, log2.id);
    }
}
