use serde::{Serialize, Deserialize};

/// A river segment with path and metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RiverSegment {
    pub id: u32,
    pub path: Vec<(f32, f32)>,
    pub strahler_order: u8,
    pub width_meters: f32,
}

/// River network containing all river segments
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RiverNetwork {
    pub segments: Vec<RiverSegment>,
}

impl RiverNetwork {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn add_segment(&mut self, segment: RiverSegment) {
        self.segments.push(segment);
    }

    pub fn clear(&mut self) {
        self.segments.clear();
    }
}

/// Extract rivers from flow accumulation data
pub fn extract_rivers(
    flow_accumulation: &[f32],
    flow_direction: &[u8],
    width: usize,
    height: usize,
    threshold: f32,
) -> RiverNetwork {
    let mut network = RiverNetwork::new();
    let mut visited = vec![false; width * height];
    let mut segment_id = 0;

    // Find all cells above threshold
    for z in 0..height {
        for x in 0..width {
            let idx = z * width + x;
            if flow_accumulation[idx] >= threshold && !visited[idx] {
                // Trace river from this headwater
                let path = trace_river_path(x, z, flow_direction, width, height, &mut visited);
                if path.len() > 2 {
                    // Calculate Strahler order (simplified: based on flow accumulation)
                    let order = calculate_order(flow_accumulation[idx]);
                    let width_meters = calculate_width(order);

                    network.add_segment(RiverSegment {
                        id: segment_id,
                        path,
                        strahler_order: order,
                        width_meters,
                    });
                    segment_id += 1;
                }
            }
        }
    }

    network
}

/// Trace a river path following flow direction
fn trace_river_path(
    start_x: usize,
    start_z: usize,
    flow_direction: &[u8],
    width: usize,
    height: usize,
    visited: &mut [bool],
) -> Vec<(f32, f32)> {
    let mut path = Vec::new();
    let mut x = start_x;
    let mut z = start_z;

    // D8 direction offsets: E, SE, S, SW, W, NW, N, NE
    let dx = [1, 1, 0, -1, -1, -1, 0, 1];
    let dz = [0, 1, 1, 1, 0, -1, -1, -1];

    for _ in 0..1000 {
        // Max path length
        let idx = z * width + x;
        if idx >= visited.len() {
            break;
        }

        path.push((x as f32, z as f32));
        visited[idx] = true;

        let dir = flow_direction[idx] as usize;
        if dir >= 8 {
            break; // No valid direction
        }

        let nx = x as i32 + dx[dir];
        let nz = z as i32 + dz[dir];

        if nx < 0 || nx >= width as i32 || nz < 0 || nz >= height as i32 {
            break; // Reached edge
        }

        x = nx as usize;
        z = nz as usize;

        let next_idx = z * width + x;
        if visited[next_idx] {
            break; // Already visited (confluence or loop)
        }
    }

    path
}

/// Calculate Strahler order from flow accumulation
fn calculate_order(flow_accumulation: f32) -> u8 {
    if flow_accumulation < 1000.0 {
        1
    } else if flow_accumulation < 5000.0 {
        2
    } else if flow_accumulation < 20000.0 {
        3
    } else if flow_accumulation < 100000.0 {
        4
    } else {
        5
    }
}

/// Calculate river width from Strahler order
fn calculate_width(order: u8) -> f32 {
    let base_width = 5.0; // meters
    base_width * 1.5_f32.powi(order as i32 - 1)
}
