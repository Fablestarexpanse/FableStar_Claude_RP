use noise::{Fbm, RidgedMulti, Perlin, NoiseFn, MultiFractal};
use super::config::TerrainConfig;
use super::heightmap::HeightmapChunk;

use super::commands::NoiseParameters;

/// Generate base terrain using layered noise with geological realism
pub fn generate_terrain(config: &TerrainConfig) -> Vec<HeightmapChunk> {
    generate_terrain_with_params(config, &NoiseParameters::default())
}

/// Generate terrain with custom noise parameters
pub fn generate_terrain_with_params(config: &TerrainConfig, params: &NoiseParameters) -> Vec<HeightmapChunk> {
    let chunk_count_x = config.chunk_count_x();
    let chunk_count_z = config.chunk_count_z();
    let mut chunks = Vec::new();

    // Check if we're generating a flat/blank world (all frequencies are 0)
    let is_flat = params.continent_frequency == 0.0 
        && params.mountain_frequency == 0.0 
        && params.hill_frequency == 0.0 
        && params.detail_frequency == 0.0;

    if is_flat {
        // Generate flat terrain at sea level for painting
        for chunk_z in 0..chunk_count_z {
            for chunk_x in 0..chunk_count_x {
                let vertex_count = config.vertex_count as usize;
                let heights = vec![config.sea_level; vertex_count * vertex_count];
                chunks.push(HeightmapChunk::from_heights((chunk_x, chunk_z), heights));
            }
        }
        return chunks;
    }

    // APPROACH: Multiple independent noise layers that create archipelagos
    // Instead of one big blob, we want scattered landmasses
    
    // Layer 1: Primary continent mask (creates 2-3 large landmasses)
    let continent_mask1 = Fbm::<Perlin>::new(config.seed)
        .set_octaves(2)
        .set_frequency(params.continent_frequency.max(0.00001) * 0.8)
        .set_persistence(0.5)
        .set_lacunarity(2.5);
    
    // Layer 2: Secondary continent mask (creates additional islands)
    let _continent_mask2 = Fbm::<Perlin>::new(config.seed + 100)
        .set_octaves(2)
        .set_frequency(params.continent_frequency.max(0.00001) * 1.2)
        .set_persistence(0.4)
        .set_lacunarity(2.8);
    
    // Layer 3: Base terrain elevation
    let continents = Fbm::<Perlin>::new(config.seed + 1)
        .set_octaves(params.continent_octaves.max(1))
        .set_frequency(params.continent_frequency.max(0.00001) * 2.0)
        .set_persistence(0.5)
        .set_lacunarity(2.0);

    // Layer 4: Mountain ranges
    let mountains = RidgedMulti::<Perlin>::new(config.seed + 2)
        .set_octaves(params.mountain_octaves.max(1))
        .set_frequency(params.mountain_frequency.max(0.0001))
        .set_lacunarity(2.2);

    // Layer 5: Hills
    let hills = Fbm::<Perlin>::new(config.seed + 3)
        .set_octaves(params.hill_octaves.max(1))
        .set_frequency(params.hill_frequency.max(0.0001))
        .set_persistence(0.4)
        .set_lacunarity(2.3);

    // Layer 6: Detail
    let detail = Fbm::<Perlin>::new(config.seed + 4)
        .set_octaves(params.detail_octaves.max(1))
        .set_frequency(params.detail_frequency.max(0.0001))
        .set_persistence(0.25)
        .set_lacunarity(2.5);

    // Combine masks to create archipelagos
    let combined_mask = continent_mask1;

    // Generate each chunk with multi-mask approach for archipelagos
    for chunk_z in 0..chunk_count_z {
        for chunk_x in 0..chunk_count_x {
            let chunk = generate_chunk_with_archipelago(
                chunk_x, chunk_z, config, params,
                &combined_mask, &continents, &mountains, &hills, &detail
            );
            chunks.push(chunk);
        }
    }

    chunks
}

/// Generate chunk with archipelago masking for distinct continents
fn generate_chunk_with_archipelago<N1, N2, N3, N4, N5>(
    chunk_x: i32,
    chunk_z: i32,
    config: &TerrainConfig,
    params: &NoiseParameters,
    continent_mask: &N1,
    base_terrain: &N2,
    mountains: &N3,
    hills: &N4,
    detail: &N5,
) -> HeightmapChunk
where
    N1: NoiseFn<f64, 2>,
    N2: NoiseFn<f64, 2>,
    N3: NoiseFn<f64, 2>,
    N4: NoiseFn<f64, 2>,
    N5: NoiseFn<f64, 2>,
{
    let vertex_count = config.vertex_count;
    let mut heights = Vec::with_capacity((vertex_count * vertex_count) as usize);

    let chunk_world_x = chunk_x as f32 * config.chunk_size as f32 * config.cell_size_meters;
    let chunk_world_z = chunk_z as f32 * config.chunk_size as f32 * config.cell_size_meters;

    for local_z in 0..vertex_count {
        for local_x in 0..vertex_count {
            let world_x = chunk_world_x + local_x as f32 * config.cell_size_meters;
            let world_z = chunk_world_z + local_z as f32 * config.cell_size_meters;

            // Sample continent mask (determines land vs ocean)
            let mask = continent_mask.get([world_x as f64, world_z as f64]) as f32;
            let mask_norm = (mask + 1.0) * 0.5;
            
            // CRITICAL: Apply SHARP threshold to create distinct continents
            // Values above threshold = land, below = ocean
            let land_threshold = params.land_coverage.unwrap_or(0.45);
            
            if mask_norm > land_threshold {
                // This is LAND - sample terrain layers
                let base = base_terrain.get([world_x as f64, world_z as f64]) as f32;
                let mount = mountains.get([world_x as f64, world_z as f64]) as f32;
                let hill = hills.get([world_x as f64, world_z as f64]) as f32;
                let det = detail.get([world_x as f64, world_z as f64]) as f32;
                
                // Normalize
                let base_norm = (base + 1.0) * 0.5;
                let mount_norm = (mount + 1.0) * 0.5;
                let hill_norm = (hill + 1.0) * 0.5;
                let det_norm = (det + 1.0) * 0.5;
                
                // Composite terrain
                let terrain = base_norm * 0.5 + mount_norm * 0.25 + hill_norm * 0.15 + det_norm * 0.1;
                
                // Fade at coastlines (smooth transition)
                let coast_fade = ((mask_norm - land_threshold) / (1.0 - land_threshold)).powf(0.5);
                let height = config.sea_level + terrain * coast_fade * (1.0 - config.sea_level);
                
                heights.push(height.clamp(0.0, 1.0));
            } else {
                // This is OCEAN
                let ocean_depth = (land_threshold - mask_norm) / land_threshold;
                let height = config.sea_level * (1.0 - ocean_depth * 0.5);
                heights.push(height.clamp(0.0, 1.0));
            }
        }
    }

    HeightmapChunk::from_heights((chunk_x, chunk_z), heights)
}

/// Generate a single chunk
fn generate_chunk<N: NoiseFn<f64, 2>>(
    chunk_x: i32,
    chunk_z: i32,
    config: &TerrainConfig,
    noise: &N,
) -> HeightmapChunk {
    let vertex_count = config.vertex_count;
    let mut heights = Vec::with_capacity((vertex_count * vertex_count) as usize);

    let chunk_world_x = chunk_x as f32 * config.chunk_size as f32 * config.cell_size_meters;
    let chunk_world_z = chunk_z as f32 * config.chunk_size as f32 * config.cell_size_meters;

    for local_z in 0..vertex_count {
        for local_x in 0..vertex_count {
            let world_x = chunk_world_x + local_x as f32 * config.cell_size_meters;
            let world_z = chunk_world_z + local_z as f32 * config.cell_size_meters;

            let noise_val = noise.get([world_x as f64, world_z as f64]) as f32;
            
            // Normalize to 0-1 range (noise is typically -1 to 1)
            let height = (noise_val + 1.0) * 0.5;
            let height = height.clamp(0.0, 1.0);

            heights.push(height);
        }
    }

    HeightmapChunk::from_heights((chunk_x, chunk_z), heights)
}

/// Generate a chunk with multiple detail layers
fn generate_chunk_detailed<N1, N2, N3>(
    chunk_x: i32,
    chunk_z: i32,
    config: &TerrainConfig,
    base_noise: &N1,
    hills: &N2,
    detail: &N3,
) -> HeightmapChunk
where
    N1: NoiseFn<f64, 2>,
    N2: NoiseFn<f64, 2>,
    N3: NoiseFn<f64, 2>,
{
    let vertex_count = config.vertex_count;
    let mut heights = Vec::with_capacity((vertex_count * vertex_count) as usize);

    let chunk_world_x = chunk_x as f32 * config.chunk_size as f32 * config.cell_size_meters;
    let chunk_world_z = chunk_z as f32 * config.chunk_size as f32 * config.cell_size_meters;

    for local_z in 0..vertex_count {
        for local_x in 0..vertex_count {
            let world_x = chunk_world_x + local_x as f32 * config.cell_size_meters;
            let world_z = chunk_world_z + local_z as f32 * config.cell_size_meters;

            // Sample all layers
            let base = base_noise.get([world_x as f64, world_z as f64]) as f32;
            let hill = hills.get([world_x as f64, world_z as f64]) as f32;
            let det = detail.get([world_x as f64, world_z as f64]) as f32;
            
            // Normalize each layer
            let base_norm = (base + 1.0) * 0.5;
            let hill_norm = (hill + 1.0) * 0.5;
            let det_norm = (det + 1.0) * 0.5;
            
            // Composite layers with weights (reduce detail contribution)
            let height = base_norm * 0.65 + hill_norm * 0.28 + det_norm * 0.07;
            let height = height.clamp(0.0, 1.0);

            heights.push(height);
        }
    }

    HeightmapChunk::from_heights((chunk_x, chunk_z), heights)
}

/// Generate terrain using simdnoise for performance (bulk generation)
pub fn generate_terrain_simd(config: &TerrainConfig) -> Vec<HeightmapChunk> {
    use simdnoise::NoiseBuilder;

    let chunk_count_x = config.chunk_count_x();
    let chunk_count_z = config.chunk_count_z();
    let mut chunks = Vec::new();

    // Generate base continental layer with simdnoise
    let total_width = config.world_width as usize;
    let total_height = config.world_height as usize;

    let base_noise = NoiseBuilder::fbm_2d(total_width, total_height)
        .with_seed(config.seed as i32)
        .with_freq(0.002)
        .with_octaves(4)
        .generate_scaled(0.0, 1.0);

    // Add detail with noise-rs
    let detail = Fbm::<Perlin>::new(config.seed + 100)
        .set_octaves(3)
        .set_frequency(0.02)
        .set_persistence(0.4);

    // Split into chunks
    for chunk_z in 0..chunk_count_z {
        for chunk_x in 0..chunk_count_x {
            let mut heights = Vec::with_capacity((config.vertex_count * config.vertex_count) as usize);

            let chunk_world_x = chunk_x as f32 * config.chunk_size as f32 * config.cell_size_meters;
            let chunk_world_z = chunk_z as f32 * config.chunk_size as f32 * config.cell_size_meters;

            for local_z in 0..config.vertex_count {
                for local_x in 0..config.vertex_count {
                    let global_x = (chunk_x * config.chunk_size as i32 + local_x as i32) as usize;
                    let global_z = (chunk_z * config.chunk_size as i32 + local_z as i32) as usize;

                    let base_height = if global_x < total_width && global_z < total_height {
                        base_noise[global_z * total_width + global_x]
                    } else {
                        0.5
                    };

                    // Add fine detail
                    let world_x = chunk_world_x + local_x as f32 * config.cell_size_meters;
                    let world_z = chunk_world_z + local_z as f32 * config.cell_size_meters;
                    let detail_val = detail.get([world_x as f64, world_z as f64]) as f32;
                    let detail_val = (detail_val + 1.0) * 0.5;

                    let height = (base_height * 0.7 + detail_val * 0.3).clamp(0.0, 1.0);
                    heights.push(height);
                }
            }

            chunks.push(HeightmapChunk::from_heights((chunk_x, chunk_z), heights));
        }
    }

    chunks
}

/// Apply post-processing to normalize and enhance terrain
pub fn post_process_terrain(chunks: &mut [HeightmapChunk], config: &TerrainConfig) {
    // Find min/max heights
    let mut min_height = f32::MAX;
    let mut max_height = f32::MIN;

    for chunk in chunks.iter() {
        for &h in &chunk.heights {
            min_height = min_height.min(h);
            max_height = max_height.max(h);
        }
    }

    // Normalize to full 0-1 range
    let range = max_height - min_height;
    if range > 0.0 {
        for chunk in chunks.iter_mut() {
            for h in &mut chunk.heights {
                *h = (*h - min_height) / range;
            }
        }
    }

    // Apply sea level adjustment with SHARP continent boundaries
    let sea_level = config.sea_level;
    for chunk in chunks.iter_mut() {
        for h in &mut chunk.heights {
            let mut height = *h;
            
            // CRITICAL: Create distinct continents by applying a threshold
            // This creates sharp land/ocean boundaries instead of gradual transitions
            
            if height < sea_level {
                // Ocean - push values DOWN to create clear separation
                let ocean_depth = height / sea_level;
                height = ocean_depth.powf(2.0) * sea_level * 0.8;  // Deeper oceans
            } else {
                // Land - push values UP to create clear separation
                let land_height = (height - sea_level) / (1.0 - sea_level);
                
                // Apply S-curve with SHARP transition at sea level
                let adjusted = if land_height < 0.3 {
                    // Coastal lowlands - gentle
                    land_height.powf(0.6)
                } else if land_height < 0.6 {
                    // Mid-elevation - steeper
                    0.3_f32.powf(0.6) + (land_height - 0.3).powf(0.8) * 0.4
                } else {
                    // Highlands - dramatic peaks
                    0.3_f32.powf(0.6) + 0.3_f32.powf(0.8) * 0.4 + (land_height - 0.6).powf(1.5) * 0.4
                };
                
                // Boost land elevation to create clear continents
                height = sea_level + adjusted * (1.0 - sea_level) * 1.2;
            }
            
            *h = height.clamp(0.0, 1.0);
        }
    }
}
