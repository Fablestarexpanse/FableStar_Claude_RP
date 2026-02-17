use rusqlite::{Connection, params};
use anyhow::{Result, Context};
use crate::simulation::world::GameWorld;

/// Manages periodic persistence of game world to SQLite
pub struct PersistenceManager {
    conn: Connection,
    last_save_tick: u64,
    save_interval: u64,  // Save every N ticks
}

impl PersistenceManager {
    /// Create a new persistence manager
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)
            .context("Failed to open database")?;
        
        // Configure for performance (from research recommendations)
        conn.execute("PRAGMA journal_mode=WAL", [])
            .context("Failed to set WAL mode")?;
        conn.execute("PRAGMA synchronous=NORMAL", [])
            .context("Failed to set synchronous mode")?;
        conn.execute("PRAGMA cache_size=-64000", [])
            .context("Failed to set cache size")?;  // 64MB cache
        
        Ok(Self {
            conn,
            last_save_tick: 0,
            save_interval: 60,  // Every 60 ticks by default
        })
    }
    
    /// Check if it's time to save
    pub fn should_save(&self, current_tick: u64) -> bool {
        current_tick - self.last_save_tick >= self.save_interval
    }
    
    /// Set the save interval
    pub fn set_save_interval(&mut self, interval: u64) {
        self.save_interval = interval;
    }
    
    /// Save the world state to database
    pub async fn save_world(&mut self, world: &GameWorld) -> Result<()> {
        let tx = self.conn.transaction()
            .context("Failed to start transaction")?;
        
        // Save world metadata
        tx.execute(
            "INSERT OR REPLACE INTO world_meta (key, value) VALUES (?, ?)",
            params!["tick_count", world.tick_count.to_string()]
        ).context("Failed to save tick count")?;
        
        // Save event log (append-only for events since last save)
        let new_events = world.get_events_since(self.last_save_tick);
        for event in &new_events {
            let event_json = serde_json::to_string(&event)
                .context("Failed to serialize event")?;
            
            tx.execute(
                "INSERT INTO event_log (tick, event_type, data, timestamp) VALUES (?, ?, ?, ?)",
                params![
                    event.tick as i64,
                    event.event.event_type(),
                    event_json,
                    event.timestamp.timestamp()
                ]
            ).context("Failed to save event")?;
        }
        
        // TODO: Save entity snapshots (only changed entities)
        // This requires tracking dirty entities in GameWorld
        // For now, we just save the tick count and events
        
        tx.commit().context("Failed to commit transaction")?;
        
        self.last_save_tick = world.tick_count;
        
        println!("💾 World saved at tick {} ({} events)", world.tick_count, new_events.len());
        Ok(())
    }
    
    /// Load world state from database
    pub fn load_world(&self) -> Result<GameWorld> {
        // Load tick count
        let tick_count: u64 = self.conn.query_row(
            "SELECT value FROM world_meta WHERE key = ?",
            params!["tick_count"],
            |row| {
                let value: String = row.get(0)?;
                Ok(value)
            }
        ).unwrap_or_else(|_| "0".to_string())
        .parse()
        .unwrap_or(0);
        
        // Create new world with loaded tick count
        let mut world = GameWorld::new();
        world.tick_count = tick_count;
        
        // TODO: Load entities from database and spawn them in ECS
        // TODO: Replay events since last snapshot to reconstruct state
        // For MVP, we start with the default starter world
        
        println!("📂 World loaded from database (tick: {})", tick_count);
        Ok(world)
    }
    
    /// Get the last saved tick
    pub fn get_last_save_tick(&self) -> u64 {
        self.last_save_tick
    }
    
    /// Compact old events (keep only recent N ticks)
    pub fn compact_events(&self, keep_ticks: u64) -> Result<usize> {
        let current_tick: u64 = self.conn.query_row(
            "SELECT value FROM world_meta WHERE key = ?",
            params!["tick_count"],
            |row| {
                let value: String = row.get(0)?;
                Ok(value)
            }
        ).unwrap_or_else(|_| "0".to_string())
        .parse()
        .unwrap_or(0);
        
        let cutoff_tick = current_tick.saturating_sub(keep_ticks);
        
        let deleted = self.conn.execute(
            "DELETE FROM event_log WHERE tick < ?",
            params![cutoff_tick as i64]
        ).context("Failed to compact events")?;
        
        println!("🗑️  Compacted {} old events (kept last {} ticks)", deleted, keep_ticks);
        Ok(deleted)
    }
    
    /// Get database statistics
    pub fn get_stats(&self) -> Result<DatabaseStats> {
        let event_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM event_log",
            [],
            |row| row.get(0)
        ).unwrap_or(0);
        
        let entity_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM entities",
            [],
            |row| row.get(0)
        ).unwrap_or(0);
        
        // Get database file size
        let page_count: i64 = self.conn.query_row(
            "PRAGMA page_count",
            [],
            |row| row.get(0)
        ).unwrap_or(0);
        
        let page_size: i64 = self.conn.query_row(
            "PRAGMA page_size",
            [],
            |row| row.get(0)
        ).unwrap_or(4096);
        
        let size_bytes = page_count * page_size;
        
        Ok(DatabaseStats {
            event_count: event_count as usize,
            entity_count: entity_count as usize,
            size_bytes: size_bytes as usize,
            last_save_tick: self.last_save_tick,
        })
    }
}

/// Database statistics
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    pub event_count: usize,
    pub entity_count: usize,
    pub size_bytes: usize,
    pub last_save_tick: u64,
}

impl DatabaseStats {
    pub fn size_mb(&self) -> f64 {
        self.size_bytes as f64 / 1_048_576.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::schema::CREATE_TABLES;

    fn setup_test_db() -> PersistenceManager {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(CREATE_TABLES).unwrap();
        
        PersistenceManager {
            conn,
            last_save_tick: 0,
            save_interval: 60,
        }
    }

    #[test]
    fn test_should_save() {
        let manager = setup_test_db();
        
        assert!(manager.should_save(60));
        assert!(!manager.should_save(59));
        assert!(manager.should_save(120));
    }
    
    #[tokio::test]
    async fn test_save_and_load() {
        let mut manager = setup_test_db();
        let mut world = GameWorld::new();
        
        // Advance world
        world.tick();
        world.tick();
        world.tick();
        
        // Save
        manager.save_world(&world).await.unwrap();
        
        // Load
        let loaded_world = manager.load_world().unwrap();
        
        // Verify tick count was persisted
        assert_eq!(loaded_world.tick_count, world.tick_count);
    }
    
    #[test]
    fn test_database_stats() {
        let manager = setup_test_db();
        let stats = manager.get_stats().unwrap();
        
        assert_eq!(stats.event_count, 0);
        assert_eq!(stats.entity_count, 0);
        assert!(stats.size_bytes > 0);
    }
}
