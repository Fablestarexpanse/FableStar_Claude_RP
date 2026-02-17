use bevy_ecs::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// All possible game events that can occur in the world
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GameEvent {
    // Movement
    PlayerMoved { from_room: Uuid, to_room: Uuid, direction: String },
    NpcMoved { npc_id: Uuid, from_room: Uuid, to_room: Uuid },
    
    // Interaction
    PlayerTalkedToNpc { npc_id: Uuid, room_id: Uuid },
    ItemPickedUp { item_id: Uuid, player_id: Uuid },
    ItemDropped { item_id: Uuid, room_id: Uuid },
    
    // Combat
    CombatStarted { attacker: Uuid, defender: Uuid },
    CombatResolved { winner: Uuid, loser: Uuid, damage: i32 },
    
    // World State
    TimeAdvanced { old_hour: u32, new_hour: u32, day: u32 },
    WeatherChanged { old_weather: String, new_weather: String },
    
    // Economy
    ItemCrafted { crafter: Uuid, item_id: Uuid, recipe: String },
    ItemSold { seller: Uuid, buyer: Uuid, item_id: Uuid, price: i32 },
    
    // Factions
    FactionRelationChanged { faction_a: Uuid, faction_b: Uuid, old_value: i32, new_value: i32 },
    PlayerReputationChanged { faction: Uuid, old_rep: i32, new_rep: i32 },
}

impl GameEvent {
    /// Get the event type as a string for database storage
    pub fn event_type(&self) -> &'static str {
        match self {
            GameEvent::PlayerMoved { .. } => "player_moved",
            GameEvent::NpcMoved { .. } => "npc_moved",
            GameEvent::PlayerTalkedToNpc { .. } => "player_talked_to_npc",
            GameEvent::ItemPickedUp { .. } => "item_picked_up",
            GameEvent::ItemDropped { .. } => "item_dropped",
            GameEvent::CombatStarted { .. } => "combat_started",
            GameEvent::CombatResolved { .. } => "combat_resolved",
            GameEvent::TimeAdvanced { .. } => "time_advanced",
            GameEvent::WeatherChanged { .. } => "weather_changed",
            GameEvent::ItemCrafted { .. } => "item_crafted",
            GameEvent::ItemSold { .. } => "item_sold",
            GameEvent::FactionRelationChanged { .. } => "faction_relation_changed",
            GameEvent::PlayerReputationChanged { .. } => "player_reputation_changed",
        }
    }
}

/// A recorded event with metadata
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventRecord {
    pub id: Uuid,
    pub tick: u64,
    pub timestamp: DateTime<Utc>,
    pub event: GameEvent,
    pub tags: Vec<String>,
}

/// Event log that tracks all world events
#[derive(Resource)]
pub struct EventLog {
    events: Vec<EventRecord>,
}

impl EventLog {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }
    
    /// Record a new event
    pub fn record(&mut self, tick: u64, event: GameEvent) -> Uuid {
        let id = Uuid::new_v4();
        let tags = Self::generate_tags(&event);
        
        self.events.push(EventRecord {
            id,
            tick,
            timestamp: Utc::now(),
            event,
            tags,
        });
        
        id
    }
    
    /// Query events by tag
    pub fn query_by_tag(&self, tag: &str, limit: usize) -> Vec<&EventRecord> {
        self.events.iter()
            .rev()
            .filter(|e| e.tags.contains(&tag.to_string()))
            .take(limit)
            .collect()
    }
    
    /// Query events since a specific tick
    pub fn query_since_tick(&self, tick: u64) -> Vec<&EventRecord> {
        self.events.iter()
            .filter(|e| e.tick >= tick)
            .collect()
    }
    
    /// Query events in a specific room
    pub fn query_in_room(&self, room_id: Uuid, limit: usize) -> Vec<&EventRecord> {
        self.events.iter()
            .rev()
            .filter(|e| {
                match &e.event {
                    GameEvent::PlayerMoved { to_room, .. } => *to_room == room_id,
                    GameEvent::NpcMoved { to_room, .. } => *to_room == room_id,
                    GameEvent::PlayerTalkedToNpc { room_id: r, .. } => *r == room_id,
                    GameEvent::ItemDropped { room_id: r, .. } => *r == room_id,
                    _ => false,
                }
            })
            .take(limit)
            .collect()
    }
    
    /// Get all events (for persistence)
    pub fn all_events(&self) -> &[EventRecord] {
        &self.events
    }
    
    /// Generate tags for an event for efficient querying
    fn generate_tags(event: &GameEvent) -> Vec<String> {
        match event {
            GameEvent::PlayerMoved { .. } => vec!["player".into(), "movement".into()],
            GameEvent::NpcMoved { npc_id, .. } => {
                vec!["npc".into(), "movement".into(), format!("npc:{}", npc_id)]
            },
            GameEvent::PlayerTalkedToNpc { npc_id, .. } => {
                vec!["player".into(), "dialogue".into(), format!("npc:{}", npc_id)]
            },
            GameEvent::ItemPickedUp { player_id, .. } => {
                vec!["player".into(), "item".into(), format!("player:{}", player_id)]
            },
            GameEvent::ItemDropped { .. } => {
                vec!["item".into(), "movement".into()]
            },
            GameEvent::CombatStarted { attacker, defender } => {
                vec![
                    "combat".into(),
                    format!("entity:{}", attacker),
                    format!("entity:{}", defender),
                ]
            },
            GameEvent::CombatResolved { winner, loser, .. } => {
                vec![
                    "combat".into(),
                    "player".into(),
                    format!("winner:{}", winner),
                    format!("loser:{}", loser),
                ]
            },
            GameEvent::TimeAdvanced { .. } => {
                vec!["world".into(), "time".into()]
            },
            GameEvent::WeatherChanged { .. } => {
                vec!["world".into(), "weather".into()]
            },
            GameEvent::ItemCrafted { crafter, .. } => {
                vec!["crafting".into(), "economy".into(), format!("crafter:{}", crafter)]
            },
            GameEvent::ItemSold { seller, buyer, .. } => {
                vec![
                    "economy".into(),
                    "trade".into(),
                    format!("seller:{}", seller),
                    format!("buyer:{}", buyer),
                ]
            },
            GameEvent::FactionRelationChanged { faction_a, faction_b, .. } => {
                vec![
                    "faction".into(),
                    format!("faction:{}", faction_a),
                    format!("faction:{}", faction_b),
                ]
            },
            GameEvent::PlayerReputationChanged { faction, .. } => {
                vec!["player".into(), "faction".into(), format!("faction:{}", faction)]
            },
        }
    }
}

impl Default for EventLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_recording() {
        let mut log = EventLog::new();
        
        let event = GameEvent::PlayerMoved {
            from_room: Uuid::new_v4(),
            to_room: Uuid::new_v4(),
            direction: "north".to_string(),
        };
        
        let event_id = log.record(1, event);
        
        assert!(log.all_events().len() == 1);
        assert_eq!(log.all_events()[0].id, event_id);
    }
    
    #[test]
    fn test_event_query_by_tag() {
        let mut log = EventLog::new();
        
        log.record(1, GameEvent::PlayerMoved {
            from_room: Uuid::new_v4(),
            to_room: Uuid::new_v4(),
            direction: "north".to_string(),
        });
        
        log.record(2, GameEvent::TimeAdvanced {
            old_hour: 10,
            new_hour: 11,
            day: 1,
        });
        
        let player_events = log.query_by_tag("player", 10);
        assert_eq!(player_events.len(), 1);
        
        let world_events = log.query_by_tag("world", 10);
        assert_eq!(world_events.len(), 1);
    }
    
    #[test]
    fn test_event_query_since_tick() {
        let mut log = EventLog::new();
        
        log.record(1, GameEvent::PlayerMoved {
            from_room: Uuid::new_v4(),
            to_room: Uuid::new_v4(),
            direction: "north".to_string(),
        });
        
        log.record(5, GameEvent::PlayerMoved {
            from_room: Uuid::new_v4(),
            to_room: Uuid::new_v4(),
            direction: "south".to_string(),
        });
        
        let recent_events = log.query_since_tick(3);
        assert_eq!(recent_events.len(), 1);
        assert_eq!(recent_events[0].tick, 5);
    }
}
