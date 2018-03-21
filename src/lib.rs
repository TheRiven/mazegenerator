extern crate rand;
use rand::distributions::{IndependentSample, Range};

use std::collections::HashMap;

mod maze;
use maze::{Cell, Maze, Node};

pub fn generate_maze(height: u64, width: u64) -> HashMap<(u64, u64), Node> {
    // Generate a maze with the given width and height
    let maze = Maze::new(height, width);

    // Stack for backtracking
    let mut stack: Vec<&Cell> = Vec::new();

    // Visted Cells
    let mut visited: Vec<&Cell> = Vec::with_capacity((height * width) as usize);

    // Walls
    let mut walls: Vec<&Cell> = Vec::new();

    // Get the inital cell and mark it as visited.
    let mut current = maze.get_cell(0, 0).unwrap();
    visited.push(current);

    // While there are unvisited cells --
    while visited.len() != ((height * width) as usize) - walls.len() {
        // Get the unvisted neighbours for the current cell
        let mut neighbours = get_cell_neighbours(&maze, current, &visited, &walls);
        println!("Neighbours: {}", neighbours.len());

        // If the cell has any neighbours that have not been visted
        if neighbours.len() > 0 {
            // pick a random neighbour
            let (chosen, index) = pick_random_neighbour(&neighbours);
            neighbours.remove(index);

            // Push the current cell to the stack
            stack.push(current); 

            // Remove the wall between the current and chosen cell
            neighbours.iter().for_each(|c| {
                walls.push(c);
            });

            // Make the chosen cell the new current cell and mark as visted
            current = chosen;
            visited.push(current);
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

    // Create output map
    let mut node_map : HashMap<(u64, u64), Node> = HashMap::with_capacity((height * width) as usize);

    for x in 0..width {
        for y in 0..height {
            let mut wall = true; 
            
            visited.iter().for_each(|v| {
                if v.x == x && v.y == y {
                    wall = false;
                }
            });

            let node = Node::new(x, y, wall);

            node_map.insert((x, y), node);
        }
    }

    // Return the map of the maze so that it can be converted into a image
    node_map
}

fn get_cell_neighbours<'a>(maze: &'a Maze, current: &Cell, visited: &Vec<&Cell>, walls: &Vec<&Cell>) -> Vec<&'a Cell> {
    let mut neighbours = maze.get_cell_neighbours(current);
    neighbours.retain(|c| {
        let mut test = true;
        for node in visited {
            if node.x == c.x && node.y == c.y {
                test = false;
            }
        }

        walls.iter().for_each(|w| {
            if w.x == c.x && w.y == c.y {
                test = false;
            }
        });

        test
    });

    neighbours
}

fn pick_random_neighbour<'a>(neighbours: &Vec<&'a Cell>) -> (&'a Cell, usize) {
    let between = Range::new(0, neighbours.len());
    let mut rng = rand::thread_rng();
    let random = between.ind_sample(&mut rng) as usize;
    let chosen = neighbours[random];

    (chosen, random)
}
