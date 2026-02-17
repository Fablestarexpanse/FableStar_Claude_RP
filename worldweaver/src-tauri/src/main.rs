// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod simulation;
mod commands;
mod state;
mod database;
mod terrain;

use simulation::world::create_shared_world;
use tokio::sync::Mutex;
use terrain::TerrainData;

fn main() {
    // Initialize the game world with starter content
    let world = create_shared_world();
    
    // Initialize terrain data
    let terrain = Mutex::new(TerrainData::default());
    
    println!("🌍 WorldWeaver starting...");
    
    tauri::Builder::default()
        .manage(world)
        .manage(terrain)
        .invoke_handler(tauri::generate_handler![
            commands::get_current_room,
            commands::get_npcs_in_current_room,
            commands::move_player,
            commands::send_player_action,
            commands::get_world_tick,
            terrain::commands::generate_terrain,
            terrain::commands::get_chunk,
            terrain::commands::apply_brush,
            terrain::commands::get_terrain_config,
            terrain::commands::get_rivers,
            terrain::commands::save_terrain,
            terrain::commands::load_terrain,
            terrain::commands::apply_weathering,
            terrain::commands::place_water_sources,
            terrain::commands::simulate_hydrology,
            terrain::commands::get_flow_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
