use bevy_ecs::world::World;
use bevy_ecs::schedule::Schedule;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use super::components::*;
use super::systems;
use super::events::{EventLog, GameEvent, EventRecord};

/// Main game world wrapper around Bevy ECS
pub struct GameWorld {
    pub ecs_world: World,
    pub schedule: Schedule,
    pub tick_count: u64,
    pub room_registry: HashMap<Uuid, String>,
}

impl GameWorld {
    /// Create a new game world with starter content
    pub fn new() -> Self {
        let mut world = World::new();
        
        // Initialize resources for systems
        world.insert_resource(systems::WorldClock::default());
        world.insert_resource(systems::WorldEvents::default());
        world.insert_resource(EventLog::default());
        
        // Build schedule with systems
        let mut schedule = Schedule::default();
        schedule.add_systems((
            systems::advance_world_clock,
            systems::update_npc_schedules,
            systems::cleanup_old_events,
        ));
        
        let room_registry = Self::spawn_starter_content(&mut world);
        
        Self { 
            ecs_world: world,
            schedule,
            tick_count: 0,
            room_registry,
        }
    }
    
    /// Execute one simulation tick
    pub fn tick(&mut self) {
        self.tick_count += 1;
        self.schedule.run(&mut self.ecs_world);
    }

    /// Spawn the initial world with multiple connected rooms
    fn spawn_starter_content(world: &mut World) -> HashMap<Uuid, String> {
        let mut registry = HashMap::new();
        
        // Create room IDs upfront so we can link them
        let inn_id = Uuid::new_v4();
        let square_id = Uuid::new_v4();
        let merchant_id = Uuid::new_v4();
        let forge_id = Uuid::new_v4();
        
        // Room 1: The Crossroads Inn (starting room)
        world.spawn((
            Name("The Crossroads Inn".to_string()),
            Description(
                "A cozy common room with worn wooden tables and a crackling fireplace. \
                The smell of roasted meat and ale fills the air. A burly innkeeper polishes \
                mugs behind the bar, and a few patrons sit in quiet conversation. \
                A heavy oak door leads north to the town square.".to_string()
            ),
            Room {
                exits: vec![
                    Exit {
                        direction: "north".to_string(),
                        target_room_id: square_id,
                        description: Some("A heavy oak door leads to the town square.".to_string()),
                    }
                ],
            },
            RoomId(inn_id),
            IsRoom,
        ));
        registry.insert(inn_id, "The Crossroads Inn".to_string());
        
        // Room 2: Town Square
        world.spawn((
            Name("Town Square".to_string()),
            Description(
                "A bustling open plaza paved with smooth cobblestones. In the center stands \
                a weathered stone fountain, its basin filled with clear water. Merchants \
                hawk their wares from colorful stalls around the perimeter. The Crossroads Inn \
                lies to the south, while the Merchant District sprawls to the east. You can \
                hear the distant ring of a hammer on anvil to the west.".to_string()
            ),
            Room {
                exits: vec![
                    Exit {
                        direction: "south".to_string(),
                        target_room_id: inn_id,
                        description: Some("The Crossroads Inn's entrance.".to_string()),
                    },
                    Exit {
                        direction: "east".to_string(),
                        target_room_id: merchant_id,
                        description: Some("A street lined with shops and market stalls.".to_string()),
                    },
                    Exit {
                        direction: "west".to_string(),
                        target_room_id: forge_id,
                        description: Some("Smoke rises from a sturdy stone building.".to_string()),
                    },
                ],
            },
            RoomId(square_id),
            IsRoom,
        ));
        registry.insert(square_id, "Town Square".to_string());
        
        // Room 3: Merchant District
        world.spawn((
            Name("Merchant District".to_string()),
            Description(
                "A narrow street crowded with shops and market stalls. Canvas awnings provide \
                shade from the afternoon sun. The air is thick with the scent of spices, \
                fresh bread, and tanned leather. Shopkeepers call out their daily specials \
                to passing customers. The town square lies to the west.".to_string()
            ),
            Room {
                exits: vec![
                    Exit {
                        direction: "west".to_string(),
                        target_room_id: square_id,
                        description: Some("The open town square.".to_string()),
                    },
                ],
            },
            RoomId(merchant_id),
            IsRoom,
        ));
        registry.insert(merchant_id, "Merchant District".to_string());
        
        // Room 4: Blacksmith's Forge
        world.spawn((
            Name("Blacksmith's Forge".to_string()),
            Description(
                "A sweltering workshop dominated by a roaring forge. Weapons and tools hang \
                from racks along the walls, and the air rings with the steady beat of hammer \
                on steel. A muscular woman works the bellows, her face streaked with soot. \
                Finished blades cool in a water trough, sending up plumes of steam. The town \
                square lies to the east.".to_string()
            ),
            Room {
                exits: vec![
                    Exit {
                        direction: "east".to_string(),
                        target_room_id: square_id,
                        description: Some("Back toward the town square.".to_string()),
                    },
                ],
            },
            RoomId(forge_id),
            IsRoom,
        ));
        registry.insert(forge_id, "Blacksmith's Forge".to_string());
        
        // NPC: Gareth the Innkeeper (in the Inn)
        world.spawn((
            Name("Gareth the Innkeeper".to_string()),
            Description(
                "A broad-shouldered man with graying hair and a welcoming smile. \
                His apron is stained from years of tavern work.".to_string()
            ),
            Position { room_id: inn_id },
            Npc {
                personality: "Friendly and talkative, knows all the local gossip. \
                             Protective of his establishment and regular customers.".to_string(),
                greeting: "Welcome to the Crossroads! What can I get you?".to_string(),
            },
            IsNpc,
        ));
        
        // NPC: Kael the Blacksmith (in the Forge)
        world.spawn((
            Name("Kael the Blacksmith".to_string()),
            Description(
                "A muscular woman with arms like tree trunks, her dark hair tied back \
                in a practical braid. Soot streaks her face and leather apron.".to_string()
            ),
            Position { room_id: forge_id },
            Npc {
                personality: "Direct and no-nonsense, but fair. Takes pride in her craft. \
                             Respects those who work hard and despises laziness.".to_string(),
                greeting: "Looking for quality steel? You've come to the right place.".to_string(),
            },
            IsNpc,
        ));
        
        // Create the player character in the starting room (Inn)
        world.spawn((
            Name("Traveler".to_string()),
            Description("A weary adventurer seeking rest and information.".to_string()),
            Position { room_id: inn_id },
            Player {
                current_input: String::new(),
                movement_history: vec![inn_id],
            },
            IsPlayer,
        ));

        println!("✓ Spawned world: 4 rooms, 2 NPCs, 1 player");
        println!("  - The Crossroads Inn (start)");
        println!("  - Town Square");
        println!("  - Merchant District");
        println!("  - Blacksmith's Forge");
        
        registry
    }

    /// Get the room ID where the player currently is
    pub fn get_player_room(&mut self) -> Option<Uuid> {
        let mut query = self.ecs_world.query_filtered::<&Position, bevy_ecs::query::With<IsPlayer>>();
        query.iter(&self.ecs_world).next().map(|pos| pos.room_id)
    }

    /// Get detailed information about a room by ID
    pub fn get_room_details(&mut self, room_id: Uuid) -> Option<RoomDetails> {
        let mut query = self.ecs_world.query_filtered::<(&RoomId, &Name, &Description, &Room), bevy_ecs::query::With<IsRoom>>();
        
        for (id, name, desc, room) in query.iter(&self.ecs_world) {
            if id.0 == room_id {
                return Some(RoomDetails {
                    id: room_id,
                    name: name.0.clone(),
                    description: desc.0.clone(),
                    exits: room.exits.clone(),
                });
            }
        }
        None
    }

    /// Get all NPCs in a specific room
    pub fn get_npcs_in_room(&mut self, room_id: Uuid) -> Vec<NpcInfo> {
        let mut query = self.ecs_world.query_filtered::<(&Name, &Description, &Position, &Npc), bevy_ecs::query::With<IsNpc>>();
        
        query.iter(&self.ecs_world)
            .filter(|(_, _, pos, _)| pos.room_id == room_id)
            .map(|(name, desc, _, npc)| NpcInfo {
                name: name.0.clone(),
                description: desc.0.clone(),
                personality: npc.personality.clone(),
                greeting: npc.greeting.clone(),
            })
            .collect()
    }
    
    /// Move player in a direction
    pub fn move_player(&mut self, direction: &str) -> Result<Uuid, String> {
        // Get current room
        let current_room_id = self.get_player_room()
            .ok_or_else(|| "Player has no current room".to_string())?;
        
        // Get room details to check exits
        let room = self.get_room_details(current_room_id)
            .ok_or_else(|| "Current room not found".to_string())?;
        
        // Find matching exit
        let exit = room.exits.iter()
            .find(|e| e.direction == direction)
            .ok_or_else(|| format!("You can't go {} from here.", direction))?;
        
        let target_room_id = exit.target_room_id;
        
        // Verify target room exists
        self.get_room_details(target_room_id)
            .ok_or_else(|| "Target room not found (world error)".to_string())?;
        
        // Record movement event first (before mutable borrow of query)
        if let Some(mut event_log) = self.ecs_world.get_resource_mut::<EventLog>() {
            event_log.record(
                self.tick_count,
                GameEvent::PlayerMoved {
                    from_room: current_room_id,
                    to_room: target_room_id,
                    direction: direction.to_string(),
                }
            );
        }
        
        // Update player position
        let mut query = self.ecs_world.query_filtered::<(&mut Position, &mut Player), bevy_ecs::query::With<IsPlayer>>();
        
        if let Some((mut pos, mut player)) = query.iter_mut(&mut self.ecs_world).next() {
            pos.room_id = target_room_id;
            player.movement_history.push(target_room_id);
            Ok(target_room_id)
        } else {
            Err("Player entity not found".to_string())
        }
    }
    
    /// Get player's movement history
    pub fn get_movement_history(&mut self) -> Vec<Uuid> {
        let mut query = self.ecs_world.query_filtered::<&Player, bevy_ecs::query::With<IsPlayer>>();
        
        query.iter(&self.ecs_world)
            .next()
            .map(|player| player.movement_history.clone())
            .unwrap_or_default()
    }
    
    /// Query events by tag
    pub fn query_events_by_tag(&self, tag: &str, limit: usize) -> Vec<EventRecord> {
        if let Some(event_log) = self.ecs_world.get_resource::<EventLog>() {
            event_log.query_by_tag(tag, limit)
                .into_iter()
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Query events in a specific room
    pub fn query_events_in_room(&self, room_id: Uuid, limit: usize) -> Vec<EventRecord> {
        if let Some(event_log) = self.ecs_world.get_resource::<EventLog>() {
            event_log.query_in_room(room_id, limit)
                .into_iter()
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get all events since a specific tick
    pub fn get_events_since(&self, tick: u64) -> Vec<EventRecord> {
        if let Some(event_log) = self.ecs_world.get_resource::<EventLog>() {
            event_log.query_since_tick(tick)
                .into_iter()
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
}

/// Serializable room details for sending to frontend
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoomDetails {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub exits: Vec<Exit>,
}

/// Serializable NPC info for sending to frontend
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NpcInfo {
    pub name: String,
    pub description: String,
    pub personality: String,
    pub greeting: String,
}

/// Thread-safe shared reference to the game world
pub type SharedWorld = Arc<Mutex<GameWorld>>;

/// Create a new shared game world instance
pub fn create_shared_world() -> SharedWorld {
    Arc::new(Mutex::new(GameWorld::new()))
}
