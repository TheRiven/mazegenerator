use std::collections::{HashMap, HashSet};
use rand::distributions::{IndependentSample, Range};
use super::maze::Direction;

pub fn kruskal(height: u32, width: u32) -> HashSet<(u32, u32)> {
    // Set of cells
    let mut cells: HashMap<(u32, u32), u32> = HashMap::new();

    let mut y = 1u32;
    let mut id = 1;
    while y < height {
        let mut x = 1u32;
        while x < width {
            cells.insert((x, y), id);
            x += 2;
            id += 1;
        }

        y += 1;
    }

    // Keep track of what has now been visited
    let visited = HashSet::new();

    // For each wall in some random order
        // Get cells that neighbour this wall
        // If 4 neighbours are avalible, pick a north/south or east/west pair
        // if only 3 are available then select the appropriate pair

        // if the cells divided by this wall belong to distinct sets:
        // remove the current wall
        // Join the sets together


    visited
}
