use serde::{Serialize, Deserialize};

/// A single chunk of heightmap data (128×128 cells, 129×129 vertices)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeightmapChunk {
    pub coord: (i32, i32),
    pub heights: Vec<f32>,        // 129×129 = 16,641 values
    pub lod: u8,
    pub flow_accumulation: Option<Vec<f32>>,
    pub biome_ids: Option<Vec<u8>>,
}

impl HeightmapChunk {
    /// Create a new chunk with flat terrain at sea level
    pub fn new(coord: (i32, i32), vertex_count: u32) -> Self {
        let size = (vertex_count * vertex_count) as usize;
        Self {
            coord,
            heights: vec![0.2; size], // Default to sea level
            lod: 0,
            flow_accumulation: None,
            biome_ids: None,
        }
    }

    /// Create a chunk from existing height data
    pub fn from_heights(coord: (i32, i32), heights: Vec<f32>) -> Self {
        Self {
            coord,
            heights,
            lod: 0,
            flow_accumulation: None,
            biome_ids: None,
        }
    }

    /// Sample height at local coordinates (nearest neighbor)
    pub fn sample(&self, local_x: f32, local_z: f32, vertex_count: u32) -> f32 {
        let ix = local_x.round().clamp(0.0, (vertex_count - 1) as f32) as usize;
        let iz = local_z.round().clamp(0.0, (vertex_count - 1) as f32) as usize;
        let idx = iz * vertex_count as usize + ix;
        self.heights.get(idx).copied().unwrap_or(0.0)
    }

    /// Sample height with bilinear interpolation
    pub fn sample_bilinear(&self, local_x: f32, local_z: f32, vertex_count: u32) -> f32 {
        let x0 = local_x.floor().clamp(0.0, (vertex_count - 2) as f32) as usize;
        let z0 = local_z.floor().clamp(0.0, (vertex_count - 2) as f32) as usize;
        let x1 = (x0 + 1).min((vertex_count - 1) as usize);
        let z1 = (z0 + 1).min((vertex_count - 1) as usize);

        let fx = local_x - x0 as f32;
        let fz = local_z - z0 as f32;

        let h00 = self.heights[z0 * vertex_count as usize + x0];
        let h10 = self.heights[z0 * vertex_count as usize + x1];
        let h01 = self.heights[z1 * vertex_count as usize + x0];
        let h11 = self.heights[z1 * vertex_count as usize + x1];

        let h0 = h00 * (1.0 - fx) + h10 * fx;
        let h1 = h01 * (1.0 - fx) + h11 * fx;
        h0 * (1.0 - fz) + h1 * fz
    }

    /// Set height at local coordinates
    pub fn set_height(&mut self, local_x: usize, local_z: usize, height: f32, vertex_count: u32) {
        if local_x < vertex_count as usize && local_z < vertex_count as usize {
            let idx = local_z * vertex_count as usize + local_x;
            if idx < self.heights.len() {
                self.heights[idx] = height;
            }
        }
    }

    /// Get height at local coordinates
    pub fn get_height(&self, local_x: usize, local_z: usize, vertex_count: u32) -> f32 {
        if local_x < vertex_count as usize && local_z < vertex_count as usize {
            let idx = local_z * vertex_count as usize + local_x;
            self.heights.get(idx).copied().unwrap_or(0.0)
        } else {
            0.0
        }
    }

    /// Calculate gradient at a position (returns (dx, dz))
    pub fn calculate_gradient(&self, x: usize, z: usize, vertex_count: u32) -> (f32, f32) {
        let h = self.get_height(x, z, vertex_count);
        
        let hx = if x < (vertex_count - 1) as usize {
            self.get_height(x + 1, z, vertex_count)
        } else {
            h
        };
        
        let hz = if z < (vertex_count - 1) as usize {
            self.get_height(x, z + 1, vertex_count)
        } else {
            h
        };
        
        (hx - h, hz - h)
    }

    /// Get the vertex count (assumes square chunk)
    pub fn vertex_count(&self) -> u32 {
        (self.heights.len() as f32).sqrt() as u32
    }
}
