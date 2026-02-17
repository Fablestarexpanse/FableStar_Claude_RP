use bevy_ecs::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Simple name component for any entity
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Name(pub String);

/// Description component for narrative text
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Description(pub String);

/// Position in the world - which room an entity is in
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Position {
    pub room_id: Uuid,
}

/// Room ID component for identifying rooms
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct RoomId(pub Uuid);

/// Player-specific data
#[derive(Component, Serialize, Deserialize, Debug)]
pub struct Player {
    pub current_input: String,
    pub movement_history: Vec<Uuid>,
}

/// Room structure with exits to other rooms
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Room {
    pub exits: Vec<Exit>,
}

/// Exit connection between rooms
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Exit {
    pub direction: String,
    pub target_room_id: Uuid,
    pub description: Option<String>,
}

/// NPC-specific data
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Npc {
    pub personality: String,
    pub greeting: String,
}

// Tag components for querying specific entity types
#[derive(Component)]
pub struct IsRoom;

#[derive(Component)]
pub struct IsPlayer;

#[derive(Component)]
pub struct IsNpc;

/// Terrain binding for rooms - links room to world map position
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct RoomTerrainBinding {
    pub world_x: f32,        // meters from origin
    pub world_z: f32,
    pub chunk_coord: (i32, i32),
    pub elevation: f32,
    pub biome: Option<String>,
}

// ============================================================================
// RPG STATS & SKILLS
// ============================================================================

/// Core character stats
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Stats {
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub charisma: i32,
    pub constitution: i32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            strength: 10,
            dexterity: 10,
            intelligence: 10,
            charisma: 10,
            constitution: 10,
        }
    }
}

/// Skill-based progression (skill name -> level 0-100)
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Skills {
    pub skills: HashMap<String, i32>,
}

impl Skills {
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
        }
    }
    
    pub fn get_skill(&self, skill: &str) -> i32 {
        *self.skills.get(skill).unwrap_or(&0)
    }
    
    pub fn improve_skill(&mut self, skill: &str, amount: i32) {
        let current = self.get_skill(skill);
        self.skills.insert(skill.to_string(), (current + amount).min(100));
    }
}

impl Default for Skills {
    fn default() -> Self {
        Self::new()
    }
}

/// Health points
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }
    
    pub fn is_alive(&self) -> bool {
        self.current > 0
    }
    
    pub fn heal(&mut self, amount: i32) {
        self.current = (self.current + amount).min(self.max);
    }
    
    pub fn damage(&mut self, amount: i32) {
        self.current = (self.current - amount).max(0);
    }
}

// ============================================================================
// NPC SCHEDULE SYSTEM (Priority-queue fallthrough pattern)
// ============================================================================

/// NPC schedule with priority-based behavior packages
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Schedule {
    pub packages: Vec<SchedulePackage>,
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
        }
    }
    
    /// Get the highest priority package that matches current conditions
    pub fn get_active_package(&self, hour: u32, player_nearby: bool) -> Option<&SchedulePackage> {
        self.packages.iter()
            .filter(|pkg| pkg.condition.matches(hour, player_nearby))
            .max_by_key(|pkg| pkg.priority)
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SchedulePackage {
    pub priority: i32,
    pub condition: ScheduleCondition,
    pub action: ScheduleAction,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ScheduleCondition {
    TimeRange { start_hour: u32, end_hour: u32 },
    Always,
    PlayerNearby,
}

impl ScheduleCondition {
    pub fn matches(&self, hour: u32, player_nearby: bool) -> bool {
        match self {
            ScheduleCondition::TimeRange { start_hour, end_hour } => {
                if start_hour <= end_hour {
                    hour >= *start_hour && hour < *end_hour
                } else {
                    // Wraps around midnight
                    hour >= *start_hour || hour < *end_hour
                }
            },
            ScheduleCondition::Always => true,
            ScheduleCondition::PlayerNearby => player_nearby,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ScheduleAction {
    StayInRoom { room_id: Uuid },
    MoveToRoom { room_id: Uuid },
    PerformActivity { activity: String },
}

// ============================================================================
// INVENTORY & ITEMS
// ============================================================================

/// Entity's inventory
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Inventory {
    pub items: Vec<Uuid>,
    pub capacity: usize,
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::new(),
            capacity,
        }
    }
    
    pub fn is_full(&self) -> bool {
        self.items.len() >= self.capacity
    }
    
    pub fn add_item(&mut self, item_id: Uuid) -> Result<(), String> {
        if self.is_full() {
            Err("Inventory is full".to_string())
        } else {
            self.items.push(item_id);
            Ok(())
        }
    }
    
    pub fn remove_item(&mut self, item_id: Uuid) -> bool {
        if let Some(pos) = self.items.iter().position(|&id| id == item_id) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new(20)
    }
}

/// Item properties
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub item_type: String,
    pub weight: f32,
    pub value: i32,
    pub stackable: bool,
    pub stack_count: u32,
}

impl Item {
    pub fn new(item_type: String, weight: f32, value: i32) -> Self {
        Self {
            item_type,
            weight,
            value,
            stackable: false,
            stack_count: 1,
        }
    }
}

// ============================================================================
// RELATIONSHIPS & MEMORY
// ============================================================================

/// Entity's relationships with other entities
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Relationships {
    pub relations: HashMap<Uuid, RelationshipData>,
}

impl Relationships {
    pub fn new() -> Self {
        Self {
            relations: HashMap::new(),
        }
    }
    
    pub fn get_affinity(&self, entity_id: Uuid) -> i32 {
        self.relations.get(&entity_id)
            .map(|r| r.affinity)
            .unwrap_or(0)
    }
    
    pub fn modify_affinity(&mut self, entity_id: Uuid, change: i32, tick: u64) {
        let relation = self.relations.entry(entity_id).or_insert(RelationshipData {
            affinity: 0,
            trust: 0,
            last_interaction_tick: tick,
        });
        relation.affinity = (relation.affinity + change).clamp(-100, 100);
        relation.last_interaction_tick = tick;
    }
}

impl Default for Relationships {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelationshipData {
    pub affinity: i32,  // -100 to 100
    pub trust: i32,     // 0 to 100
    pub last_interaction_tick: u64,
}

/// NPC dialogue memory
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct DialogueMemory {
    pub conversations: Vec<ConversationRecord>,
    pub max_memories: usize,
}

impl DialogueMemory {
    pub fn new(max_memories: usize) -> Self {
        Self {
            conversations: Vec::new(),
            max_memories,
        }
    }
    
    pub fn add_conversation(&mut self, with_entity: Uuid, tick: u64, summary: String, topics: Vec<String>) {
        self.conversations.push(ConversationRecord {
            with_entity,
            tick,
            summary,
            topics,
        });
        
        // Keep only the most recent memories
        if self.conversations.len() > self.max_memories {
            self.conversations.remove(0);
        }
    }
    
    pub fn get_recent_conversations(&self, with_entity: Uuid, limit: usize) -> Vec<&ConversationRecord> {
        self.conversations.iter()
            .rev()
            .filter(|c| c.with_entity == with_entity)
            .take(limit)
            .collect()
    }
}

impl Default for DialogueMemory {
    fn default() -> Self {
        Self::new(20)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConversationRecord {
    pub with_entity: Uuid,
    pub tick: u64,
    pub summary: String,
    pub topics: Vec<String>,
}

// ============================================================================
// FACTIONS
// ============================================================================

/// Entity's faction membership
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct FactionMembership {
    pub faction_id: Uuid,
    pub rank: String,
    pub reputation: i32,
}

impl FactionMembership {
    pub fn new(faction_id: Uuid, rank: String) -> Self {
        Self {
            faction_id,
            rank,
            reputation: 0,
        }
    }
}

/// Faction entity
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Faction {
    pub name: String,
    pub relations: HashMap<Uuid, i32>,  // faction_id -> relation (-100 to 100)
}

impl Faction {
    pub fn new(name: String) -> Self {
        Self {
            name,
            relations: HashMap::new(),
        }
    }
    
    pub fn get_relation(&self, faction_id: Uuid) -> i32 {
        *self.relations.get(&faction_id).unwrap_or(&0)
    }
    
    pub fn set_relation(&mut self, faction_id: Uuid, value: i32) {
        self.relations.insert(faction_id, value.clamp(-100, 100));
    }
}
