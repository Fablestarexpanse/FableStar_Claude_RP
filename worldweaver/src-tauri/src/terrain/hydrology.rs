use std::collections::BinaryHeap;
use std::cmp::Ordering;

/// Cell for priority queue (min-heap)
#[derive(Copy, Clone)]
struct Cell {
    x: usize,
    z: usize,
    height: f32,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}

impl Eq for Cell {}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse ordering for min-heap
        other.height.partial_cmp(&self.height)
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

/// Fill depressions using Priority-Flood algorithm (Barnes et al. 2014)
pub fn fill_depressions(heights: &mut [f32], width: usize, height: usize) {
    let mut open = BinaryHeap::new();
    let mut closed = vec![false; width * height];
    let epsilon = 0.0001; // Small increment to ensure drainage

    // Seed with edge cells
    for x in 0..width {
        // Top edge
        let idx = x;
        open.push(Cell { x, z: 0, height: heights[idx] });
        closed[idx] = true;

        // Bottom edge
        let z = height - 1;
        let idx = z * width + x;
        open.push(Cell { x, z, height: heights[idx] });
        closed[idx] = true;
    }

    for z in 1..(height - 1) {
        // Left edge
        let idx = z * width;
        open.push(Cell { x: 0, z, height: heights[idx] });
        closed[idx] = true;

        // Right edge
        let x = width - 1;
        let idx = z * width + x;
        open.push(Cell { x, z, height: heights[idx] });
        closed[idx] = true;
    }

    // Process cells in priority order
    while let Some(cell) = open.pop() {
        for (nx, nz) in get_neighbors_8(cell.x, cell.z, width, height) {
            let idx = nz * width + nx;
            if !closed[idx] {
                let neighbor_height = heights[idx];
                if neighbor_height < cell.height {
                    // Fill depression
                    heights[idx] = cell.height + epsilon;
                    open.push(Cell { x: nx, z: nz, height: heights[idx] });
                } else {
                    open.push(Cell { x: nx, z: nz, height: neighbor_height });
                }
                closed[idx] = true;
            }
        }
    }
}

/// Calculate D8 flow direction for each cell
pub fn calculate_flow_direction(heights: &[f32], width: usize, height: usize) -> Vec<u8> {
    let mut flow_dir = vec![255u8; width * height]; // 255 = no flow

    // D8 direction offsets: E, SE, S, SW, W, NW, N, NE
    let dx = [1, 1, 0, -1, -1, -1, 0, 1];
    let dz = [0, 1, 1, 1, 0, -1, -1, -1];

    for z in 0..height {
        for x in 0..width {
            let idx = z * width + x;
            let h = heights[idx];

            let mut steepest_slope = 0.0;
            let mut steepest_dir = 255u8;

            for dir in 0..8 {
                let nx = x as i32 + dx[dir];
                let nz = z as i32 + dz[dir];

                if nx >= 0 && nx < width as i32 && nz >= 0 && nz < height as i32 {
                    let nidx = nz as usize * width + nx as usize;
                    let nh = heights[nidx];
                    let slope = h - nh;

                    if slope > steepest_slope {
                        steepest_slope = slope;
                        steepest_dir = dir as u8;
                    }
                }
            }

            flow_dir[idx] = steepest_dir;
        }
    }

    flow_dir
}

/// Calculate flow accumulation
pub fn calculate_flow_accumulation(
    heights: &[f32],
    flow_direction: &[u8],
    width: usize,
    height: usize,
) -> Vec<f32> {
    let mut accumulation = vec![1.0; width * height]; // Each cell starts with 1

    // Sort cells by elevation (descending)
    let mut cells: Vec<(usize, usize, f32)> = Vec::new();
    for z in 0..height {
        for x in 0..width {
            let idx = z * width + x;
            cells.push((x, z, heights[idx]));
        }
    }
    cells.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(Ordering::Equal));

    // D8 direction offsets
    let dx = [1, 1, 0, -1, -1, -1, 0, 1];
    let dz = [0, 1, 1, 1, 0, -1, -1, -1];

    // Propagate flow downstream
    for (x, z, _) in cells {
        let idx = z * width + x;
        let dir = flow_direction[idx];

        if dir < 8 {
            let nx = x as i32 + dx[dir as usize];
            let nz = z as i32 + dz[dir as usize];

            if nx >= 0 && nx < width as i32 && nz >= 0 && nz < height as i32 {
                let nidx = nz as usize * width + nx as usize;
                accumulation[nidx] += accumulation[idx];
            }
        }
    }

    accumulation
}

/// Get 8-directional neighbors
fn get_neighbors_8(x: usize, z: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let dx = [1, 1, 0, -1, -1, -1, 0, 1];
    let dz = [0, 1, 1, 1, 0, -1, -1, -1];

    for i in 0..8 {
        let nx = x as i32 + dx[i];
        let nz = z as i32 + dz[i];

        if nx >= 0 && nx < width as i32 && nz >= 0 && nz < height as i32 {
            neighbors.push((nx as usize, nz as usize));
        }
    }

    neighbors
}

/// Apply thermal erosion (simpler supplement to hydraulic erosion)
pub fn apply_thermal_erosion(
    heights: &mut [f32],
    width: usize,
    height: usize,
    talus_angle: f32,
    iterations: u32,
) {
    let talus_threshold = talus_angle.tan();

    for _ in 0..iterations {
        let mut changes = vec![0.0; width * height];

        for z in 0..height {
            for x in 0..width {
                let idx = z * width + x;
                let h = heights[idx];

                for (nx, nz) in get_neighbors_8(x, z, width, height) {
                    let nidx = nz * width + nx;
                    let nh = heights[nidx];
                    let diff = h - nh;

                    if diff > talus_threshold {
                        let transfer = diff * 0.5;
                        changes[idx] -= transfer;
                        changes[nidx] += transfer;
                    }
                }
            }
        }

        // Apply changes
        for i in 0..heights.len() {
            heights[i] = (heights[i] + changes[i] * 0.1).clamp(0.0, 1.0);
        }
    }
}
