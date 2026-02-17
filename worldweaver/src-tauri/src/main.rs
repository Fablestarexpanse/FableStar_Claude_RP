// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod simulation;
mod commands;
mod state;
mod database;

use simulation::world::create_shared_world;

fn main() {
    // Initialize the game world with starter content
    let world = create_shared_world();
    
    println!("🌍 WorldWeaver starting...");
    
    tauri::Builder::default()
        .manage(world)
        .invoke_handler(tauri::generate_handler![
            commands::get_current_room,
            commands::get_npcs_in_current_room,
            commands::move_player,
            commands::send_player_action,
            commands::get_world_tick,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
