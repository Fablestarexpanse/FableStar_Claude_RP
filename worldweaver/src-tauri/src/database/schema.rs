/// Current database schema version
pub const SCHEMA_VERSION: i32 = 1;

/// SQL statements to create all tables
pub const CREATE_TABLES: &str = r#"
CREATE TABLE IF NOT EXISTS world_meta (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS entities (
    id TEXT PRIMARY KEY,
    entity_type TEXT NOT NULL,
    data BLOB NOT NULL,
    created_at INTEGER NOT NULL,
    modified_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS event_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tick INTEGER NOT NULL,
    event_type TEXT NOT NULL,
    entity_id TEXT,
    data TEXT NOT NULL,
    timestamp INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_entities_type ON entities(entity_type);
CREATE INDEX IF NOT EXISTS idx_events_tick ON event_log(tick);
CREATE INDEX IF NOT EXISTS idx_events_entity ON event_log(entity_id);

-- Insert schema version
INSERT OR REPLACE INTO world_meta (key, value) VALUES ('schema_version', '1');
"#;
