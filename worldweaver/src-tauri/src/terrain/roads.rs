use pathfinding::prelude::astar;
use serde::{Serialize, Deserialize};

/// A road path between two points
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Road {
    pub path: Vec<(i32, i32)>,
    pub cost: u32,
}

/// Calculate road cost based on slope
pub fn road_cost(
    from: (i32, i32),
    to: (i32, i32),
    heights: &[f32],
    width: usize,
    height: usize,
) -> u32 {
    let (fx, fz) = from;
    let (tx, tz) = to;

    if fx < 0 || fx >= width as i32 || fz < 0 || fz >= height as i32 {
        return u32::MAX;
    }
    if tx < 0 || tx >= width as i32 || tz < 0 || tz >= height as i32 {
        return u32::MAX;
    }

    let h_from = heights[fz as usize * width + fx as usize];
    let h_to = heights[tz as usize * width + tx as usize];

    // Diagonal movement costs more
    let horizontal = if fx != tx && fz != tz { 141 } else { 100 };

    // Slope-squared cost function
    let slope = (h_to - h_from).abs() / (horizontal as f32 / 100.0);
    (horizontal as f32 * (1.0 + 8.0 * slope * slope)) as u32
}

/// Generate a road between two points using A*
pub fn generate_road(
    start: (i32, i32),
    goal: (i32, i32),
    heights: &[f32],
    width: usize,
    height: usize,
) -> Option<Road> {
    let result = astar(
        &start,
        |&(x, z)| {
            let mut neighbors = Vec::new();
            for dz in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dz == 0 {
                        continue;
                    }
                    let nx = x + dx;
                    let nz = z + dz;
                    if nx >= 0 && nx < width as i32 && nz >= 0 && nz < height as i32 {
                        let cost = road_cost((x, z), (nx, nz), heights, width, height);
                        if cost < u32::MAX {
                            neighbors.push(((nx, nz), cost));
                        }
                    }
                }
            }
            neighbors
        },
        |&(x, z)| {
            // Manhattan distance heuristic
            ((goal.0 - x).abs() + (goal.1 - z).abs()) as u32 * 100
        },
        |&pos| pos == goal,
    );

    result.map(|(path, cost)| Road { path, cost })
}

/// Get neighbors for a position (8-directional)
pub fn get_neighbors(pos: (i32, i32), width: usize, height: usize) -> Vec<(i32, i32)> {
    let (x, z) = pos;
    let mut neighbors = Vec::new();

    for dz in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dz == 0 {
                continue;
            }
            let nx = x + dx;
            let nz = z + dz;
            if nx >= 0 && nx < width as i32 && nz >= 0 && nz < height as i32 {
                neighbors.push((nx, nz));
            }
        }
    }

    neighbors
}
