/// MCP Tool definitions for WorldWeaver
/// These tools allow Claude to query world state and generate contextual narrative

use serde::{Serialize, Deserialize};

/// Tool: Get Room State
/// Returns current state of a room including NPCs, exits, and ambient conditions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetRoomStateTool {
    pub name: String,
    pub description: String,
    pub parameters: RoomStateParams,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoomStateParams {
    pub room_id: String,
}

impl Default for GetRoomStateTool {
    fn default() -> Self {
        Self {
            name: "get_room_state".to_string(),
            description: "Get current state of a room including NPCs present, exits, time of day, and recent events".to_string(),
            parameters: RoomStateParams {
                room_id: String::new(),
            },
        }
    }
}

/// Tool: Get NPC Context
/// Returns NPC personality, memory, current activity, and mood for dialogue generation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetNPCContextTool {
    pub name: String,
    pub description: String,
    pub parameters: NPCContextParams,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NPCContextParams {
    pub npc_id: String,
    pub player_id: Option<String>,
}

impl Default for GetNPCContextTool {
    fn default() -> Self {
        Self {
            name: "get_npc_context".to_string(),
            description: "Get NPC personality, memory, current activity, and mood for generating contextual dialogue".to_string(),
            parameters: NPCContextParams {
                npc_id: String::new(),
                player_id: None,
            },
        }
    }
}

/// Tool: Get World Events
/// Returns recent world events that might affect narrative
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetWorldEventsTool {
    pub name: String,
    pub description: String,
    pub parameters: WorldEventsParams,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldEventsParams {
    pub limit: usize,
    pub event_types: Option<Vec<String>>,
}

impl Default for GetWorldEventsTool {
    fn default() -> Self {
        Self {
            name: "get_world_events".to_string(),
            description: "Get recent world events that might affect the narrative or NPC dialogue".to_string(),
            parameters: WorldEventsParams {
                limit: 10,
                event_types: None,
            },
        }
    }
}

/// Tool: Record Conversation
/// Records a conversation summary in NPC memory for future reference
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecordConversationTool {
    pub name: String,
    pub description: String,
    pub parameters: RecordConversationParams,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecordConversationParams {
    pub npc_id: String,
    pub player_name: String,
    pub summary: String,
}

impl Default for RecordConversationTool {
    fn default() -> Self {
        Self {
            name: "record_conversation".to_string(),
            description: "Record a conversation summary in NPC memory so they remember past interactions".to_string(),
            parameters: RecordConversationParams {
                npc_id: String::new(),
                player_name: String::new(),
                summary: String::new(),
            },
        }
    }
}

/// Tool: Query Faction Relations
/// Returns reputation values and faction relationships
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryFactionRelationsTool {
    pub name: String,
    pub description: String,
    pub parameters: FactionRelationsParams,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FactionRelationsParams {
    pub player_id: String,
    pub npc_id: Option<String>,
}

impl Default for QueryFactionRelationsTool {
    fn default() -> Self {
        Self {
            name: "query_faction_relations".to_string(),
            description: "Get player reputation with factions and how it affects NPC interactions".to_string(),
            parameters: FactionRelationsParams {
                player_id: String::new(),
                npc_id: None,
            },
        }
    }
}

/// Tool: Get Economy State
/// Returns shop prices, commodity availability, and economic conditions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetEconomyStateTool {
    pub name: String,
    pub description: String,
    pub parameters: EconomyStateParams,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EconomyStateParams {
    pub shop_id: Option<String>,
    pub commodity_type: Option<String>,
}

impl Default for GetEconomyStateTool {
    fn default() -> Self {
        Self {
            name: "get_economy_state".to_string(),
            description: "Get current shop prices, commodity availability, and economic conditions affecting trade".to_string(),
            parameters: EconomyStateParams {
                shop_id: None,
                commodity_type: None,
            },
        }
    }
}

/// Registry of all available MCP tools
pub struct ToolRegistry {
    pub tools: Vec<ToolDefinition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ToolDefinition {
    GetRoomState(GetRoomStateTool),
    GetNPCContext(GetNPCContextTool),
    GetWorldEvents(GetWorldEventsTool),
    RecordConversation(RecordConversationTool),
    QueryFactionRelations(QueryFactionRelationsTool),
    GetEconomyState(GetEconomyStateTool),
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: vec![
                ToolDefinition::GetRoomState(GetRoomStateTool::default()),
                ToolDefinition::GetNPCContext(GetNPCContextTool::default()),
                ToolDefinition::GetWorldEvents(GetWorldEventsTool::default()),
                ToolDefinition::RecordConversation(RecordConversationTool::default()),
                ToolDefinition::QueryFactionRelations(QueryFactionRelationsTool::default()),
                ToolDefinition::GetEconomyState(GetEconomyStateTool::default()),
            ],
        }
    }

    pub fn get_tool_names(&self) -> Vec<String> {
        self.tools.iter().map(|tool| {
            match tool {
                ToolDefinition::GetRoomState(t) => t.name.clone(),
                ToolDefinition::GetNPCContext(t) => t.name.clone(),
                ToolDefinition::GetWorldEvents(t) => t.name.clone(),
                ToolDefinition::RecordConversation(t) => t.name.clone(),
                ToolDefinition::QueryFactionRelations(t) => t.name.clone(),
                ToolDefinition::GetEconomyState(t) => t.name.clone(),
            }
        }).collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Critical Design Note:
// 
// These tools are READ-ONLY for world state queries and WRITE-ONLY for memory recording.
// Claude NEVER makes mechanical decisions such as:
// - Setting prices
// - Determining success/failure of actions
// - Generating loot
// - Modifying game state directly
// 
// The LLM is the VOICE of the world, not its BRAIN.
// All mechanical decisions are made by the deterministic simulation engine.
