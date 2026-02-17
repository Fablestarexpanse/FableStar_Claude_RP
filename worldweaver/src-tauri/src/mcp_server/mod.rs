pub mod tools;
pub mod context;
pub mod server;

/// MCP (Model Context Protocol) server implementation for WorldWeaver
/// This module provides tools for Claude to query world state and generate narrative
/// 
/// Note: This is a placeholder implementation. Full MCP integration requires:
/// 1. The rmcp crate to be properly configured
/// 2. Claude Desktop or API connection setup
/// 3. Tool registration and stdio transport configuration

use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use anyhow::Result;

use crate::simulation::world::GameWorld;

/// MCP server for WorldWeaver
/// Provides tools for Claude to interact with the game world
pub struct WorldWeaverMCP {
    simulation: Arc<Mutex<GameWorld>>,
}

impl WorldWeaverMCP {
    pub fn new(simulation: Arc<Mutex<GameWorld>>) -> Self {
        Self { simulation }
    }

    /// Get current state of a room including NPCs present
    pub async fn get_room_state(&self, room_id: String) -> Result<RoomState> {
        let mut sim = self.simulation.lock().await;
        let room_uuid = Uuid::parse_str(&room_id)?;
        
        let room = sim.get_room_details(room_uuid)
            .ok_or_else(|| anyhow::anyhow!("Room not found"))?;
        
        let npcs = sim.get_npcs_in_room(room_uuid);
        
        Ok(RoomState {
            room_name: room.name,
            room_description: room.description,
            npcs_present: npcs.iter().map(|npc| npc.name.clone()).collect(),
            exits: room.exits.iter().map(|e| e.direction.clone()).collect(),
            current_time: format!("Day {}, Hour {}", 1, 12), // Placeholder
        })
    }

    /// Get NPC personality and context for dialogue generation
    pub async fn get_npc_context(&self, npc_id: String) -> Result<NPCContext> {
        let mut sim = self.simulation.lock().await;
        let room_id = sim.get_player_room()
            .ok_or_else(|| anyhow::anyhow!("Player has no position"))?;
        
        let npcs = sim.get_npcs_in_room(room_id);
        
        // Find NPC by name (for MVP)
        let npc = npcs.iter()
            .find(|n| n.name.to_lowercase().contains(&npc_id.to_lowercase()))
            .ok_or_else(|| anyhow::anyhow!("NPC not found"))?;
        
        Ok(NPCContext {
            name: npc.name.clone(),
            personality: npc.personality.clone(),
            greeting: npc.greeting.clone(),
            current_activity: "tending the bar".to_string(), // Placeholder
            mood: "friendly".to_string(), // Placeholder
            recent_conversations: vec![], // Placeholder
        })
    }

    /// Record a conversation in NPC memory
    pub async fn record_conversation(
        &self,
        npc_id: String,
        player_name: String,
        summary: String,
    ) -> Result<String> {
        // TODO: Implement NPC memory storage
        println!("📝 Recording conversation: {} with {}: {}", player_name, npc_id, summary);
        Ok("Conversation recorded".to_string())
    }

    /// Get recent world events
    pub async fn get_world_events(&self, _limit: usize) -> Result<Vec<String>> {
        // TODO: Query event log from database
        Ok(vec![
            "The Crossroads Inn opened for business".to_string(),
            "A traveler arrived seeking rest".to_string(),
        ])
    }
}

/// Room state for MCP tools
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoomState {
    pub room_name: String,
    pub room_description: String,
    pub npcs_present: Vec<String>,
    pub exits: Vec<String>,
    pub current_time: String,
}

/// NPC context for dialogue generation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NPCContext {
    pub name: String,
    pub personality: String,
    pub greeting: String,
    pub current_activity: String,
    pub mood: String,
    pub recent_conversations: Vec<String>,
}

/// Future: MCP tool definitions using rmcp macros
/// This is what the implementation will look like once rmcp is properly integrated:
/// 
/// ```rust
/// use rmcp::prelude::*;
/// 
/// #[tool_router]
/// impl WorldWeaverMCP {
///     #[tool(description = "Get current state of a room including NPCs present")]
///     async fn get_room_state(&self, room_id: String) -> Result<Json<RoomState>> {
///         // Implementation
///     }
///     
///     #[tool(description = "Get NPC personality and memory for dialogue generation")]
///     async fn get_npc_context(&self, npc_id: String) -> Result<Json<NPCContext>> {
///         // Implementation
///     }
/// }
/// ```

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_server_creation() {
        let world = Arc::new(Mutex::new(GameWorld::new()));
        let mcp = WorldWeaverMCP::new(world);
        
        // Test that we can create the server
        assert!(true);
    }
}
