use bevy_ecs::prelude::*;
use serde::{Serialize, Deserialize};

/// World theme for biome naming and styling
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WorldTheme {
    Fantasy,
    Modern,
    SciFi,
}

impl Default for WorldTheme {
    fn default() -> Self {
        WorldTheme::Fantasy
    }
}

/// Terrain configuration - bevy_ecs Component
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct TerrainConfig {
    pub chunk_size: u32,          // 128
    pub vertex_count: u32,        // 129 (chunk_size + 1)
    pub world_width: u32,         // e.g., 4096
    pub world_height: u32,        // e.g., 4096
    pub cell_size_meters: f32,    // 100.0 → 409.6km world
    pub max_elevation: f32,       // 4000.0 meters
    pub sea_level: f32,           // 0.2 (normalized)
    pub seed: u32,
    pub theme: WorldTheme,
}

impl Default for TerrainConfig {
    fn default() -> Self {
        Self {
            chunk_size: 128,
            vertex_count: 129,
            world_width: 2048,
            world_height: 2048,
            cell_size_meters: 100.0,
            max_elevation: 4000.0,
            sea_level: 0.2,
            seed: 12345,
            theme: WorldTheme::Fantasy,
        }
    }
}

impl TerrainConfig {
    pub fn new(width: u32, height: u32, seed: u32, theme: WorldTheme) -> Self {
        Self {
            world_width: width,
            world_height: height,
            seed,
            theme,
            ..Default::default()
        }
    }

    pub fn chunk_count_x(&self) -> i32 {
        ((self.world_width + self.chunk_size - 1) / self.chunk_size) as i32
    }

    pub fn chunk_count_z(&self) -> i32 {
        ((self.world_height + self.chunk_size - 1) / self.chunk_size) as i32
    }

    pub fn world_to_chunk(&self, world_x: f32, world_z: f32) -> (i32, i32) {
        let chunk_x = (world_x / (self.chunk_size as f32 * self.cell_size_meters)).floor() as i32;
        let chunk_z = (world_z / (self.chunk_size as f32 * self.cell_size_meters)).floor() as i32;
        (chunk_x, chunk_z)
    }
}
