use rand::distributions::{IndependentSample, Range};
use std::collections::HashSet;

type Cell = (u32, u32);

pub fn generate(mut maze: HashSet<Cell>) -> HashSet<Cell> {
    // List of deadends
    let mut deadends = Vec::new();

    for node in &maze {
        // For each node find its neighbours, if they only have one then mark it as a dead end.
        if is_cell_deadend(*node, &maze) {
            deadends.push(*node);
        }
    }

    // Select a number of dead ends to change to paths
    deadends = pick_deadends(deadends, 0.05);

    // Convert the deadends into paths
    for end in deadends {
        maze = convert_to_path(end, maze);
    }

    maze
}

fn convert_to_path(cell: Cell, mut maze: HashSet<Cell>) -> HashSet<Cell> {
    // Get the cell neighbours and work out which one we are connected to
    // then pick one of the other neighbours and make a connection there as
    // well.

    let mut neighbouring_cells = get_cell_neighbours(cell, &maze, 2);
    let neighbouring_path = get_cell_neighbours(cell, &maze, 1);

    // Remove the cell that is already connected to this one
    neighbouring_cells = find_matching_cell_path(neighbouring_cells, neighbouring_path[0]);

    // Pick one of the neighbouring cells to create the new path with.
    let (neighbour, _) = pick_random_cell(&neighbouring_cells);

    // Create path
    maze = add_path_cell(cell, neighbour, maze);

    maze
}

fn add_path_cell(from_cell: Cell, to_cell: Cell, mut maze: HashSet<Cell>) -> HashSet<Cell> {
    // get the position between the two known cells and create a new cell for the maze
    let start_x = from_cell.0 as i32;
    let start_y = from_cell.1 as i32;
    let end_x = to_cell.0 as i32;
    let end_y = to_cell.1 as i32;

    let wall_pos_x = start_x + (end_x - start_x) / 2;
    let wall_pos_y = start_y + (end_y - start_y) / 2;

    let new_cell = (wall_pos_x as u32, wall_pos_y as u32);

    maze.insert(new_cell);

    maze
}

fn find_matching_cell_path(mut cells: Vec<Cell>, path_cell: Cell) -> Vec<Cell> {
    let mut index = 0;

    for (i, cell) in cells.iter().enumerate() {
        let x = (cell.0 as i32 - path_cell.0 as i32).abs();
        let y = (cell.1 as i32 - path_cell.1 as i32).abs();

        if x + y == 1 {
            index = i;
        }
    }

    cells.remove(index);

    cells
}

fn is_cell_deadend(cell: Cell, maze: &HashSet<Cell>) -> bool {
    let neighbours = get_cell_neighbours(cell, &maze, 1);

    neighbours.len() == 1
}

fn pick_deadends(mut deadends: Vec<Cell>, percent: f32) -> Vec<Cell> {
    let mut selected_ends = Vec::new();

    let change_count = (deadends.len() as f32 * percent).ceil() as u32;

    println!("Removing {} dead ends from maze", change_count);

    for _ in 0..change_count {
        let (end, index) = pick_random_cell(&deadends);
        selected_ends.push(end);
        deadends.remove(index);
    }

    selected_ends
}

fn get_cell_neighbours(cell: Cell, maze: &HashSet<Cell>, offset: u32) -> Vec<Cell> {
    let x = cell.0;
    let y = cell.1;

    let mut neighbours = Vec::new();

    if y > 1 {
        if let Some(north) = maze.get(&(x, y - offset)) {
            neighbours.push(*north);
        };
    }

    if let Some(east) = maze.get(&(x + offset, y)) {
        neighbours.push(*east);
    };

    if let Some(south) = maze.get(&(x, y + offset)) {
        neighbours.push(*south);
    };

    if x > 1 {
        if let Some(west) = maze.get(&(x - offset, y)) {
            neighbours.push(*west);
        };
    }

    neighbours
}

fn pick_random_cell(cell_list: &Vec<Cell>) -> (Cell, usize) {
    let between = Range::new(0, cell_list.len());
    let mut rng = super::rand::thread_rng();
    let random = between.ind_sample(&mut rng) as usize;
    let chosen = cell_list[random];

    (chosen, random)
}
