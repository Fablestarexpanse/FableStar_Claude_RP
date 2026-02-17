use tauri::{State, Emitter};
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use super::TerrainData;
use super::config::{TerrainConfig, WorldTheme};
use super::brush::BrushOp;

/// Noise generation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseParameters {
    pub continent_frequency: f64,
    pub continent_octaves: usize,
    pub mountain_frequency: f64,
    pub mountain_octaves: usize,
    pub hill_frequency: f64,
    pub hill_octaves: usize,
    pub detail_frequency: f64,
    pub detail_octaves: usize,
    pub land_coverage: Option<f32>,  // Threshold for land vs ocean
}

impl Default for NoiseParameters {
    fn default() -> Self {
        Self {
            continent_frequency: 0.00005,
            continent_octaves: 3,
            mountain_frequency: 0.0002,
            mountain_octaves: 4,
            hill_frequency: 0.0005,
            hill_octaves: 3,
            detail_frequency: 0.001,
            detail_octaves: 2,
            land_coverage: Some(0.45),
        }
    }
}

/// Request to generate new terrain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateTerrainRequest {
    pub width: u32,
    pub height: u32,
    pub seed: u32,
    pub theme: WorldTheme,
    pub use_erosion: bool,
    pub erosion_iterations: u32,
    pub noise_params: Option<NoiseParameters>,
}

/// Response with generation progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateTerrainResponse {
    pub success: bool,
    pub message: String,
    pub chunk_count: usize,
}

/// Progress update during generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationProgress {
    pub stage: String,
    pub progress: f32, // 0.0 to 1.0
    pub message: String,
}

/// Request to get a chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChunkRequest {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub lod: u8,
}

/// Request to apply brush
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyBrushRequest {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub center_x: f32,
    pub center_z: f32,
    pub radius: f32,
    pub strength: f32,
    pub brush_type: String,
}

/// Generate new terrain
#[tauri::command]
pub async fn generate_terrain(
    request: GenerateTerrainRequest,
    terrain: State<'_, Mutex<TerrainData>>,
    app: tauri::AppHandle,
) -> Result<GenerateTerrainResponse, String> {
    use super::noise_gen::{generate_terrain_simd, generate_terrain_with_params, post_process_terrain};
    use super::erosion::{erode_terrain_parallel, ErosionParams};
    use super::hydrology::{fill_depressions, calculate_flow_direction, calculate_flow_accumulation};

    // Helper to emit progress
    let emit_progress = |stage: &str, progress: f32, message: &str| {
        let _ = app.emit("terrain-progress", GenerationProgress {
            stage: stage.to_string(),
            progress,
            message: message.to_string(),
        });
    };

    emit_progress("🌍 Shaping continents...", 0.0, "Generating base terrain");

    let config = TerrainConfig::new(request.width, request.height, request.seed, request.theme);
    
    // Generate base terrain with custom noise parameters if provided
    let mut chunks = if let Some(params) = request.noise_params {
        generate_terrain_with_params(&config, &params)
    } else {
        generate_terrain_simd(&config)
    };
    
    emit_progress("⛰️ Raising mountains...", 0.2, "Applying elevation curves");
    post_process_terrain(&mut chunks, &config);

    // Apply erosion if requested
    if request.use_erosion {
        emit_progress("🏔️ Carving valleys...", 0.35, "Preparing erosion simulation");
        
        // Flatten chunks into single heightmap for erosion
        let vertex_count = config.vertex_count as usize;
        let total_width = config.world_width as usize;
        let total_height = config.world_height as usize;
        let mut heights = vec![0.0; total_width * total_height];

        for chunk in &chunks {
            let chunk_start_x = (chunk.coord.0 * config.chunk_size as i32) as usize;
            let chunk_start_z = (chunk.coord.1 * config.chunk_size as i32) as usize;

            for local_z in 0..vertex_count {
                for local_x in 0..vertex_count {
                    let global_x = chunk_start_x + local_x;
                    let global_z = chunk_start_z + local_z;
                    if global_x < total_width && global_z < total_height {
                        let chunk_idx = local_z * vertex_count + local_x;
                        let global_idx = global_z * total_width + global_x;
                        heights[global_idx] = chunk.heights[chunk_idx];
                    }
                }
            }
        }

        emit_progress("🌊 Filling lakes...", 0.45, "Removing terrain depressions");
        // Fill depressions
        fill_depressions(&mut heights, total_width, total_height);

        emit_progress("💧 Simulating erosion...", 0.55, "Running hydraulic erosion");
        // Apply hydraulic erosion
        let params = ErosionParams {
            num_droplets: request.erosion_iterations * 1000,
            ..Default::default()
        };
        erode_terrain_parallel(&mut heights, total_width, total_height, &params);

        emit_progress("🏞️ Tracing rivers...", 0.75, "Calculating water flow");
        // Calculate flow for rivers
        let flow_direction = calculate_flow_direction(&heights, total_width, total_height);
        let flow_accumulation = calculate_flow_accumulation(&heights, &flow_direction, total_width, total_height);

        // Copy back to chunks
        for chunk in &mut chunks {
            let chunk_start_x = (chunk.coord.0 * config.chunk_size as i32) as usize;
            let chunk_start_z = (chunk.coord.1 * config.chunk_size as i32) as usize;

            let mut flow_data = Vec::new();

            for local_z in 0..vertex_count {
                for local_x in 0..vertex_count {
                    let global_x = chunk_start_x + local_x;
                    let global_z = chunk_start_z + local_z;
                    if global_x < total_width && global_z < total_height {
                        let chunk_idx = local_z * vertex_count + local_x;
                        let global_idx = global_z * total_width + global_x;
                        chunk.heights[chunk_idx] = heights[global_idx];
                        flow_data.push(flow_accumulation[global_idx]);
                    }
                }
            }

            chunk.flow_accumulation = Some(flow_data);
        }

        emit_progress("🌲 Placing forests...", 0.85, "Extracting river networks");
        // Extract rivers
        use super::rivers::extract_rivers;
        let river_network = extract_rivers(&flow_accumulation, &flow_direction, total_width, total_height, 1000.0);
        
        let mut terrain = terrain.lock().await;
        terrain.river_network = river_network;
    } else {
        emit_progress("🌲 Placing forests...", 0.7, "Skipping erosion");
    }

    emit_progress("✨ Finalizing world...", 0.95, "Saving terrain data");
    
    // Update terrain data
    let chunk_count = chunks.len();
    let mut terrain = terrain.lock().await;
    terrain.config = config;
    terrain.chunks.clear();
    for chunk in chunks {
        terrain.chunks.insert(chunk.coord, chunk);
    }
    terrain.dirty_chunks.clear();

    emit_progress("✅ Complete!", 1.0, "Terrain generation finished");

    Ok(GenerateTerrainResponse {
        success: true,
        message: format!("Generated {} chunks", chunk_count),
        chunk_count,
    })
}

/// Get a chunk's height data
#[tauri::command]
pub async fn get_chunk(
    request: GetChunkRequest,
    terrain: State<'_, Mutex<TerrainData>>,
) -> Result<Vec<u8>, String> {
    let terrain = terrain.lock().await;
    
    let chunk = terrain.chunks.get(&(request.chunk_x, request.chunk_z))
        .ok_or("Chunk not found")?;

    // Return raw f32 bytes (binary IPC)
    let bytes: Vec<u8> = chunk.heights.iter()
        .flat_map(|h| h.to_le_bytes())
        .collect();

    Ok(bytes)
}

/// Apply brush operation to chunk
#[tauri::command]
pub async fn apply_brush(
    request: ApplyBrushRequest,
    terrain: State<'_, Mutex<TerrainData>>,
) -> Result<Vec<u8>, String> {
    let mut terrain = terrain.lock().await;
    
    let op = match request.brush_type.as_str() {
        "raise" => BrushOp::Raise,
        "lower" => BrushOp::Lower,
        "smooth" => BrushOp::Smooth,
        "flatten" => BrushOp::Flatten { target_height: 0.5 },
        "erode" => BrushOp::Erode { droplet_count: 100 },
        "noise" => BrushOp::Noise { scale: 0.1, strength: request.strength },
        _ => return Err("Unknown brush type".into()),
    };

    let vertex_count = terrain.config.vertex_count;
    
    // Get chunk and apply brush
    let chunk = terrain.chunks.get_mut(&(request.chunk_x, request.chunk_z))
        .ok_or("Chunk not found")?;
    chunk.apply_brush(request.center_x, request.center_z, request.radius, request.strength, op, vertex_count);
    
    // Mark dirty
    terrain.dirty_chunks.insert((request.chunk_x, request.chunk_z));

    // Return modified heights as raw bytes
    let chunk = terrain.chunks.get(&(request.chunk_x, request.chunk_z)).unwrap();
    let bytes: Vec<u8> = chunk.heights.iter()
        .flat_map(|h| h.to_le_bytes())
        .collect();

    Ok(bytes)
}

/// Get terrain configuration
#[tauri::command]
pub async fn get_terrain_config(
    terrain: State<'_, Mutex<TerrainData>>,
) -> Result<TerrainConfig, String> {
    let terrain = terrain.lock().await;
    Ok(terrain.config.clone())
}

/// Place water sources for hydrology simulation
#[tauri::command]
pub async fn place_water_sources(
    count: usize,
    source_type: String,
    terrain: State<'_, Mutex<TerrainData>>,
) -> Result<serde_json::Value, String> {
    use rand::Rng;
    
    let mut terrain_data = terrain.lock().await;
    let config = &terrain_data.config;
    
    // Flatten chunks to get heightmap
    let total_width = config.world_width as usize;
    let total_height = config.world_height as usize;
    let mut heights = vec![0.0; total_width * total_height];
    
    for ((chunk_x, chunk_z), chunk) in &terrain_data.chunks {
        let chunk_offset_x = *chunk_x as usize * config.chunk_size as usize;
        let chunk_offset_z = *chunk_z as usize * config.chunk_size as usize;
        
        for local_z in 0..config.vertex_count as usize {
            for local_x in 0..config.vertex_count as usize {
                let global_x = chunk_offset_x + local_x;
                let global_z = chunk_offset_z + local_z;
                
                if global_x < total_width && global_z < total_height {
                    let chunk_idx = local_z * config.vertex_count as usize + local_x;
                    let global_idx = global_z * total_width + global_x;
                    heights[global_idx] = chunk.heights[chunk_idx];
                }
            }
        }
    }
    
    // Place water sources based on type
    let mut sources = Vec::new();
    let mut rng = rand::rng();
    
    match source_type.as_str() {
        "random" => {
            // Random locations
            for _ in 0..count {
                let x = rng.random_range(0..total_width);
                let y = rng.random_range(0..total_height);
                sources.push(super::WaterSource {
                    x,
                    y,
                    flow_rate: 1.0,
                    active: true,
                });
            }
        },
        "peaks" => {
            // Find high elevation points
            let mut peak_candidates: Vec<(usize, usize, f32)> = Vec::new();
            for y in 1..total_height-1 {
                for x in 1..total_width-1 {
                    let idx = y * total_width + x;
                    let h = heights[idx];
                    
                    // Check if this is a local maximum
                    let is_peak = h > heights[idx - 1] &&
                                  h > heights[idx + 1] &&
                                  h > heights[idx - total_width] &&
                                  h > heights[idx + total_width] &&
                                  h > config.sea_level + 0.1; // Above sea level
                    
                    if is_peak {
                        peak_candidates.push((x, y, h));
                    }
                }
            }
            
            // Sort by height and take top N
            peak_candidates.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
            for (x, y, _) in peak_candidates.iter().take(count) {
                sources.push(super::WaterSource {
                    x: *x,
                    y: *y,
                    flow_rate: 2.0, // More flow from peaks
                    active: true,
                });
            }
        },
        "ridges" => {
            // Find ridge lines (high gradient perpendicular to flow)
            for _ in 0..count {
                let x = rng.random_range(1..total_width-1);
                let y = rng.random_range(1..total_height-1);
                let idx = y * total_width + x;
                
                // Check if above sea level
                if heights[idx] > config.sea_level + 0.05 {
                    sources.push(super::WaterSource {
                        x,
                        y,
                        flow_rate: 1.5,
                        active: true,
                    });
                }
            }
        },
        "uniform" => {
            // Uniform grid
            let grid_size = (count as f32).sqrt() as usize;
            let step_x = total_width / (grid_size + 1);
            let step_y = total_height / (grid_size + 1);
            
            for gy in 1..=grid_size {
                for gx in 1..=grid_size {
                    let x = gx * step_x;
                    let y = gy * step_y;
                    let idx = y * total_width + x;
                    
                    if heights[idx] > config.sea_level {
                        sources.push(super::WaterSource {
                            x,
                            y,
                            flow_rate: 1.0,
                            active: true,
                        });
                    }
                }
            }
        },
        _ => return Err("Invalid source type".to_string()),
    }
    
    let sources_placed = sources.len();
    terrain_data.water_sources = sources;
    
    Ok(serde_json::json!({
        "success": true,
        "sources_placed": sources_placed,
        "message": format!("Placed {} water sources", sources_placed)
    }))
}

/// Simulate hydrology with water particles
#[tauri::command]
pub async fn simulate_hydrology(
    steps: u32,
    enable_lakes: bool,
    enable_capture: bool,
    terrain: State<'_, Mutex<TerrainData>>,
    app: tauri::AppHandle,
) -> Result<GenerateTerrainResponse, String> {
    use super::erosion::{erode_terrain_parallel, ErosionParams};
    use super::hydrology::{fill_depressions, calculate_flow_direction, calculate_flow_accumulation};
    use super::rivers::extract_rivers;
    
    let emit_progress = |stage: &str, progress: f32, message: &str| {
        let _ = app.emit("terrain-progress", GenerationProgress {
            stage: stage.to_string(),
            progress,
            message: message.to_string(),
        });
    };
    
    emit_progress("💧 Initializing simulation...", 0.0, "Preparing water sources");
    
    let mut terrain_data = terrain.lock().await;
    
    if terrain_data.water_sources.is_empty() {
        return Err("No water sources placed. Place water sources first.".to_string());
    }
    
    let config = terrain_data.config.clone();
    let total_width = config.world_width as usize;
    let total_height = config.world_height as usize;
    
    // Flatten chunks into heightmap
    let mut heights = vec![0.0; total_width * total_height];
    
    for ((chunk_x, chunk_z), chunk) in &terrain_data.chunks {
        let chunk_offset_x = *chunk_x as usize * config.chunk_size as usize;
        let chunk_offset_z = *chunk_z as usize * config.chunk_size as usize;
        
        for local_z in 0..config.vertex_count as usize {
            for local_x in 0..config.vertex_count as usize {
                let global_x = chunk_offset_x + local_x;
                let global_z = chunk_offset_z + local_z;
                
                if global_x < total_width && global_z < total_height {
                    let chunk_idx = local_z * config.vertex_count as usize + local_x;
                    let global_idx = global_z * total_width + global_x;
                    heights[global_idx] = chunk.heights[chunk_idx];
                }
            }
        }
    }
    
    emit_progress("🌊 Simulating water flow...", 0.2, format!("Running {} time steps", steps).as_str());
    
    // Run particle-based erosion from each water source
    let params = ErosionParams {
        num_droplets: steps * terrain_data.water_sources.len() as u32 * 10,
        ..Default::default()
    };
    erode_terrain_parallel(&mut heights, total_width, total_height, &params);
    
    if enable_lakes {
        emit_progress("🏞️ Forming lakes...", 0.5, "Filling depressions");
        fill_depressions(&mut heights, total_width, total_height);
    }
    
    emit_progress("🌊 Calculating flow...", 0.7, "Tracing water paths");
    let flow_direction = calculate_flow_direction(&heights, total_width, total_height);
    let flow_accumulation = calculate_flow_accumulation(&heights, &flow_direction, total_width, total_height);
    
    emit_progress("🏞️ Extracting rivers...", 0.85, "Finding river networks");
    let river_network = extract_rivers(&flow_accumulation, &flow_direction, total_width, total_height, 500.0);
    
    // Update terrain with eroded heights
    let mut dirty_chunks = Vec::new();
    for ((chunk_x, chunk_z), chunk) in &mut terrain_data.chunks {
        let chunk_offset_x = *chunk_x as usize * config.chunk_size as usize;
        let chunk_offset_z = *chunk_z as usize * config.chunk_size as usize;
        
        for local_z in 0..config.vertex_count as usize {
            for local_x in 0..config.vertex_count as usize {
                let global_x = chunk_offset_x + local_x;
                let global_z = chunk_offset_z + local_z;
                
                if global_x < total_width && global_z < total_height {
                    let chunk_idx = local_z * config.vertex_count as usize + local_x;
                    let global_idx = global_z * total_width + global_x;
                    chunk.heights[chunk_idx] = heights[global_idx];
                }
            }
        }
        dirty_chunks.push((*chunk_x, *chunk_z));
    }
    
    for coord in dirty_chunks {
        terrain_data.dirty_chunks.insert(coord);
    }
    
    terrain_data.river_network = river_network;
    
    emit_progress("✅ Complete!", 1.0, "Hydrology simulation finished");
    
    Ok(GenerateTerrainResponse {
        success: true,
        message: format!("Simulated {} steps from {} water sources", steps, terrain_data.water_sources.len()),
        chunk_count: terrain_data.chunks.len(),
    })
}

/// Apply weathering simulation (additional erosion + river formation)
#[tauri::command]
pub async fn apply_weathering(
    iterations: u32,
    terrain: State<'_, Mutex<TerrainData>>,
    app: tauri::AppHandle,
) -> Result<GenerateTerrainResponse, String> {
    use super::erosion::{erode_terrain_parallel, ErosionParams};
    use super::hydrology::{fill_depressions, calculate_flow_direction, calculate_flow_accumulation};
    use super::rivers::extract_rivers;
    
    let emit_progress = |stage: &str, progress: f32, message: &str| {
        let _ = app.emit("terrain-progress", GenerationProgress {
            stage: stage.to_string(),
            progress,
            message: message.to_string(),
        });
    };
    
    emit_progress("🌊 Simulating weathering...", 0.0, "Preparing terrain");
    
    let mut terrain_data = terrain.lock().await;
    let config = terrain_data.config.clone();
    
    // Flatten chunks into single heightmap
    let total_width = config.world_width as usize;
    let total_height = config.world_height as usize;
    let mut heights = vec![0.0; total_width * total_height];
    
    for ((chunk_x, chunk_z), chunk) in &terrain_data.chunks {
        let chunk_offset_x = *chunk_x as usize * config.chunk_size as usize;
        let chunk_offset_z = *chunk_z as usize * config.chunk_size as usize;
        
        for local_z in 0..config.vertex_count as usize {
            for local_x in 0..config.vertex_count as usize {
                let global_x = chunk_offset_x + local_x;
                let global_z = chunk_offset_z + local_z;
                
                if global_x < total_width && global_z < total_height {
                    let chunk_idx = local_z * config.vertex_count as usize + local_x;
                    let global_idx = global_z * total_width + global_x;
                    heights[global_idx] = chunk.heights[chunk_idx];
                }
            }
        }
    }
    
    emit_progress("💧 Filling depressions...", 0.2, "Removing terrain pits");
    fill_depressions(&mut heights, total_width, total_height);
    
    emit_progress("🏔️ Eroding terrain...", 0.4, "Simulating water erosion");
    let params = ErosionParams {
        num_droplets: iterations * 1000,
        ..Default::default()
    };
    erode_terrain_parallel(&mut heights, total_width, total_height, &params);
    
    emit_progress("🌊 Calculating flow...", 0.7, "Tracing water paths");
    let flow_direction = calculate_flow_direction(&heights, total_width, total_height);
    let flow_accumulation = calculate_flow_accumulation(&heights, &flow_direction, total_width, total_height);
    
    emit_progress("🏞️ Extracting rivers...", 0.85, "Finding river networks");
    let river_network = extract_rivers(&flow_accumulation, &flow_direction, total_width, total_height, 1000.0);
    
    // Update terrain with eroded heights
    let mut dirty_chunks = Vec::new();
    for ((chunk_x, chunk_z), chunk) in &mut terrain_data.chunks {
        let chunk_offset_x = *chunk_x as usize * config.chunk_size as usize;
        let chunk_offset_z = *chunk_z as usize * config.chunk_size as usize;
        
        for local_z in 0..config.vertex_count as usize {
            for local_x in 0..config.vertex_count as usize {
                let global_x = chunk_offset_x + local_x;
                let global_z = chunk_offset_z + local_z;
                
                if global_x < total_width && global_z < total_height {
                    let chunk_idx = local_z * config.vertex_count as usize + local_x;
                    let global_idx = global_z * total_width + global_x;
                    chunk.heights[chunk_idx] = heights[global_idx];
                }
            }
        }
        dirty_chunks.push((*chunk_x, *chunk_z));
    }
    
    // Mark all chunks as dirty
    for coord in dirty_chunks {
        terrain_data.dirty_chunks.insert(coord);
    }
    
    terrain_data.river_network = river_network;
    
    emit_progress("✅ Complete!", 1.0, "Weathering simulation finished");
    
    Ok(GenerateTerrainResponse {
        success: true,
        message: format!("Applied {} weathering iterations", iterations),
        chunk_count: terrain_data.chunks.len(),
    })
}

/// Get river network
#[tauri::command]
pub async fn get_rivers(
    terrain: State<'_, Mutex<TerrainData>>,
) -> Result<Vec<super::rivers::RiverSegment>, String> {
    let terrain = terrain.lock().await;
    Ok(terrain.river_network.segments.clone())
}

/// Get flow accumulation data for rendering rivers/lakes
#[tauri::command]
pub async fn get_flow_data(
    terrain: State<'_, Mutex<TerrainData>>,
) -> Result<Vec<u8>, String> {
    use super::hydrology::{calculate_flow_direction, calculate_flow_accumulation};
    
    let terrain_data = terrain.lock().await;
    let config = &terrain_data.config;
    
    // Flatten chunks into heightmap
    let total_width = config.world_width as usize;
    let total_height = config.world_height as usize;
    let mut heights = vec![0.0; total_width * total_height];
    
    for ((chunk_x, chunk_z), chunk) in &terrain_data.chunks {
        let chunk_offset_x = *chunk_x as usize * config.chunk_size as usize;
        let chunk_offset_z = *chunk_z as usize * config.chunk_size as usize;
        
        for local_z in 0..config.vertex_count as usize {
            for local_x in 0..config.vertex_count as usize {
                let global_x = chunk_offset_x + local_x;
                let global_z = chunk_offset_z + local_z;
                
                if global_x < total_width && global_z < total_height {
                    let chunk_idx = local_z * config.vertex_count as usize + local_x;
                    let global_idx = global_z * total_width + global_x;
                    heights[global_idx] = chunk.heights[chunk_idx];
                }
            }
        }
    }
    
    // Calculate flow
    let flow_direction = calculate_flow_direction(&heights, total_width, total_height);
    let flow_accumulation = calculate_flow_accumulation(&heights, &flow_direction, total_width, total_height);
    
    // Normalize flow to 0-255 range for texture
    let max_flow = flow_accumulation.iter().cloned().fold(0.0f32, f32::max);
    let flow_bytes: Vec<u8> = flow_accumulation.iter()
        .map(|&f| ((f / max_flow) * 255.0).min(255.0) as u8)
        .collect();
    
    Ok(flow_bytes)
}

/// Save terrain to database
#[tauri::command]
pub async fn save_terrain(
    terrain: State<'_, Mutex<TerrainData>>,
) -> Result<String, String> {
    use super::persistence::TerrainDatabase;

    let terrain = terrain.lock().await;
    
    let db = TerrainDatabase::new("terrain.db")
        .map_err(|e| format!("Failed to open database: {}", e))?;

    // Save config
    db.save_config(&terrain.config)
        .map_err(|e| format!("Failed to save config: {}", e))?;

    // Save all chunks
    let mut saved_count = 0;
    for chunk in terrain.chunks.values() {
        db.save_chunk(chunk)
            .map_err(|e| format!("Failed to save chunk: {}", e))?;
        saved_count += 1;
    }

    // Save rivers
    for segment in &terrain.river_network.segments {
        db.save_river_segment(segment)
            .map_err(|e| format!("Failed to save river: {}", e))?;
    }

    Ok(format!("Saved {} chunks and {} rivers", saved_count, terrain.river_network.segments.len()))
}

/// Load terrain from database
#[tauri::command]
pub async fn load_terrain(
    terrain: State<'_, Mutex<TerrainData>>,
) -> Result<String, String> {
    use super::persistence::TerrainDatabase;

    let db = TerrainDatabase::new("terrain.db")
        .map_err(|e| format!("Failed to open database: {}", e))?;

    // Load config
    let config = db.load_config()
        .map_err(|e| format!("Failed to load config: {}", e))?;

    // Load all chunks
    let mut chunks = std::collections::HashMap::new();
    for chunk_z in 0..config.chunk_count_z() {
        for chunk_x in 0..config.chunk_count_x() {
            if db.chunk_exists(chunk_x, chunk_z, 0)
                .map_err(|e| format!("Failed to check chunk: {}", e))? {
                let chunk = db.load_chunk(chunk_x, chunk_z, 0)
                    .map_err(|e| format!("Failed to load chunk: {}", e))?;
                chunks.insert((chunk_x, chunk_z), chunk);
            }
        }
    }

    // Load rivers
    let river_segments = db.load_river_segments()
        .map_err(|e| format!("Failed to load rivers: {}", e))?;

    let chunk_count = chunks.len();
    let river_count = river_segments.len();

    let mut terrain = terrain.lock().await;
    terrain.config = config;
    terrain.chunks = chunks;
    terrain.river_network.segments = river_segments;
    terrain.dirty_chunks.clear();

    Ok(format!("Loaded {} chunks and {} rivers", chunk_count, river_count))
}
