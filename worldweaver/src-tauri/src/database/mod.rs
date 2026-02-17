use rusqlite::{Connection, params};
use anyhow::{Result, Context};
use std::path::Path;

pub mod schema;
pub mod queries;
pub mod persistence;

/// Database wrapper for world persistence
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Open or create a database at the given path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)
            .context("Failed to open database")?;
        
        // Enable WAL mode for better concurrency
        conn.execute("PRAGMA journal_mode=WAL", [])
            .context("Failed to set WAL mode")?;
        
        conn.execute("PRAGMA synchronous=NORMAL", [])
            .context("Failed to set synchronous mode")?;
        
        // Initialize schema
        conn.execute_batch(schema::CREATE_TABLES)
            .context("Failed to create tables")?;
        
        Ok(Self { conn })
    }
    
    /// Get the current schema version from the database
    pub fn get_schema_version(&self) -> Result<i32> {
        let version: String = self.conn.query_row(
            "SELECT value FROM world_meta WHERE key = 'schema_version'",
            [],
            |row| row.get(0)
        ).context("Failed to get schema version")?;
        
        version.parse().context("Invalid schema version")
    }
    
    /// Save an entity to the database
    pub fn save_entity(&self, id: &str, entity_type: &str, data: &[u8]) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        
        self.conn.execute(
            "INSERT OR REPLACE INTO entities (id, entity_type, data, created_at, modified_at)
             VALUES (?1, ?2, ?3, ?4, ?4)",
            params![id, entity_type, data, now]
        ).context("Failed to save entity")?;
        
        Ok(())
    }
    
    /// Log a world event
    pub fn log_event(&self, tick: u64, event_type: &str, entity_id: Option<&str>, data: &str) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        
        self.conn.execute(
            "INSERT INTO event_log (tick, event_type, entity_id, data, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![tick as i64, event_type, entity_id, data, now]
        ).context("Failed to log event")?;
        
        Ok(())
    }
}
