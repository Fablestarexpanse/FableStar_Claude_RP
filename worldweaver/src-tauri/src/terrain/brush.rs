use serde::{Serialize, Deserialize};
use super::heightmap::HeightmapChunk;

/// Brush operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrushOp {
    Raise,
    Lower,
    Smooth,
    Flatten { target_height: f32 },
    Erode { droplet_count: u32 },
    Noise { scale: f32, strength: f32 },
}

impl HeightmapChunk {
    /// Apply a brush operation to the chunk
    pub fn apply_brush(
        &mut self,
        center_x: f32,
        center_z: f32,
        radius: f32,
        strength: f32,
        op: BrushOp,
        vertex_count: u32,
    ) {
        match op {
            BrushOp::Raise => self.apply_raise(center_x, center_z, radius, strength, vertex_count),
            BrushOp::Lower => self.apply_lower(center_x, center_z, radius, strength, vertex_count),
            BrushOp::Smooth => self.apply_smooth(center_x, center_z, radius, strength, vertex_count),
            BrushOp::Flatten { target_height } => {
                self.apply_flatten(center_x, center_z, radius, strength, target_height, vertex_count)
            }
            BrushOp::Erode { droplet_count } => {
                self.apply_erode(center_x, center_z, radius, droplet_count, vertex_count)
            }
            BrushOp::Noise { scale, strength: noise_strength } => {
                self.apply_noise(center_x, center_z, radius, scale, noise_strength, vertex_count)
            }
        }
    }

    /// Raise terrain with Gaussian falloff
    fn apply_raise(&mut self, center_x: f32, center_z: f32, radius: f32, strength: f32, vertex_count: u32) {
        let min_x = ((center_x - radius).floor().max(0.0) as usize).min(vertex_count as usize - 1);
        let max_x = ((center_x + radius).ceil().min(vertex_count as f32 - 1.0) as usize).min(vertex_count as usize - 1);
        let min_z = ((center_z - radius).floor().max(0.0) as usize).min(vertex_count as usize - 1);
        let max_z = ((center_z + radius).ceil().min(vertex_count as f32 - 1.0) as usize).min(vertex_count as usize - 1);

        for z in min_z..=max_z {
            for x in min_x..=max_x {
                let dx = x as f32 - center_x;
                let dz = z as f32 - center_z;
                let dist = (dx * dx + dz * dz).sqrt();

                if dist <= radius {
                    let falloff = gaussian_falloff(dist, radius);
                    let idx = z * vertex_count as usize + x;
                    if idx < self.heights.len() {
                        self.heights[idx] += strength * falloff * 0.01;
                        self.heights[idx] = self.heights[idx].clamp(0.0, 1.0);
                    }
                }
            }
        }
    }

    /// Lower terrain with Gaussian falloff
    fn apply_lower(&mut self, center_x: f32, center_z: f32, radius: f32, strength: f32, vertex_count: u32) {
        self.apply_raise(center_x, center_z, radius, -strength, vertex_count);
    }

    /// Smooth terrain using box blur
    fn apply_smooth(&mut self, center_x: f32, center_z: f32, radius: f32, strength: f32, vertex_count: u32) {
        let min_x = ((center_x - radius).floor().max(0.0) as usize).min(vertex_count as usize - 1);
        let max_x = ((center_x + radius).ceil().min(vertex_count as f32 - 1.0) as usize).min(vertex_count as usize - 1);
        let min_z = ((center_z - radius).floor().max(0.0) as usize).min(vertex_count as usize - 1);
        let max_z = ((center_z + radius).ceil().min(vertex_count as f32 - 1.0) as usize).min(vertex_count as usize - 1);

        let mut smoothed = self.heights.clone();

        for z in min_z..=max_z {
            for x in min_x..=max_x {
                let dx = x as f32 - center_x;
                let dz = z as f32 - center_z;
                let dist = (dx * dx + dz * dz).sqrt();

                if dist <= radius {
                    let falloff = gaussian_falloff(dist, radius);
                    let avg = self.calculate_average(x, z, 1, vertex_count);
                    let idx = z * vertex_count as usize + x;
                    if idx < smoothed.len() {
                        smoothed[idx] = self.heights[idx] * (1.0 - strength * falloff) + avg * (strength * falloff);
                    }
                }
            }
        }

        self.heights = smoothed;
    }

    /// Flatten terrain to target height
    fn apply_flatten(
        &mut self,
        center_x: f32,
        center_z: f32,
        radius: f32,
        strength: f32,
        target_height: f32,
        vertex_count: u32,
    ) {
        let min_x = ((center_x - radius).floor().max(0.0) as usize).min(vertex_count as usize - 1);
        let max_x = ((center_x + radius).ceil().min(vertex_count as f32 - 1.0) as usize).min(vertex_count as usize - 1);
        let min_z = ((center_z - radius).floor().max(0.0) as usize).min(vertex_count as usize - 1);
        let max_z = ((center_z + radius).ceil().min(vertex_count as f32 - 1.0) as usize).min(vertex_count as usize - 1);

        for z in min_z..=max_z {
            for x in min_x..=max_x {
                let dx = x as f32 - center_x;
                let dz = z as f32 - center_z;
                let dist = (dx * dx + dz * dz).sqrt();

                if dist <= radius {
                    let falloff = gaussian_falloff(dist, radius);
                    let idx = z * vertex_count as usize + x;
                    if idx < self.heights.len() {
                        self.heights[idx] = self.heights[idx] * (1.0 - strength * falloff)
                            + target_height * (strength * falloff);
                    }
                }
            }
        }
    }

    /// Apply localized erosion (simplified version)
    fn apply_erode(&mut self, center_x: f32, center_z: f32, radius: f32, _droplet_count: u32, vertex_count: u32) {
        // Simplified erosion: slightly lower peaks and raise valleys
        let min_x = ((center_x - radius).floor().max(0.0) as usize).min(vertex_count as usize - 1);
        let max_x = ((center_x + radius).ceil().min(vertex_count as f32 - 1.0) as usize).min(vertex_count as usize - 1);
        let min_z = ((center_z - radius).floor().max(0.0) as usize).min(vertex_count as usize - 1);
        let max_z = ((center_z + radius).ceil().min(vertex_count as f32 - 1.0) as usize).min(vertex_count as usize - 1);

        for z in min_z..=max_z {
            for x in min_x..=max_x {
                let dx = x as f32 - center_x;
                let dz = z as f32 - center_z;
                let dist = (dx * dx + dz * dz).sqrt();

                if dist <= radius {
                    let falloff = gaussian_falloff(dist, radius);
                    let avg = self.calculate_average(x, z, 2, vertex_count);
                    let idx = z * vertex_count as usize + x;
                    if idx < self.heights.len() {
                        // Move toward average (erosion effect)
                        self.heights[idx] = self.heights[idx] * (1.0 - 0.3 * falloff) + avg * (0.3 * falloff);
                    }
                }
            }
        }
    }

    /// Add procedural noise
    fn apply_noise(
        &mut self,
        center_x: f32,
        center_z: f32,
        radius: f32,
        scale: f32,
        strength: f32,
        vertex_count: u32,
    ) {
        use noise::{NoiseFn, Perlin};
        let perlin = Perlin::new(rand::random());

        let min_x = ((center_x - radius).floor().max(0.0) as usize).min(vertex_count as usize - 1);
        let max_x = ((center_x + radius).ceil().min(vertex_count as f32 - 1.0) as usize).min(vertex_count as usize - 1);
        let min_z = ((center_z - radius).floor().max(0.0) as usize).min(vertex_count as usize - 1);
        let max_z = ((center_z + radius).ceil().min(vertex_count as f32 - 1.0) as usize).min(vertex_count as usize - 1);

        for z in min_z..=max_z {
            for x in min_x..=max_x {
                let dx = x as f32 - center_x;
                let dz = z as f32 - center_z;
                let dist = (dx * dx + dz * dz).sqrt();

                if dist <= radius {
                    let falloff = gaussian_falloff(dist, radius);
                    let noise_val = perlin.get([x as f64 * scale as f64, z as f64 * scale as f64]) as f32;
                    let idx = z * vertex_count as usize + x;
                    if idx < self.heights.len() {
                        self.heights[idx] += noise_val * strength * falloff * 0.01;
                        self.heights[idx] = self.heights[idx].clamp(0.0, 1.0);
                    }
                }
            }
        }
    }

    /// Calculate average height in a neighborhood
    fn calculate_average(&self, x: usize, z: usize, kernel_size: usize, vertex_count: u32) -> f32 {
        let mut sum = 0.0;
        let mut count = 0;

        for dz in -(kernel_size as i32)..=(kernel_size as i32) {
            for dx in -(kernel_size as i32)..=(kernel_size as i32) {
                let nx = (x as i32 + dx).max(0).min(vertex_count as i32 - 1) as usize;
                let nz = (z as i32 + dz).max(0).min(vertex_count as i32 - 1) as usize;
                let idx = nz * vertex_count as usize + nx;
                if idx < self.heights.len() {
                    sum += self.heights[idx];
                    count += 1;
                }
            }
        }

        if count > 0 {
            sum / count as f32
        } else {
            self.heights[z * vertex_count as usize + x]
        }
    }
}

/// Gaussian falloff function
fn gaussian_falloff(distance: f32, radius: f32) -> f32 {
    let normalized = distance / radius;
    (-normalized * normalized * 4.0).exp()
}
