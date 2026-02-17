use tauri::State;
use crate::simulation::world::{SharedWorld, RoomDetails, NpcInfo};

/// Custom error type for Tauri commands
#[derive(serde::Serialize)]
pub struct CommandError {
    message: String,
}

impl From<anyhow::Error> for CommandError {
    fn from(err: anyhow::Error) -> Self {
        CommandError {
            message: err.to_string(),
        }
    }
}

impl From<String> for CommandError {
    fn from(err: String) -> Self {
        CommandError {
            message: err,
        }
    }
}

/// Get the current room where the player is located
#[tauri::command]
pub async fn get_current_room(
    world: State<'_, SharedWorld>
) -> Result<RoomDetails, CommandError> {
    let mut world_lock = world.lock().await;
    
    let room_id = world_lock.get_player_room()
        .ok_or_else(|| anyhow::anyhow!("Player has no position"))?;
    
    let room_details = world_lock.get_room_details(room_id)
        .ok_or_else(|| anyhow::anyhow!("Room not found"))?;
    
    Ok(room_details)
}

/// Get NPCs in the current room
#[tauri::command]
pub async fn get_npcs_in_current_room(
    world: State<'_, SharedWorld>
) -> Result<Vec<NpcInfo>, CommandError> {
    let mut world_lock = world.lock().await;
    
    let room_id = world_lock.get_player_room()
        .ok_or_else(|| anyhow::anyhow!("Player has no position"))?;
    
    let npcs = world_lock.get_npcs_in_room(room_id);
    
    Ok(npcs)
}

/// Move player in a direction
#[tauri::command]
pub async fn move_player(
    direction: String,
    world: State<'_, SharedWorld>
) -> Result<RoomDetails, CommandError> {
    let mut world_lock = world.lock().await;
    
    let direction_lower = direction.trim().to_lowercase();
    
    // Attempt to move player
    let new_room_id = world_lock.move_player(&direction_lower)?;
    
    // Get the new room details
    let room_details = world_lock.get_room_details(new_room_id)
        .ok_or_else(|| anyhow::anyhow!("New room not found after movement"))?;
    
    Ok(room_details)
}

/// Process a player action/command
#[tauri::command]
pub async fn send_player_action(
    action: String,
    world: State<'_, SharedWorld>
) -> Result<String, CommandError> {
    let action_lower = action.trim().to_lowercase();
    
    // Check if it's a movement command and provide helpful error
    let movement_directions = ["north", "n", "south", "s", "east", "e", "west", "w", "up", "u", "down", "d"];
    if movement_directions.contains(&action_lower.as_str()) {
        return Err(CommandError {
            message: format!("Movement detected! Just type '{}' to move.", action_lower)
        });
    }
    
    let response = match action_lower.as_str() {
        "look" | "l" => {
            let mut world_lock = world.lock().await;
            let room_id = world_lock.get_player_room()
                .ok_or_else(|| anyhow::anyhow!("No current room"))?;
            let room = world_lock.get_room_details(room_id)
                .ok_or_else(|| anyhow::anyhow!("Room not found"))?;
            
            let npcs = world_lock.get_npcs_in_room(room_id);
            
            let mut response = format!("{}\n\n{}\n\nObvious exits: {}", 
                room.name,
                room.description,
                room.exits.iter()
                    .map(|e| e.direction.clone())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            
            if !npcs.is_empty() {
                response.push_str("\n\nYou see:");
                for npc in npcs {
                    response.push_str(&format!("\n  - {}", npc.name));
                }
            }
            
            response
        },
        "help" => {
            "Available commands:\n\
             - look (or l): Examine your surroundings\n\
             - north/south/east/west (n/s/e/w): Move in that direction\n\
             - up/down (u/d): Move up or down\n\
             - talk to [name]: Start a conversation\n\
             - help: Show this message".to_string()
        },
        _ if action_lower.starts_with("talk to") => {
            let mut world_lock = world.lock().await;
            let room_id = world_lock.get_player_room()
                .ok_or_else(|| anyhow::anyhow!("No current room"))?;
            let npcs = world_lock.get_npcs_in_room(room_id);
            
            if npcs.is_empty() {
                "There's nobody here to talk to.".to_string()
            } else {
                format!("{} looks up as you approach.\n\n\
                        [Full NPC dialogue powered by Claude coming in Phase 4]\n\n\
                        Present NPCs: {}", 
                    npcs[0].name,
                    npcs.iter().map(|n| n.name.as_str()).collect::<Vec<_>>().join(", ")
                )
            }
        },
        _ => {
            format!("You try to '{}', but nothing happens. Type 'help' for available commands.", action)
        }
    };
    
    Ok(response)
}

/// Get the current world tick count
#[tauri::command]
pub async fn get_world_tick(
    world: State<'_, SharedWorld>
) -> Result<u64, CommandError> {
    let world_lock = world.lock().await;
    Ok(world_lock.tick_count)
}
