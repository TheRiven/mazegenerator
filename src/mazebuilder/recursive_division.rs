use rand::distributions::{IndependentSample, Range};
use std::collections::{HashMap, HashSet};

type Position = (u32, u32);

enum Cell {
    Space,
    Door,
    Wall,
}

struct Chamber {
    top_left: Position,
    height: u32,
    width: u32,
}

struct Maze {
    cells: HashMap<Position, Cell>,
    height: u32,
    width: u32,
}

impl Maze {
    fn new(height: u32, width: u32) -> Maze {
        let mut maze_cells = HashMap::new();
        for x in 0..width {
            for y in 0..height {
                if x == 0 {
                    maze_cells.insert((x,y), Cell::Wall);
                } else if y == 0 {
                    maze_cells.insert((x,y), Cell::Wall);
                }

                if x == width {
                    maze_cells.insert((x,y), Cell::Wall);
                } else if y == height {
                    maze_cells.insert((x,y), Cell::Wall);
                }

                maze_cells.insert((x, y), Cell::Space);
            }
        }

        Maze {
            cells: maze_cells,
            height,
            width,
        }
    }

    fn check_wall(&self, position: Position) -> bool {
        let cell = self
            .cells
            .get(&position)
            .expect("Tried to find cell at invalid position!");

        match cell {
            Cell::Wall => true,
            _ => false,
        }
    }
}

pub fn generate(height: u32, width: u32) -> HashSet<Position> {
    let mut maze = Maze::new(height, width);

    let root_chamber = Chamber {
        top_left: (1, 1),
        height,
        width,
    };

    // Create a chamber using the given height and width.
    // divide the chamber into 4 smaller chambers with 4 walls.
    // pick 3 of the 4 walls and open a "doorway" in each of them.
    // Repeat with each smaller chamber until each chamber has a width
    // of one in each direction.

    maze = divide_chamber(root_chamber, true, maze);

    maze.cells.retain(|_pos, cell| match cell {
        Cell::Space => true,
        Cell::Door => true,
        Cell::Wall => false,
    });

    let result = maze.cells.drain().map(|v| v.0).collect();

    result
}

fn divide_chamber(chamber: Chamber, verticle_wall: bool, mut maze: Maze) -> Maze {
    // pick wall axis
    let wall_x = pick_random_u32(chamber.top_left.0, chamber.top_left.0 + chamber.width);
    let wall_y = pick_random_u32(chamber.top_left.1, chamber.top_left.1 + chamber.height);

    // Keep track of the new walls.
    let mut wall_list = Vec::new();

    // create walls
    if verticle_wall {
        for y in chamber.top_left.1..chamber.height {
            let position = (wall_x, y);
            maze.cells.insert(position, Cell::Wall);
            wall_list.push(position);
        }
    } else {
        for x in chamber.top_left.0..chamber.width {
            let position = (x, wall_y);
            maze.cells.insert(position, Cell::Wall);
            wall_list.push(position);
        }
    }

    // Create door in wall
    let index = pick_random_u32(0, wall_list.len() as u32) as usize;
    let door_position = wall_list[index];
    maze.cells.insert(door_position, Cell::Door);

    // Divide the chamber into two using the door as the reference point
    let chambers = create_sub_chambers(chamber, door_position, verticle_wall, &maze);

    for chamber in chambers {
        maze = divide_chamber(chamber, !verticle_wall, maze);
    }

    maze
}

fn create_sub_chambers(
    chamber: Chamber,
    door_position: Position,
    verticle_wall: bool,
    maze: &Maze,
) -> Vec<Chamber> {
    // use the door position to find the 2 new chambers
    let north = (door_position.0, door_position.1 - 1);
    let east = (door_position.0 + 1, door_position.1);
    let south = (door_position.0, door_position.1 + 1);
    let west = (door_position.0 - 1, door_position.1);

    let chamber1_start = match verticle_wall {
        true => east,
        false => north,
    };

    let chamber2_start = match verticle_wall {
        true => west,
        false => south,
    };

    let chamber1 = create_new_chamber(chamber1_start, maze);
    let chamber2 = create_new_chamber(chamber2_start, maze);

    vec![chamber1, chamber2]
}

fn create_new_chamber(start: Position, maze: &Maze) -> Chamber {
    // find the top left
    let mut current_pos = start;

    // move to the left until there is no more room to move to get the x.
    let mut next_pos = (current_pos.0 - 1, current_pos.1);
    while !maze.check_wall(next_pos) {
        current_pos = next_pos;
        next_pos = (current_pos.0 - 1, current_pos.1);
    }
    let top_x = current_pos.0;

    // move up until there is no more room to get the y.
    next_pos = (current_pos.0, current_pos.1 - 1);
    while !maze.check_wall(next_pos) {
        current_pos = next_pos;
        next_pos = (current_pos.0, current_pos.1 - 1);
    }
    let top_y = current_pos.1;

    // find the height
    next_pos = (current_pos.0, current_pos.1 + 1);
    while !maze.check_wall(next_pos) {
        current_pos = next_pos;
        next_pos = (current_pos.0, current_pos.1 + 1);
    }
    let height = current_pos.1;

    // find the width
    next_pos = (current_pos.0 + 1, current_pos.1);
    while !maze.check_wall(next_pos) {
        current_pos = next_pos;
        next_pos = (current_pos.0 + 1, current_pos.1);
    }
    let width = current_pos.0;

    Chamber {
        top_left: (top_x, top_y),
        height,
        width,
    }
}

// Random functions

fn pick_random_u32(min: u32, max: u32) -> u32 {
    let between = Range::new(min, max);
    let mut rng = super::rand::thread_rng();

    between.ind_sample(&mut rng)
}

// fn pick_random_position(chamber: &Chamber) -> Position {
//     let min_x = chamber.top_left.0;
//     let min_y = chamber.top_left.1;
//     let max_x = chamber.top_left.0 + chamber.width;
//     let max_y = chamber.top_left.1 + chamber.height;

//     let between_x = Range::new(min_x, max_x);
//     let between_y = Range::new(min_y, max_y);
//     let mut rng = super::rand::thread_rng();

//     let pick_x = between_x.ind_sample(&mut rng);
//     let pick_y = between_y.ind_sample(&mut rng);

//     (pick_x, pick_y)
// }
