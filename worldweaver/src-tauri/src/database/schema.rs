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

-- Map generator tables
CREATE TABLE IF NOT EXISTS generated_maps (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    theme TEXT NOT NULL,
    seed INTEGER NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    data_json TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    modified_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS map_settlements (
    id TEXT PRIMARY KEY,
    map_id TEXT NOT NULL,
    name TEXT NOT NULL,
    x REAL NOT NULL,
    y REAL NOT NULL,
    settlement_type TEXT NOT NULL,
    population INTEGER NOT NULL,
    biome TEXT NOT NULL,
    room_id TEXT,
    FOREIGN KEY (map_id) REFERENCES generated_maps(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_map_settlements_map ON map_settlements(map_id);

-- Insert schema version
INSERT OR REPLACE INTO world_meta (key, value) VALUES ('schema_version', '1');
"#;
