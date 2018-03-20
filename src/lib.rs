extern crate rand;

use rand::distributions::{IndependentSample, Range};

mod maze;
use maze::{Cell, Maze};

pub fn generate_maze(height: u64, width: u64) {
    // Generate a maze with the given width and height
    let maze = Maze::new(height, width);

    // Stack for backtracking
    let mut stack: Vec<&Cell> = Vec::new();

    // Visted Cells
    let mut visited: Vec<&Cell> = Vec::with_capacity((height * width) as usize);

    // Get the inital cell and mark it as visited.
    let mut current = maze.get_cell(0, 0).unwrap();
    visited.push(current);

    // While there are unvisited cells --
    while visited.len() != (height * width) as usize {
        // Get the unvisted neighbours for the current cell
        let neighbours = get_cell_neighbours(&maze, current, &visited);

        // If the cell has any neighbours that have not been visted
        if neighbours.len() > 0 {
            // pick a random neighbour
            let chosen = pick_random_neighbour(neighbours);

            // Push the current cell to the stack
            stack.push(current);

            // Remove the wall between the current and chosen cell

            // Make the chosen cell the new current cell and mark as visted
            current = chosen;
        } else {
            // If the stack is not empty
            if stack.len() > 0 {
                // pop a cell from the stack and make it the current one
                current = stack.pop().unwrap();
            } else {
                println!("No Cells left in the stack!");
                break;
            }
        }
    }
}

fn get_cell_neighbours<'a>(maze: &'a Maze, current: &Cell, visited: &Vec<&Cell>) -> Vec<&'a Cell> {
    let mut neighbours = maze.get_cell_neighbours(current);
    neighbours.retain(|c| {
        let mut test = false;
        for node in visited {
            if node.x == c.x && node.y == c.y {
                test = true;
            }
        }

        test
    });

    neighbours
}

fn pick_random_neighbour(neighbours: Vec<&Cell>) -> &Cell {
    let between = Range::new(0, neighbours.len());
    let mut rng = rand::thread_rng();
    let random = between.ind_sample(&mut rng) as usize;
    let chosen = neighbours[random];

    chosen 
}
