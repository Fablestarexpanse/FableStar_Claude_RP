use rusqlite::{Connection, params};
use anyhow::{Result, Context};
use serde_json;
use uuid::Uuid;

use crate::simulation::world::{GameWorld, RoomDetails, NpcInfo};

/// Database queries for world serialization and deserialization
pub struct WorldQueries {
    conn: Connection,
}

impl WorldQueries {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    /// Save a room entity to the database
    pub fn save_room(&self, room_id: Uuid, room: &RoomDetails) -> Result<()> {
        let exits_json = serde_json::to_string(&room.exits)
            .context("Failed to serialize room exits")?;
        
        let now = chrono::Utc::now().timestamp();
        
        self.conn.execute(
            "INSERT OR REPLACE INTO entities (id, entity_type, data, created_at, modified_at)
             VALUES (?1, ?2, ?3, ?4, ?4)",
            params![
                room_id.to_string(),
                "room",
                exits_json.as_bytes(),
                now
            ]
        ).context("Failed to save room")?;
        
        Ok(())
    }

    /// Save an NPC entity to the database
    pub fn save_npc(&self, npc_id: Uuid, npc: &NpcInfo, _room_id: Uuid) -> Result<()> {
        let npc_json = serde_json::to_string(&npc)
            .context("Failed to serialize NPC")?;
        
        let now = chrono::Utc::now().timestamp();
        
        self.conn.execute(
            "INSERT OR REPLACE INTO entities (id, entity_type, data, created_at, modified_at)
             VALUES (?1, ?2, ?3, ?4, ?4)",
            params![
                npc_id.to_string(),
                "npc",
                npc_json.as_bytes(),
                now
            ]
        ).context("Failed to save NPC")?;
        
        Ok(())
    }

    /// Save world tick count
    pub fn save_tick_count(&self, tick_count: u64) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO world_meta (key, value) VALUES (?1, ?2)",
            params!["tick_count", tick_count.to_string()]
        ).context("Failed to save tick count")?;
        
        Ok(())
    }

    /// Load world tick count
    pub fn load_tick_count(&self) -> Result<u64> {
        let tick_str: String = self.conn.query_row(
            "SELECT value FROM world_meta WHERE key = ?1",
            params!["tick_count"],
            |row| row.get(0)
        ).unwrap_or_else(|_| "0".to_string());
        
        Ok(tick_str.parse().unwrap_or(0))
    }

    /// Save entire world state
    pub fn save_world(&mut self, world: &GameWorld) -> Result<()> {
        let tx = self.conn.transaction()
            .context("Failed to start transaction")?;
        
        // Save tick count
        tx.execute(
            "INSERT OR REPLACE INTO world_meta (key, value) VALUES (?1, ?2)",
            params!["tick_count", world.tick_count.to_string()]
        ).context("Failed to save tick count in transaction")?;
        
        // TODO: Iterate through ECS entities and save them
        // For MVP, we have limited entities, so this is a placeholder
        
        tx.commit().context("Failed to commit transaction")?;
        
        println!("💾 World saved to database (tick: {})", world.tick_count);
        Ok(())
    }

    /// Load world state from database
    pub fn load_world(&self) -> Result<GameWorld> {
        let mut world = GameWorld::new();
        
        // Load tick count
        world.tick_count = self.load_tick_count()?;
        
        // TODO: Load entities from database and spawn them in ECS
        // For MVP, we start with the default starter world
        
        println!("📂 World loaded from database (tick: {})", world.tick_count);
        Ok(world)
    }

    /// Log a world event
    pub fn log_event(&self, tick: u64, event_type: &str, entity_id: Option<Uuid>, data: &str) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        
        self.conn.execute(
            "INSERT INTO event_log (tick, event_type, entity_id, data, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                tick as i64,
                event_type,
                entity_id.map(|id| id.to_string()),
                data,
                now
            ]
        ).context("Failed to log event")?;
        
        Ok(())
    }

    /// Get recent events from the log
    pub fn get_recent_events(&self, limit: usize) -> Result<Vec<WorldEvent>> {
        let mut stmt = self.conn.prepare(
            "SELECT tick, event_type, data FROM event_log 
             ORDER BY tick DESC LIMIT ?1"
        )?;
        
        let events = stmt.query_map(params![limit], |row| {
            Ok(WorldEvent {
                tick: row.get::<_, i64>(0)? as u64,
                event_type: row.get(1)?,
                data: row.get(2)?,
            })
        })?;
        
        let mut result = Vec::new();
        for event in events {
            result.push(event?);
        }
        
        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub struct WorldEvent {
    pub tick: u64,
    pub event_type: String,
    pub data: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use crate::database::schema::CREATE_TABLES;

    fn setup_test_db() -> WorldQueries {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(CREATE_TABLES).unwrap();
        WorldQueries::new(conn)
    }

    #[test]
    fn test_save_and_load_tick_count() {
        let queries = setup_test_db();
        
        queries.save_tick_count(42).unwrap();
        let loaded = queries.load_tick_count().unwrap();
        
        assert_eq!(loaded, 42);
    }

    #[test]
    fn test_log_event() {
        let queries = setup_test_db();
        
        queries.log_event(
            100,
            "player_action",
            Some(Uuid::new_v4()),
            "Player entered tavern"
        ).unwrap();
        
        let events = queries.get_recent_events(10).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].tick, 100);
    }
}
