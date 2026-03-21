-- UP: Create the semantic_logs table
CREATE TABLE IF NOT EXISTS semantic_logs (
    id              TEXT PRIMARY KEY NOT NULL,    -- ULID as text
    timestamp       TEXT NOT NULL,                -- ISO 8601
    source_frame_id TEXT NOT NULL,                -- ULID of the originating frame
    description     TEXT NOT NULL,                -- VLM-generated description
    active_application TEXT,                      -- Detected active window
    activity_category  TEXT,                      -- Classified activity type
    key_entities    TEXT NOT NULL DEFAULT '[]',   -- JSON array of strings
    confidence_score REAL NOT NULL DEFAULT 0.0 CHECK (confidence_score >= 0.0 AND confidence_score <= 1.0),
    raw_vlm_response TEXT NOT NULL,               -- Full VLM JSON response
    created_at      TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Index for time-range queries (the most common query pattern)
CREATE INDEX IF NOT EXISTS idx_semantic_logs_timestamp ON semantic_logs(timestamp);

-- Index for filtering by application
CREATE INDEX IF NOT EXISTS idx_semantic_logs_app ON semantic_logs(active_application);
