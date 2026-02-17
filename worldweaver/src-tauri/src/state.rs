use crate::simulation::world::SharedWorld;

/// Shared application state accessible from all Tauri commands
pub struct AppState {
    pub world: SharedWorld,
}
