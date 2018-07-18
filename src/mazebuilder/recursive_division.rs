use std::collections::HashSet;

type Cell = (u32, u32);

pub fn generate(height: u32, width: u32) -> HashSet<Cell> {
    let maze_result = HashSet::new();

    // Create a chamber using the given height and width.
    // divide the chamber into 4 smaller chambers with 4 walls.
    // pick 3 of the 4 walls and open a "doorway" in each of them.
    // Repeat with each smaller chamber until each chamber has a width
    // of one in each direction.


    maze_result
}