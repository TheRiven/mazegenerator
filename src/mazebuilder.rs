use rand::distributions::{IndependentSample, Range};

use std::collections::HashMap;

use maze::{Cell, Direction, Maze, Node};

pub fn generate_maze(height: u64, width: u64) -> HashMap<(u64, u64), Node> {
    // Generate a maze with the given width and height
    let maze = Maze::new(height, width);

    // Stack for backtracking
    let mut stack: Vec<&Cell> = Vec::new();

    // Visted Cells
    let mut visited: HashMap<(u64, u64), Node> = HashMap::with_capacity((height * width) as usize);

    // Get the inital cell and mark it as visited.
    let mut current = maze.get_cell(1, 1).unwrap();
    visited.insert((current.x, current.y), Node::new(false));

    // While there are unvisited cells --
    while visited.len() != (height * width) as usize {
        // Get the unvisted neighbours for the current cell
        let neighbours = get_cell_neighbours(&maze, current, &visited);

        // If the cell has any neighbours that have not been visted
        if neighbours.len() > 0 {
            // pick a random neighbour
            let (chosen, dir) = pick_random_neighbour(&neighbours);

            // Push the current cell to the stack
            stack.push(current);

            // Remove the wall between the current and chosen cell
            // check the direction of the chosen cell to see where
            // we have moved. Then go back one cell as this is the wall.
            let wall = match dir {
                Direction::North => maze.get_cell(current.x, current.y - 1),
                Direction::East => maze.get_cell(current.x + 1, current.y),
                Direction::South => maze.get_cell(current.x, current.y + 1),
                Direction::West => maze.get_cell(current.x - 1, current.y),
            }.unwrap();

            visited.insert((wall.x, wall.y), Node::new(false));

            // Make the chosen cell the new current cell and mark as visted
            current = chosen;
            visited.insert((current.x, current.y), Node::new(false));
        } else {
            // If the stack is not empty
            if stack.len() > 0 {
                // pop a cell from the stack and make it the current one
                current = stack.pop().unwrap();
            } else {
                // We are done!
                break;
            }
        }
    }
    println!("Path generation finished.");

    // Return the path map of the maze so that it can be converted into a image
    visited
}

fn get_cell_neighbours<'a>(
    maze: &'a Maze,
    current: &Cell,
    visited: &HashMap<(u64, u64), Node>,
) -> Vec<(&'a Cell, Direction)> {
    let mut neighbours = maze.get_cell_neighbours(current);
    neighbours.retain(|c| {
        let mut test = true;
        let cell = c.0;

        if visited.contains_key(&(cell.x, cell.y)) {
            test = false;
        }

        test
    });

    neighbours
}

fn pick_random_neighbour<'a>(neighbours: &Vec<(&'a Cell, Direction)>) -> ((&'a Cell, Direction)) {
    let between = Range::new(0, neighbours.len());
    let mut rng = super::rand::thread_rng();
    let random = between.ind_sample(&mut rng) as usize;
    let chosen = neighbours[random];

    (chosen)
}
