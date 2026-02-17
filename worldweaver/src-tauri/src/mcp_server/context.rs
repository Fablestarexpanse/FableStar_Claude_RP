/// Context assembly for Claude LLM
/// Prepares rich, structured context from ECS world state for narrative generation

use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use anyhow::Result;

use crate::simulation::world::{GameWorld, RoomDetails, NpcInfo};
use crate::simulation::events::EventRecord;
use crate::simulation::components::RelationshipData;

/// Assembles context from game world for LLM consumption
pub struct ContextAssembler {
    simulation: Arc<Mutex<GameWorld>>,
}

impl ContextAssembler {
    pub fn new(simulation: Arc<Mutex<GameWorld>>) -> Self {
        Self { simulation }
    }

    /// Build comprehensive context for room description generation
    pub async fn build_room_context(&self, room_id: Uuid) -> Result<RoomContext> {
        let mut sim = self.simulation.lock().await;
        
        let room = sim.get_room_details(room_id)
            .ok_or_else(|| anyhow::anyhow!("Room not found"))?;
        
        let npcs = sim.get_npcs_in_room(room_id);
        
        let ambient = self.calculate_ambient_conditions(&room, &npcs);
        
        Ok(RoomContext {
            room_details: room.clone(),
            npcs_present: npcs,
            time_of_day: self.get_time_description(),
            weather: "clear skies".to_string(), // Placeholder
            recent_events: vec![], // TODO: Query from event log
            ambient_conditions: ambient,
        })
    }

    /// Build context for NPC dialogue generation
    pub async fn build_dialogue_context(
        &self,
        npc_name: &str,
        _player_id: Uuid,
    ) -> Result<DialogueContext> {
        let mut sim = self.simulation.lock().await;

        // Get NPC's current room
        let room_id = sim.get_player_room()
            .ok_or_else(|| anyhow::anyhow!("Cannot find player location"))?;

        let npcs = sim.get_npcs_in_room(room_id);
        let npc = npcs.iter()
            .find(|n| n.name.to_lowercase().contains(&npc_name.to_lowercase()))
            .ok_or_else(|| anyhow::anyhow!("NPC not found in current room"))?
            .clone();

        // Get relevant world events (filtered by tags)
        let relevant_events = sim.query_events_by_tag("player", 20);
        
        // Calculate mood based on recent events and personality
        let mood = self.calculate_npc_mood(&npc, &relevant_events);
        
        // Summarize events for context
        let event_summaries = self.summarize_events(&relevant_events);
        
        // Build relationship data (placeholder until we have actual relationship tracking)
        let relationship = RelationshipData {
            affinity: 0,
            trust: 50,
            last_interaction_tick: sim.tick_count,
        };
        
        // Get room context
        let room_context = self.build_room_context(room_id).await?;

        Ok(DialogueContext {
            npc,
            npc_memory: vec![], // TODO: Load from DialogueMemory component
            npc_current_activity: self.get_npc_activity(&room_context),
            npc_mood: mood,
            player_reputation: relationship.affinity,
            room_context,
            faction_relations: vec![], // TODO: Query faction system when implemented
            conversation_history: vec![], // TODO: Get from DialogueMemory
            relevant_events: event_summaries,
        })
    }

    /// Get time of day description
    fn get_time_description(&self) -> String {
        // TODO: Get from WorldClock resource
        "midday".to_string()
    }

    /// Calculate ambient conditions based on room and NPCs
    fn calculate_ambient_conditions(&self, _room: &RoomDetails, npcs: &[NpcInfo]) -> String {
        if npcs.is_empty() {
            "The room is quiet and empty.".to_string()
        } else {
            format!("The room is occupied by {} people.", npcs.len())
        }
    }

    /// Calculate NPC mood based on recent events and personality
    fn calculate_npc_mood(&self, npc: &NpcInfo, events: &[EventRecord]) -> String {
        // Mood calculation based on:
        // - Base personality traits
        // - Recent events affecting the NPC
        // - Current time of day (future enhancement)
        // - NPC needs/stress (future: Dwarf Fortress style)
        
        let mut mood_score = 0;
        
        // Base mood from personality
        let personality_lower = npc.personality.to_lowercase();
        if personality_lower.contains("friendly") || personality_lower.contains("welcoming") {
            mood_score += 20;
        } else if personality_lower.contains("grumpy") || personality_lower.contains("hostile") {
            mood_score -= 20;
        }
        
        // Adjust based on recent events
        for event in events.iter().take(5) {
            // Check if event involves this NPC (by name matching in tags)
            if event.tags.iter().any(|tag| tag.to_lowercase().contains(&npc.name.to_lowercase())) {
                // Positive events
                if event.tags.contains(&"dialogue".to_string()) {
                    mood_score += 5; // Conversation improves mood
                }
                // Negative events would decrease mood_score
            }
        }
        
        // Convert score to descriptive mood
        if mood_score > 30 {
            "cheerful and welcoming"
        } else if mood_score > 10 {
            "friendly and approachable"
        } else if mood_score > -10 {
            "neutral and professional"
        } else if mood_score > -30 {
            "reserved and cautious"
        } else {
            "hostile and suspicious"
        }.to_string()
    }

    /// Get NPC's current activity based on room context
    fn get_npc_activity(&self, room_context: &RoomContext) -> String {
        // Derive activity from room name for now
        if room_context.room_details.name.contains("Inn") {
            "tending the bar".to_string()
        } else if room_context.room_details.name.contains("Forge") {
            "working at the forge".to_string()
        } else if room_context.room_details.name.contains("Square") {
            "observing the marketplace".to_string()
        } else {
            "present in the room".to_string()
        }
    }
    
    /// Summarize events for LLM context
    fn summarize_events(&self, events: &[EventRecord]) -> Vec<String> {
        events.iter()
            .take(10) // Limit to recent events
            .map(|e| format!("Tick {}: {:?}", e.tick, e.event))
            .collect()
    }
    
    /// Build context for world event narration
    pub async fn build_event_context(&self, event_type: &str) -> Result<EventContext> {
        Ok(EventContext {
            event_type: event_type.to_string(),
            world_state: "stable".to_string(), // TODO: Calculate from simulation
            affected_factions: vec![],
            time_of_occurrence: self.get_time_description(),
        })
    }
}

/// Rich context for room description generation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoomContext {
    pub room_details: RoomDetails,
    pub npcs_present: Vec<NpcInfo>,
    pub time_of_day: String,
    pub weather: String,
    pub recent_events: Vec<String>,
    pub ambient_conditions: String,
}

/// Context for NPC dialogue generation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DialogueContext {
    pub npc: NpcInfo,
    pub npc_memory: Vec<String>, // Past conversations
    pub npc_current_activity: String,
    pub npc_mood: String,
    pub player_reputation: i32,
    pub room_context: RoomContext,
    pub faction_relations: Vec<FactionRelation>,
    pub conversation_history: Vec<String>, // Recent conversation summaries
    pub relevant_events: Vec<String>, // Recent world events affecting this NPC
}

/// Faction relationship data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FactionRelation {
    pub faction_name: String,
    pub reputation: i32,
    pub standing: String, // "hostile", "neutral", "friendly", "allied"
}

/// Context for world event narration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventContext {
    pub event_type: String,
    pub world_state: String,
    pub affected_factions: Vec<String>,
    pub time_of_occurrence: String,
}

/// Guidelines for LLM context usage:
/// 
/// 1. **Room Descriptions**: Use RoomContext to generate atmospheric descriptions
///    - Time of day affects lighting and activity
///    - Weather affects mood and visibility
///    - NPCs present should be naturally mentioned
///    - Recent events provide narrative hooks
/// 
/// 2. **NPC Dialogue**: Use DialogueContext for consistent character voice
///    - Personality traits guide tone and word choice
///    - Memory ensures continuity across conversations
///    - Current activity explains NPC behavior
///    - Mood affects response style
///    - Reputation affects NPC attitude toward player
/// 
/// 3. **Event Narration**: Use EventContext for world-changing moments
///    - Event type determines narrative focus
///    - World state provides context
///    - Faction involvement adds political dimension
/// 
/// 4. **Critical Rule**: LLM generates NARRATIVE ONLY
///    - Never invent prices, stats, or mechanical values
///    - Never determine success/failure of actions
///    - Never modify world state directly
///    - Always query simulation for mechanical truth

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_context_assembler_creation() {
        let world = Arc::new(Mutex::new(GameWorld::new()));
        let assembler = ContextAssembler::new(world);
        
        // Test that we can create the assembler
        assert!(true);
    }

    #[test]
    fn test_ambient_conditions() {
        let world = Arc::new(Mutex::new(GameWorld::new()));
        let assembler = ContextAssembler::new(world);
        
        let room = RoomDetails {
            name: "Test Room".to_string(),
            description: "A test".to_string(),
            exits: vec![],
        };
        
        let conditions = assembler.calculate_ambient_conditions(&room, &[]);
        assert!(conditions.contains("quiet"));
    }
}
