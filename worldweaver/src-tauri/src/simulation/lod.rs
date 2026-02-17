use uuid::Uuid;
use std::collections::HashMap;

/// Simulation detail level based on distance from player
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationDetail {
    Full,        // Player's room - every tick, full AI, real-time LLM
    Reduced,     // Adjacent rooms - every 10 ticks, simplified AI
    Abstract,    // Same region - every 100 ticks, schedule-only
    Statistical, // Distant - every 1000 ticks or on-demand
}

/// Room graph for distance calculations
pub struct RoomGraph {
    adjacency: HashMap<Uuid, Vec<Uuid>>,
    regions: HashMap<Uuid, Uuid>,  // room_id -> region_id
}

impl RoomGraph {
    pub fn new() -> Self {
        Self {
            adjacency: HashMap::new(),
            regions: HashMap::new(),
        }
    }
    
    /// Add a bidirectional connection between rooms
    pub fn add_connection(&mut self, room_a: Uuid, room_b: Uuid) {
        self.adjacency.entry(room_a).or_insert_with(Vec::new).push(room_b);
        self.adjacency.entry(room_b).or_insert_with(Vec::new).push(room_a);
    }
    
    /// Set the region for a room
    pub fn set_region(&mut self, room_id: Uuid, region_id: Uuid) {
        self.regions.insert(room_id, region_id);
    }
    
    /// Check if two rooms are adjacent (directly connected)
    pub fn is_adjacent(&self, room_a: Uuid, room_b: Uuid) -> bool {
        self.adjacency.get(&room_a)
            .map(|neighbors| neighbors.contains(&room_b))
            .unwrap_or(false)
    }
    
    /// Check if two rooms are in the same region
    pub fn same_region(&self, room_a: Uuid, room_b: Uuid) -> bool {
        match (self.regions.get(&room_a), self.regions.get(&room_b)) {
            (Some(region_a), Some(region_b)) => region_a == region_b,
            _ => false,
        }
    }
    
    /// Get all adjacent rooms
    pub fn get_adjacent_rooms(&self, room_id: Uuid) -> Vec<Uuid> {
        self.adjacency.get(&room_id)
            .cloned()
            .unwrap_or_default()
    }
}

impl Default for RoomGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Manages simulation level of detail based on distance from player
pub struct LodManager {
    player_room: Uuid,
    room_graph: RoomGraph,
}

impl LodManager {
    pub fn new(player_room: Uuid) -> Self {
        Self {
            player_room,
            room_graph: RoomGraph::new(),
        }
    }
    
    /// Update the player's current room
    pub fn update_player_room(&mut self, room_id: Uuid) {
        self.player_room = room_id;
    }
    
    /// Get mutable access to the room graph
    pub fn room_graph_mut(&mut self) -> &mut RoomGraph {
        &mut self.room_graph
    }
    
    /// Get immutable access to the room graph
    pub fn room_graph(&self) -> &RoomGraph {
        &self.room_graph
    }
    
    /// Determine the simulation detail level for an NPC's room
    pub fn determine_lod(&self, npc_room: Uuid) -> SimulationDetail {
        if npc_room == self.player_room {
            SimulationDetail::Full
        } else if self.room_graph.is_adjacent(self.player_room, npc_room) {
            SimulationDetail::Reduced
        } else if self.room_graph.same_region(self.player_room, npc_room) {
            SimulationDetail::Abstract
        } else {
            SimulationDetail::Statistical
        }
    }
    
    /// Check if an NPC should be simulated this tick based on LOD
    /// Uses staggered updates to spread computational load
    pub fn should_simulate_npc(&self, tick: u64, npc_id: Uuid, detail: SimulationDetail) -> bool {
        match detail {
            SimulationDetail::Full => true,
            SimulationDetail::Reduced => {
                // Every 10 ticks, staggered by NPC ID
                tick % 10 == (npc_id.as_u128() % 10) as u64
            },
            SimulationDetail::Abstract => {
                // Every 100 ticks, staggered by NPC ID
                tick % 100 == (npc_id.as_u128() % 100) as u64
            },
            SimulationDetail::Statistical => {
                // Every 1000 ticks, staggered by NPC ID
                tick % 1000 == (npc_id.as_u128() % 1000) as u64
            },
        }
    }
    
    /// Get all rooms that should be simulated at full detail
    pub fn get_full_detail_rooms(&self) -> Vec<Uuid> {
        vec![self.player_room]
    }
    
    /// Get all rooms that should be simulated at reduced detail
    pub fn get_reduced_detail_rooms(&self) -> Vec<Uuid> {
        self.room_graph.get_adjacent_rooms(self.player_room)
    }
    
    /// Get statistics about current LOD distribution
    pub fn get_lod_stats(&self, all_rooms: &[Uuid]) -> LodStats {
        let mut stats = LodStats::default();
        
        for &room_id in all_rooms {
            match self.determine_lod(room_id) {
                SimulationDetail::Full => stats.full += 1,
                SimulationDetail::Reduced => stats.reduced += 1,
                SimulationDetail::Abstract => stats.abstract_detail += 1,
                SimulationDetail::Statistical => stats.statistical += 1,
            }
        }
        
        stats
    }
}

/// Statistics about LOD distribution
#[derive(Debug, Default, Clone)]
pub struct LodStats {
    pub full: usize,
    pub reduced: usize,
    pub abstract_detail: usize,
    pub statistical: usize,
}

impl LodStats {
    pub fn total(&self) -> usize {
        self.full + self.reduced + self.abstract_detail + self.statistical
    }
    
    pub fn active_simulation_count(&self) -> usize {
        self.full + self.reduced
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_graph_adjacency() {
        let mut graph = RoomGraph::new();
        let room_a = Uuid::new_v4();
        let room_b = Uuid::new_v4();
        let room_c = Uuid::new_v4();
        
        graph.add_connection(room_a, room_b);
        
        assert!(graph.is_adjacent(room_a, room_b));
        assert!(graph.is_adjacent(room_b, room_a));
        assert!(!graph.is_adjacent(room_a, room_c));
    }
    
    #[test]
    fn test_room_graph_regions() {
        let mut graph = RoomGraph::new();
        let room_a = Uuid::new_v4();
        let room_b = Uuid::new_v4();
        let room_c = Uuid::new_v4();
        let region_1 = Uuid::new_v4();
        let region_2 = Uuid::new_v4();
        
        graph.set_region(room_a, region_1);
        graph.set_region(room_b, region_1);
        graph.set_region(room_c, region_2);
        
        assert!(graph.same_region(room_a, room_b));
        assert!(!graph.same_region(room_a, room_c));
    }
    
    #[test]
    fn test_lod_determination() {
        let player_room = Uuid::new_v4();
        let adjacent_room = Uuid::new_v4();
        let distant_room = Uuid::new_v4();
        
        let mut lod = LodManager::new(player_room);
        lod.room_graph_mut().add_connection(player_room, adjacent_room);
        
        assert_eq!(lod.determine_lod(player_room), SimulationDetail::Full);
        assert_eq!(lod.determine_lod(adjacent_room), SimulationDetail::Reduced);
        assert_eq!(lod.determine_lod(distant_room), SimulationDetail::Statistical);
    }
    
    #[test]
    fn test_npc_simulation_staggering() {
        let player_room = Uuid::new_v4();
        let lod = LodManager::new(player_room);
        
        let npc_id = Uuid::new_v4();
        
        // Full detail should always simulate
        assert!(lod.should_simulate_npc(0, npc_id, SimulationDetail::Full));
        assert!(lod.should_simulate_npc(1, npc_id, SimulationDetail::Full));
        
        // Reduced detail should simulate every 10 ticks (staggered)
        let reduced_count = (0..100)
            .filter(|&tick| lod.should_simulate_npc(tick, npc_id, SimulationDetail::Reduced))
            .count();
        assert_eq!(reduced_count, 10);
        
        // Abstract detail should simulate every 100 ticks (staggered)
        let abstract_count = (0..1000)
            .filter(|&tick| lod.should_simulate_npc(tick, npc_id, SimulationDetail::Abstract))
            .count();
        assert_eq!(abstract_count, 10);
    }
}
