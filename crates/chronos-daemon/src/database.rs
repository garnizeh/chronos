use chrono::{DateTime, Utc};
use chronos_core::models::SemanticLog;
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;

/// The Database struct encapsulates the SQLite connection pool.
/// In Go, this would be equivalent to a struct holding a `*sql.DB`.
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Create a new Database instance connecting to the given URL.
    /// This also runs any pending migrations automatically.
    pub async fn new(url: &str) -> Result<Self, chronos_core::error::ChronosError> {
        let options = SqliteConnectOptions::from_str(url)
            .map_err(|e| chronos_core::error::ChronosError::Database(e.to_string()))?
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            // TODO(provisional): Using 5 connections for SQLite.
            // Rationale: Keeps resource footprint low for a background daemon while allowing concurrent CLI queries.
            // Trigger: Scaling to 10+ concurrent dashboard users or heavy background analytics.
            // Direction: Move to a config-based pool size or dynamic adjustment.
            .max_connections(5)
            .connect_with(options)
            .await
            .map_err(|e: sqlx::Error| chronos_core::error::ChronosError::Database(e.to_string()))?;

        // Run migrations. Path is relative to the crate root.
        sqlx::migrate!("../../migrations")
            .run(&pool)
            .await
            .map_err(|e: sqlx::migrate::MigrateError| {
                // [JUSTIFIED GAP]: Migration errors are unrecoverable system faults.
                chronos_core::error::ChronosError::Database(e.to_string())
            })?;

        Ok(Self { pool })
    }

    /// Create an in-memory database for testing purposes.
    /// Useful for isolated, fast integration tests.
    /// Using a unique shared cache URI ensures each call gets an isolated database.
    pub async fn new_in_memory() -> Result<Self, chronos_core::error::ChronosError> {
        let unique_id = ulid::Ulid::new().to_string();
        let url = format!("sqlite:file:{}?mode=memory&cache=shared", unique_id);
        Self::new(&url).await
    }

    /// Insert a new SemanticLog into the database.
    pub async fn insert_semantic_log(
        &self,
        log: &SemanticLog,
    ) -> Result<(), chronos_core::error::ChronosError> {
        let key_entities_json =
            serde_json::to_string(&log.key_entities).map_err(|e: serde_json::Error| {
                // [JUSTIFIED GAP]: Serializing simple types (Vec<String>) is logically infallible.
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

    /// Retrieve logs within a specific date range, up to the specified limit.
    pub async fn get_logs_by_date_range(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        limit: u64,
    ) -> Result<Vec<SemanticLog>, chronos_core::error::ChronosError> {
        let limit_i64 = i64::try_from(limit).map_err(|_| {
            chronos_core::error::ChronosError::InvalidInput(format!(
                "Limit {} is too large (max: {})",
                limit,
                i64::MAX
            ))
        })?;

        let rows = sqlx::query_as::<_, SemanticLogRow>(
            "SELECT * FROM semantic_logs WHERE timestamp BETWEEN ? AND ? ORDER BY timestamp ASC LIMIT ?",
        )
        .bind(from.to_rfc3339())
        .bind(to.to_rfc3339())
        .bind(limit_i64)
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
        limit: u64,
    ) -> Result<Vec<SemanticLog>, chronos_core::error::ChronosError> {
        let limit_i64 = i64::try_from(limit).map_err(|_| {
            chronos_core::error::ChronosError::InvalidInput(format!(
                "Limit {} is too large (max: {})",
                limit,
                i64::MAX
            ))
        })?;

        let rows = sqlx::query_as::<_, SemanticLogRow>(
            "SELECT * FROM semantic_logs ORDER BY timestamp DESC LIMIT ?",
        )
        .bind(limit_i64)
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
            raw_vlm_response: "{\"status\": \"ok\"}".to_string(),
        };

        db.insert_semantic_log(&log).await.unwrap();

        let count = db.get_log_count().await.unwrap();
        assert_eq!(count, 1);

        let recent = db.get_recent_logs(1).await.unwrap();
        assert_eq!(recent.len(), 1);

        // Verify all fields match after round-trip
        // Note: SQLite storage might have slightly different RFC3339 string representation
        // so we compare the important values or use the PartialEq implementation if precise.
        let retrieved = &recent[0];
        assert_eq!(retrieved.id, log.id);
        assert_eq!(retrieved.source_frame_id, log.source_frame_id);
        assert_eq!(retrieved.description, log.description);
        assert_eq!(retrieved.active_application, log.active_application);
        assert_eq!(retrieved.activity_category, log.activity_category);
        assert_eq!(retrieved.key_entities, log.key_entities);
        assert_eq!(retrieved.confidence_score, log.confidence_score);
        assert_eq!(retrieved.raw_vlm_response, log.raw_vlm_response);

        // Timestamps should match down to the second (SQLite precision in this impl)
        assert_eq!(retrieved.timestamp.to_rfc3339(), log.timestamp.to_rfc3339());
    }

    #[tokio::test]
    async fn test_empty_database_returns_zero_count() {
        let db = Database::new_in_memory().await.unwrap();
        let count = db.get_log_count().await.unwrap();
        assert_eq!(count, 0);

        let recent = db.get_recent_logs(10).await.unwrap();
        assert!(recent.is_empty());
    }

    #[tokio::test]
    async fn test_get_recent_logs_ordering_and_limit() {
        let db = Database::new_in_memory().await.unwrap();
        let now = Utc::now();

        // Insert 5 logs with increasing timestamps
        for i in 0..5 {
            let log = SemanticLog {
                id: Ulid::new(),
                timestamp: now + chrono::Duration::try_seconds(i).unwrap(),
                source_frame_id: Ulid::new(),
                description: format!("Log {}", i),
                active_application: None,
                activity_category: None,
                key_entities: vec![],
                confidence_score: 1.0,
                raw_vlm_response: "".to_string(),
            };
            db.insert_semantic_log(&log).await.unwrap();
        }

        // Test limit
        let recent_3 = db.get_recent_logs(3).await.unwrap();
        assert_eq!(recent_3.len(), 3);

        // Test ordering (most recent first)
        assert_eq!(recent_3[0].description, "Log 4");
        assert_eq!(recent_3[1].description, "Log 3");
        assert_eq!(recent_3[2].description, "Log 2");
    }

    #[tokio::test]
    async fn test_get_logs_by_date_range_inclusivity() {
        let db = Database::new_in_memory().await.unwrap();
        let now = Utc::now();
        let start = now - chrono::Duration::try_minutes(10).unwrap();
        let end = now;

        let log_at_start = SemanticLog {
            id: Ulid::new(),
            timestamp: start,
            source_frame_id: Ulid::new(),
            description: "At start".to_string(),
            active_application: None,
            activity_category: None,
            key_entities: vec![],
            confidence_score: 1.0,
            raw_vlm_response: "".to_string(),
        };

        let log_at_end = SemanticLog {
            id: Ulid::new(),
            timestamp: end,
            source_frame_id: Ulid::new(),
            description: "At end".to_string(),
            active_application: None,
            activity_category: None,
            key_entities: vec![],
            confidence_score: 1.0,
            raw_vlm_response: "".to_string(),
        };

        let log_outside = SemanticLog {
            id: Ulid::new(),
            timestamp: end + chrono::Duration::try_seconds(1).unwrap(),
            description: "Outside".to_string(),
            ..log_at_start.clone()
        };

        db.insert_semantic_log(&log_at_start).await.unwrap();
        db.insert_semantic_log(&log_at_end).await.unwrap();
        db.insert_semantic_log(&log_outside).await.unwrap();
        // Should include both start and end (inclusivity check)
        let range_logs = db.get_logs_by_date_range(start, end, 10).await.unwrap();
        assert_eq!(range_logs.len(), 2);
        assert!(range_logs.iter().any(|l| l.description == "At start"));
        assert!(range_logs.iter().any(|l| l.description == "At end"));
    }

    #[tokio::test]
    async fn test_get_logs_by_date_range_with_limit() {
        let db = Database::new_in_memory().await.unwrap();
        let now = Utc::now();
        let start = now - chrono::Duration::try_minutes(10).unwrap();
        let end = now;

        // Insert 5 logs in range
        for i in 0..5 {
            let log = SemanticLog {
                id: Ulid::new(),
                timestamp: start + chrono::Duration::try_seconds(i).unwrap(),
                source_frame_id: Ulid::new(),
                description: format!("Log {}", i),
                active_application: None,
                activity_category: None,
                key_entities: vec![],
                confidence_score: 1.0,
                raw_vlm_response: "".to_string(),
            };
            db.insert_semantic_log(&log).await.unwrap();
        }

        // Query with limit 2
        let logs = db.get_logs_by_date_range(start, end, 2).await.unwrap();
        assert_eq!(logs.len(), 2);
    }

    #[tokio::test]
    async fn test_get_recent_logs_with_limit() {
        let db = Database::new_in_memory().await.unwrap();
        // Insert a dummy log
        db.insert_semantic_log(&SemanticLog {
            id: Ulid::new(),
            source_frame_id: Ulid::new(),
            timestamp: Utc::now(),
            description: "Recent 1".to_string(),
            active_application: None,
            activity_category: None,
            key_entities: vec![],
            confidence_score: 1.0,
            raw_vlm_response: "".to_string(),
        })
        .await
        .unwrap();

        let recent = db.get_recent_logs(1).await.unwrap();
        assert_eq!(recent.len(), 1);
    }

    #[tokio::test]
    async fn test_confidence_score_constraint() {
        let db = Database::new_in_memory().await.unwrap();
        let mut log = SemanticLog {
            id: Ulid::new(),
            timestamp: Utc::now(),
            source_frame_id: Ulid::new(),
            description: "Invalid score".to_string(),
            active_application: None,
            activity_category: None,
            key_entities: vec![],
            confidence_score: 1.1, // OUT OF RANGE [0.0, 1.0]
            raw_vlm_response: "".to_string(),
        };

        // This should fail due to the CHECK constraint in the DB schema
        let result = db.insert_semantic_log(&log).await;
        let err = result.expect_err("Expected CHECK constraint failure");
        if let chronos_core::error::ChronosError::Database(msg) = err {
            assert!(msg.to_lowercase().contains("check constraint failed"));
        } else {
            // [JUSTIFIED GAP]: This branch is a safeguard for the test itself;
            // it is hit only if the test framework or DB expectations fail.
            panic!("Expected Database error, got {:?}", err);
        }

        // Test lower bound
        log.confidence_score = -0.1;
        let result = db.insert_semantic_log(&log).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_malformed_data_handling() {
        let db = Database::new_in_memory().await.unwrap();

        // 1. Invalid ID
        sqlx::query("INSERT INTO semantic_logs (id, timestamp, source_frame_id, description, active_application, activity_category, key_entities, confidence_score, raw_vlm_response) 
            VALUES ('not-a-ulid', '2023-01-01T00:00:00Z', '01GD6Q3B86Y4G1C6EM86YXAK8F', 'd', 'a', 'c', '[]', 1.0, 'r')")
            .execute(&db.pool).await.unwrap();
        assert!(db.get_recent_logs(1).await.is_err());
        sqlx::query("DELETE FROM semantic_logs")
            .execute(&db.pool)
            .await
            .unwrap();

        // 2. Invalid Timestamp
        sqlx::query("INSERT INTO semantic_logs (id, timestamp, source_frame_id, description, active_application, activity_category, key_entities, confidence_score, raw_vlm_response) 
            VALUES ('01GD6Q3B86Y4G1C6EM86YXAK8F', 'invalid-date', '01GD6Q3B86Y4G1C6EM86YXAK8F', 'd', 'a', 'c', '[]', 1.0, 'r')")
            .execute(&db.pool).await.unwrap();
        assert!(db.get_recent_logs(1).await.is_err());
        sqlx::query("DELETE FROM semantic_logs")
            .execute(&db.pool)
            .await
            .unwrap();

        // 3. Invalid Source Frame ID
        sqlx::query("INSERT INTO semantic_logs (id, timestamp, source_frame_id, description, active_application, activity_category, key_entities, confidence_score, raw_vlm_response) 
            VALUES ('01GD6Q3B86Y4G1C6EM86YXAK8F', '2023-01-01T00:00:00Z', 'not-a-ulid', 'd', 'a', 'c', '[]', 1.0, 'r')")
            .execute(&db.pool).await.unwrap();
        assert!(db.get_recent_logs(1).await.is_err());
        sqlx::query("DELETE FROM semantic_logs")
            .execute(&db.pool)
            .await
            .unwrap();

        // 4. Invalid Key Entities (JSON)
        sqlx::query("INSERT INTO semantic_logs (id, timestamp, source_frame_id, description, active_application, activity_category, key_entities, confidence_score, raw_vlm_response) 
            VALUES ('01GD6Q3B86Y4G1C6EM86YXAK8F', '2023-01-01T00:00:00Z', '01GD6Q3B86Y4G1C6EM86YXAK8F', 'd', 'a', 'c', 'invalid-json', 1.0, 'r')")
            .execute(&db.pool).await.unwrap();
        assert!(db.get_recent_logs(1).await.is_err());
    }

    #[tokio::test]
    async fn test_database_new_failure() {
        // Use an invalid sqlite URI to trigger connection error
        let result = Database::new("sqlite://invalid/path/that/cannot/exist/db.sqlite").await;
        assert!(result.is_err());
    }
    #[tokio::test]
    async fn test_limit_overflow_protection() {
        let db = Database::new_in_memory().await.unwrap();
        let too_large_limit = i64::MAX as u64 + 1;

        let res = db.get_recent_logs(too_large_limit).await;
        assert!(matches!(
            res,
            Err(chronos_core::error::ChronosError::InvalidInput(_))
        ));

        let res = db
            .get_logs_by_date_range(Utc::now(), Utc::now(), too_large_limit)
            .await;
        assert!(matches!(
            res,
            Err(chronos_core::error::ChronosError::InvalidInput(_))
        ));
    }
}
