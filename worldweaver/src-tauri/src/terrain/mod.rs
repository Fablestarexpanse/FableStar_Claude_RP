use bevy_ecs::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet, VecDeque};

pub mod config;
pub mod heightmap;
pub mod noise_gen;
pub mod erosion;
pub mod hydrology;
pub mod rivers;
pub mod biomes;
pub mod roads;
pub mod persistence;
pub mod brush;
pub mod commands;

use config::TerrainConfig;
use heightmap::HeightmapChunk;
use rivers::RiverNetwork;
use biomes::BiomeRegistry;

/// Water source for hydrology simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterSource {
    pub x: usize,
    pub y: usize,
    pub flow_rate: f32,  // Amount of water per step
    pub active: bool,
}

/// Main terrain data resource for bevy_ecs
#[derive(Resource)]
pub struct TerrainData {
    pub config: TerrainConfig,
    pub chunks: HashMap<(i32, i32), HeightmapChunk>,
    pub dirty_chunks: HashSet<(i32, i32)>,
    pub river_network: RiverNetwork,
    pub biome_definitions: BiomeRegistry,
    pub undo_stack: UndoStack,
    pub water_sources: Vec<WaterSource>,
}

impl Default for TerrainData {
    fn default() -> Self {
        Self {
            config: TerrainConfig::default(),
            chunks: HashMap::new(),
            dirty_chunks: HashSet::new(),
            river_network: RiverNetwork::new(),
            biome_definitions: BiomeRegistry::new(),
            undo_stack: UndoStack::new(),
            water_sources: Vec::new(),
        }
    }
}

impl TerrainData {
    pub fn new(config: TerrainConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    /// Sample height at world coordinates
    pub fn sample_height(&self, world_x: f32, world_z: f32) -> Option<f32> {
        let (chunk_x, chunk_z) = self.config.world_to_chunk(world_x, world_z);
        let chunk = self.chunks.get(&(chunk_x, chunk_z))?;

        let chunk_world_x = chunk_x as f32 * self.config.chunk_size as f32 * self.config.cell_size_meters;
        let chunk_world_z = chunk_z as f32 * self.config.chunk_size as f32 * self.config.cell_size_meters;

        let local_x = (world_x - chunk_world_x) / self.config.cell_size_meters;
        let local_z = (world_z - chunk_world_z) / self.config.cell_size_meters;

        Some(chunk.sample_bilinear(local_x, local_z, self.config.vertex_count))
    }

    /// Get chunk at coordinates
    pub fn get_chunk(&self, chunk_x: i32, chunk_z: i32) -> Option<&HeightmapChunk> {
        self.chunks.get(&(chunk_x, chunk_z))
    }

    /// Get mutable chunk at coordinates
    pub fn get_chunk_mut(&mut self, chunk_x: i32, chunk_z: i32) -> Option<&mut HeightmapChunk> {
        self.chunks.get_mut(&(chunk_x, chunk_z))
    }

    /// Mark chunk as dirty
    pub fn mark_dirty(&mut self, chunk_x: i32, chunk_z: i32) {
        self.dirty_chunks.insert((chunk_x, chunk_z));
    }

    /// Clear all dirty flags
    pub fn clear_dirty(&mut self) {
        self.dirty_chunks.clear();
    }
}

/// Undo/redo system using XOR deltas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoEntry {
    pub chunk_coord: (i32, i32),
    pub delta: Vec<u8>,                    // zstd-compressed XOR
    pub affected_rect: (u16, u16, u16, u16),
    pub group_id: u64,                     // groups multi-chunk strokes
}

pub struct UndoStack {
    entries: VecDeque<UndoEntry>,
    current_group: u64,
    max_entries: usize,
}

impl UndoStack {
    pub fn new() -> Self {
        Self {
            entries: VecDeque::new(),
            current_group: 0,
            max_entries: 1000,
        }
    }

    /// Start a new undo group
    pub fn begin_group(&mut self) {
        self.current_group += 1;
    }

    /// Record an undo entry
    pub fn record(&mut self, chunk: &HeightmapChunk, before: &[f32]) {
        if chunk.heights.len() != before.len() {
            return;
        }

        // Create XOR delta
        let xor_delta: Vec<u8> = chunk.heights.iter()
            .zip(before.iter())
            .flat_map(|(a, b)| (a.to_bits() ^ b.to_bits()).to_le_bytes())
            .collect();

        // Compress delta
        if let Ok(compressed) = zstd::encode_all(&xor_delta[..], 3) {
            let entry = UndoEntry {
                chunk_coord: chunk.coord,
                delta: compressed,
                affected_rect: (0, 0, 0, 0), // TODO: Calculate actual rect
                group_id: self.current_group,
            };

            self.entries.push_back(entry);

            // Limit size
            while self.entries.len() > self.max_entries {
                self.entries.pop_front();
            }
        }
    }

    /// Undo last operation
    pub fn undo(&mut self, terrain: &mut TerrainData) -> bool {
        if let Some(entry) = self.entries.pop_back() {
            self.apply_delta(terrain, &entry);
            true
        } else {
            false
        }
    }

    /// Apply XOR delta to terrain
    fn apply_delta(&self, terrain: &mut TerrainData, entry: &UndoEntry) {
        if let Some(chunk) = terrain.get_chunk_mut(entry.chunk_coord.0, entry.chunk_coord.1) {
            if let Ok(xor_delta) = zstd::decode_all(&entry.delta[..]) {
                let xor_values: Vec<u32> = xor_delta
                    .chunks_exact(4)
                    .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                    .collect();

                for (i, xor_val) in xor_values.iter().enumerate() {
                    if i < chunk.heights.len() {
                        let current_bits = chunk.heights[i].to_bits();
                        let new_bits = current_bits ^ xor_val;
                        chunk.heights[i] = f32::from_bits(new_bits);
                    }
                }

                terrain.mark_dirty(entry.chunk_coord.0, entry.chunk_coord.1);
            }
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.entries.is_empty()
    }
}

impl Default for UndoStack {
    fn default() -> Self {
        Self::new()
    }
}

/// Bevy system to sync terrain with room entities
pub fn sync_terrain_rooms(
    mut rooms: Query<(&crate::simulation::components::RoomId, &mut crate::simulation::components::RoomTerrainBinding)>,
    terrain: Res<TerrainData>,
) {
    for (_room_id, mut binding) in rooms.iter_mut() {
        let (chunk_x, chunk_z) = binding.chunk_coord;
        if let Some(chunk) = terrain.chunks.get(&(chunk_x, chunk_z)) {
            let local_x = binding.world_x % (terrain.config.chunk_size as f32 * terrain.config.cell_size_meters);
            let local_z = binding.world_z % (terrain.config.chunk_size as f32 * terrain.config.cell_size_meters);
            let local_x = local_x / terrain.config.cell_size_meters;
            let local_z = local_z / terrain.config.cell_size_meters;
            binding.elevation = chunk.sample_bilinear(local_x, local_z, terrain.config.vertex_count);
        }
    }
}
