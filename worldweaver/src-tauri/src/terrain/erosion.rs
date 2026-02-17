use rand::Rng;
use rayon::prelude::*;

/// Erosion parameters for particle-based hydraulic erosion
#[derive(Clone, Debug)]
pub struct ErosionParams {
    pub num_droplets: u32,
    pub max_lifetime: u32,
    pub erosion_radius: u32,
    pub inertia: f32,
    pub sediment_capacity_factor: f32,
    pub min_sediment_capacity: f32,
    pub erosion_speed: f32,
    pub deposition_speed: f32,
    pub evaporation_rate: f32,
    pub gravity: f32,
}

impl Default for ErosionParams {
    fn default() -> Self {
        Self {
            num_droplets: 200_000,
            max_lifetime: 64,
            erosion_radius: 3,
            inertia: 0.1,
            sediment_capacity_factor: 6.0,
            min_sediment_capacity: 0.01,
            erosion_speed: 0.5,
            deposition_speed: 0.3,
            evaporation_rate: 0.02,
            gravity: 8.0,
        }
    }
}

/// Apply hydraulic erosion to heightmap
pub fn erode_terrain(
    heights: &mut [f32],
    width: usize,
    height: usize,
    params: &ErosionParams,
) {
    let mut rng = rand::rng();

    for _ in 0..params.num_droplets {
        simulate_droplet(heights, width, height, params, &mut rng);
    }
}

/// Apply hydraulic erosion in parallel
pub fn erode_terrain_parallel(
    heights: &mut [f32],
    width: usize,
    height: usize,
    params: &ErosionParams,
) {
    use std::sync::Mutex;
    let heights_mutex = Mutex::new(heights);

    (0..params.num_droplets).into_par_iter().for_each(|_| {
        let mut rng = rand::rng();
        let mut local_changes: Vec<(usize, f32)> = Vec::new();

        // Simulate droplet and collect changes
        {
            let heights = heights_mutex.lock().unwrap();
            simulate_droplet_collect(&heights, width, height, params, &mut rng, &mut local_changes);
        }

        // Apply changes atomically
        if !local_changes.is_empty() {
            let mut heights = heights_mutex.lock().unwrap();
            for (idx, delta) in local_changes {
                if idx < heights.len() {
                    heights[idx] = (heights[idx] + delta).clamp(0.0, 1.0);
                }
            }
        }
    });
}

/// Simulate a single water droplet (Beyer algorithm)
fn simulate_droplet(
    heights: &mut [f32],
    width: usize,
    height: usize,
    params: &ErosionParams,
    rng: &mut impl Rng,
) {
    let mut x = rng.random_range(0.0..width as f32);
    let mut z = rng.random_range(0.0..height as f32);
    let mut dir_x = 0.0;
    let mut dir_z = 0.0;
    let mut velocity = 1.0;
    let mut water = 1.0;
    let mut sediment = 0.0;

    for _ in 0..params.max_lifetime {
        let ix = x as usize;
        let iz = z as usize;

        if ix >= width - 1 || iz >= height - 1 {
            break;
        }

        // Calculate gradient
        let (grad_x, grad_z) = calculate_gradient(heights, ix, iz, width);

        // Update direction with inertia
        dir_x = dir_x * params.inertia - grad_x * (1.0 - params.inertia);
        dir_z = dir_z * params.inertia - grad_z * (1.0 - params.inertia);

        // Normalize direction
        let len = (dir_x * dir_x + dir_z * dir_z).sqrt();
        if len > 0.0 {
            dir_x /= len;
            dir_z /= len;
        }

        // Move droplet
        let new_x = x + dir_x;
        let new_z = z + dir_z;

        if new_x < 0.0 || new_x >= (width - 1) as f32 || new_z < 0.0 || new_z >= (height - 1) as f32 {
            break;
        }

        // Calculate height difference
        let old_height = sample_height(heights, x, z, width);
        let new_height = sample_height(heights, new_x, new_z, width);
        let height_diff = new_height - old_height;

        // Calculate sediment capacity
        let capacity = (-height_diff).max(params.min_sediment_capacity) 
            * velocity 
            * water 
            * params.sediment_capacity_factor;

        // Erode or deposit
        if sediment > capacity || height_diff > 0.0 {
            // Deposit
            let amount_to_deposit = if height_diff > 0.0 {
                (height_diff).min(sediment)
            } else {
                (sediment - capacity) * params.deposition_speed
            };

            sediment -= amount_to_deposit;
            deposit(heights, x, z, amount_to_deposit, params.erosion_radius, width, height);
        } else {
            // Erode
            let amount_to_erode = (capacity - sediment).min(-height_diff) * params.erosion_speed;
            erode(heights, x, z, amount_to_erode, params.erosion_radius, width, height);
            sediment += amount_to_erode;
        }

        // Update velocity and water
        velocity = (velocity * velocity + height_diff * params.gravity).sqrt();
        water *= 1.0 - params.evaporation_rate;

        x = new_x;
        z = new_z;
    }
}

/// Simulate droplet and collect changes (for parallel version)
fn simulate_droplet_collect(
    heights: &[f32],
    width: usize,
    height: usize,
    params: &ErosionParams,
    rng: &mut impl Rng,
    changes: &mut Vec<(usize, f32)>,
) {
    let mut x = rng.random_range(0.0..width as f32);
    let mut z = rng.random_range(0.0..height as f32);
    let mut dir_x = 0.0;
    let mut dir_z = 0.0;
    let mut velocity = 1.0;
    let mut water = 1.0;
    let mut sediment = 0.0;

    for _ in 0..params.max_lifetime {
        let ix = x as usize;
        let iz = z as usize;

        if ix >= width - 1 || iz >= height - 1 {
            break;
        }

        let (grad_x, grad_z) = calculate_gradient(heights, ix, iz, width);

        dir_x = dir_x * params.inertia - grad_x * (1.0 - params.inertia);
        dir_z = dir_z * params.inertia - grad_z * (1.0 - params.inertia);

        let len = (dir_x * dir_x + dir_z * dir_z).sqrt();
        if len > 0.0 {
            dir_x /= len;
            dir_z /= len;
        }

        let new_x = x + dir_x;
        let new_z = z + dir_z;

        if new_x < 0.0 || new_x >= (width - 1) as f32 || new_z < 0.0 || new_z >= (height - 1) as f32 {
            break;
        }

        let old_height = sample_height(heights, x, z, width);
        let new_height = sample_height(heights, new_x, new_z, width);
        let height_diff = new_height - old_height;

        let capacity = (-height_diff).max(params.min_sediment_capacity) 
            * velocity 
            * water 
            * params.sediment_capacity_factor;

        if sediment > capacity || height_diff > 0.0 {
            let amount_to_deposit = if height_diff > 0.0 {
                (height_diff).min(sediment)
            } else {
                (sediment - capacity) * params.deposition_speed
            };

            sediment -= amount_to_deposit;
            collect_deposit(changes, x, z, amount_to_deposit, params.erosion_radius, width, height);
        } else {
            let amount_to_erode = (capacity - sediment).min(-height_diff) * params.erosion_speed;
            collect_erosion(changes, x, z, amount_to_erode, params.erosion_radius, width, height);
            sediment += amount_to_erode;
        }

        velocity = (velocity * velocity + height_diff * params.gravity).sqrt();
        water *= 1.0 - params.evaporation_rate;

        x = new_x;
        z = new_z;
    }
}

/// Calculate gradient at position
fn calculate_gradient(heights: &[f32], x: usize, z: usize, width: usize) -> (f32, f32) {
    let h = heights[z * width + x];
    let hx = heights[z * width + (x + 1)];
    let hz = heights[(z + 1) * width + x];
    (hx - h, hz - h)
}

/// Sample height with bilinear interpolation
fn sample_height(heights: &[f32], x: f32, z: f32, width: usize) -> f32 {
    let ix = x.floor() as usize;
    let iz = z.floor() as usize;
    let fx = x - ix as f32;
    let fz = z - iz as f32;

    let h00 = heights[iz * width + ix];
    let h10 = heights[iz * width + (ix + 1)];
    let h01 = heights[(iz + 1) * width + ix];
    let h11 = heights[(iz + 1) * width + (ix + 1)];

    let h0 = h00 * (1.0 - fx) + h10 * fx;
    let h1 = h01 * (1.0 - fx) + h11 * fx;
    h0 * (1.0 - fz) + h1 * fz
}

/// Erode terrain at position with Gaussian brush
fn erode(heights: &mut [f32], x: f32, z: f32, amount: f32, radius: u32, width: usize, height: usize) {
    let ix = x as i32;
    let iz = z as i32;
    let r = radius as i32;

    for dz in -r..=r {
        for dx in -r..=r {
            let nx = ix + dx;
            let nz = iz + dz;

            if nx >= 0 && nx < width as i32 && nz >= 0 && nz < height as i32 {
                let dist = ((dx * dx + dz * dz) as f32).sqrt();
                if dist <= radius as f32 {
                    let weight = gaussian_weight(dist, radius as f32);
                    let idx = nz as usize * width + nx as usize;
                    heights[idx] = (heights[idx] - amount * weight).max(0.0);
                }
            }
        }
    }
}

/// Deposit sediment at position with Gaussian brush
fn deposit(heights: &mut [f32], x: f32, z: f32, amount: f32, radius: u32, width: usize, height: usize) {
    let ix = x as i32;
    let iz = z as i32;
    let r = radius as i32;

    for dz in -r..=r {
        for dx in -r..=r {
            let nx = ix + dx;
            let nz = iz + dz;

            if nx >= 0 && nx < width as i32 && nz >= 0 && nz < height as i32 {
                let dist = ((dx * dx + dz * dz) as f32).sqrt();
                if dist <= radius as f32 {
                    let weight = gaussian_weight(dist, radius as f32);
                    let idx = nz as usize * width + nx as usize;
                    heights[idx] = (heights[idx] + amount * weight).min(1.0);
                }
            }
        }
    }
}

/// Collect erosion changes
fn collect_erosion(changes: &mut Vec<(usize, f32)>, x: f32, z: f32, amount: f32, radius: u32, width: usize, height: usize) {
    let ix = x as i32;
    let iz = z as i32;
    let r = radius as i32;

    for dz in -r..=r {
        for dx in -r..=r {
            let nx = ix + dx;
            let nz = iz + dz;

            if nx >= 0 && nx < width as i32 && nz >= 0 && nz < height as i32 {
                let dist = ((dx * dx + dz * dz) as f32).sqrt();
                if dist <= radius as f32 {
                    let weight = gaussian_weight(dist, radius as f32);
                    let idx = nz as usize * width + nx as usize;
                    changes.push((idx, -amount * weight));
                }
            }
        }
    }
}

/// Collect deposition changes
fn collect_deposit(changes: &mut Vec<(usize, f32)>, x: f32, z: f32, amount: f32, radius: u32, width: usize, height: usize) {
    let ix = x as i32;
    let iz = z as i32;
    let r = radius as i32;

    for dz in -r..=r {
        for dx in -r..=r {
            let nx = ix + dx;
            let nz = iz + dz;

            if nx >= 0 && nx < width as i32 && nz >= 0 && nz < height as i32 {
                let dist = ((dx * dx + dz * dz) as f32).sqrt();
                if dist <= radius as f32 {
                    let weight = gaussian_weight(dist, radius as f32);
                    let idx = nz as usize * width + nx as usize;
                    changes.push((idx, amount * weight));
                }
            }
        }
    }
}

/// Gaussian weight function
fn gaussian_weight(distance: f32, radius: f32) -> f32 {
    let normalized = distance / radius;
    (-normalized * normalized * 4.0).exp()
}
