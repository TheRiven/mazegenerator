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
}

impl Maze {
    fn new(height: u32, width: u32) -> Maze {
        let mut maze_cells = HashMap::new();
        for x in 0..width {
            for y in 0..height {
                if x == 0 || y == 0 {
                    maze_cells.insert((x, y), Cell::Wall);
                } else if x == width - 1 || y == height - 1 {
                    maze_cells.insert((x, y), Cell::Wall);
                } else {
                    maze_cells.insert((x, y), Cell::Space);
                }
            }
        }

        Maze {
            cells: maze_cells,
        }
    }

    fn check_wall(&self, position: Position) -> bool {
        let cell = self.cells.get(&position);

        match cell {
            Some(c) => match c {
                Cell::Wall => true,
                Cell::Door => true,
                _ => false,
            },
            None => panic!(
                "Check Wall -- Tried to find cell at invalid position: {:?}",
                position
            ),
        }
    }

    fn update_maze_cell(&mut self, position: Position, cell: Cell) {
        let cell_status = match self.cells.get(&position) {
            Some(status) => match status {
                Cell::Wall => Cell::Wall,
                Cell::Door => Cell::Door,
                Cell::Space => Cell::Space,
            },
            None => panic!("Found no cell at: {:?}", position),
        };

        match cell {
            Cell::Wall => match cell_status {
                Cell::Space => {
                    let _ = self.cells.insert(position, cell);
                }
                Cell::Door => panic!("Tried to place a wall on a door at {:?}", position),
                Cell::Wall => panic!("Tried to place a wall on another wall at {:?}", position),
            },
            Cell::Door => match cell_status {
                Cell::Wall => {
                    let _ = self.cells.insert(position, cell);
                }
                Cell::Door => panic!("Tried to place a door on another door at {:?}", position),
                Cell::Space => panic!("Tried to place a door in open space at {:?}", position),
            },
            Cell::Space => panic!("Tried to place a open space after maze creation!"),
        }
    }
}

pub fn generate(height: u32, width: u32) -> HashSet<Position> {
    println!("Generating maze with height {} and width {}", height, width);
    let mut maze = Maze::new(height, width);

    let root_chamber = Chamber {
        top_left: (1, 1),
        height: height - 1,
        width: width - 1,
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
    // Check the chamber is big enough to divide
    let size = chamber.height * chamber.width;
    if size < 2 {
        println!(
            "Chamber {:?} has reached a size of less then 2",
            chamber.top_left
        );
        return maze;
    }

    // pick wall axis
    let wall_x = if chamber.top_left.0 >= (chamber.top_left.0 + chamber.width) {
        chamber.top_left.0
    } else {
        pick_random_u32(chamber.top_left.0, chamber.top_left.0 + chamber.width)
    };
    let wall_y = if chamber.top_left.1 >= (chamber.top_left.1 + chamber.height) {
        chamber.top_left.1
    } else {
        pick_random_u32(chamber.top_left.1, chamber.top_left.1 + chamber.height)
    };

    // Keep track of the new walls.
    let mut wall_list = Vec::new();

    // create walls
    match verticle_wall {
        true => match chamber.height > 1 {
            true => create_verticle_wall(&chamber, wall_x, &mut maze, &mut wall_list),
            false => create_horizontal_wall(&chamber, wall_y, &mut maze, &mut wall_list),
        },
        false => match chamber.width > 1 {
            true => create_horizontal_wall(&chamber, wall_y, &mut maze, &mut wall_list),
            false => create_verticle_wall(&chamber, wall_x, &mut maze, &mut wall_list),
        },
    }

    // Create door in wall
    let index = pick_random_u32(0, wall_list.len() as u32) as usize;
    let door_position = wall_list[index];
    maze.update_maze_cell(door_position, Cell::Door);

    // Divide the chamber into two using the door as the reference point
    let chambers = create_sub_chambers(door_position, verticle_wall, &maze);

    // for sub_chamber in chambers {
    //     println!(
    //         "Dividing chamber {:?} with Height {} and width {}",
    //         sub_chamber.top_left, sub_chamber.height, sub_chamber.width
    //     );
    //     maze = divide_chamber(sub_chamber, !verticle_wall, maze);
    // }

    maze
}

fn create_verticle_wall(
    chamber: &Chamber,
    wall_x: u32,
    maze: &mut Maze,
    wall_list: &mut Vec<Position>,
) {
    let y_start = chamber.top_left.1;
    let y_end = chamber.top_left.1 + chamber.height - 1;

    for y in y_start..y_end {
        let position = (wall_x, y);
        maze.update_maze_cell(position, Cell::Wall);
        wall_list.push(position);
    }
}

fn create_horizontal_wall(
    chamber: &Chamber,
    wall_y: u32,
    maze: &mut Maze,
    wall_list: &mut Vec<Position>,
) {
    let x_start = chamber.top_left.0;
    let x_end = chamber.top_left.0 + chamber.width - 1;

    for x in x_start..x_end {
        let position = (x, wall_y);
        maze.update_maze_cell(position, Cell::Wall);
        wall_list.push(position);
    }
}

fn create_sub_chambers(door_position: Position, verticle_wall: bool, maze: &Maze) -> Vec<Chamber> {
    // use the door position to find the 2 new chambers
    let north = match maze.check_wall(north(door_position)) {
        true => None,
        false => Some(north(door_position)),
    };

    let east = match maze.check_wall(east(door_position)) {
        true => None,
        false => Some(east(door_position)),
    };

    let south = match maze.check_wall(south(door_position)) {
        true => None,
        false => Some(south(door_position)),
    };

    let west = match maze.check_wall(west(door_position)) {
        true => None,
        false => Some(west(door_position)),
    };

    let chamber1_start = match verticle_wall {
        true => east,
        false => north,
    };

    let chamber2_start = match verticle_wall {
        true => west,
        false => south,
    };

    let mut chambers = Vec::new();

    if let Some(chamber) = chamber1_start {
        let chamber1 = create_new_chamber(chamber, maze);
        chambers.push(chamber1);
    };

    if let Some(chamber) = chamber2_start {
        let chamber2 = create_new_chamber(chamber, maze);
        chambers.push(chamber2);
    };

    chambers
}

fn create_new_chamber(start: Position, maze: &Maze) -> Chamber {
    // find the top left
    let mut current_pos = start;

    // move to the left until there is no more room to move to get the x.
    if current_pos.0 == 0 {
        panic!(
            "For some reason we have reached {:?} and are now trying to subtract from it!",
            current_pos
        );
    }
    let mut next_pos = west(current_pos);
    while !maze.check_wall(next_pos) {
        current_pos = next_pos;
        next_pos = west(current_pos);
    }
    let top_x = current_pos.0;

    // move up until there is no more room to get the y.
    next_pos = north(current_pos);
    while !maze.check_wall(next_pos) {
        current_pos = next_pos;
        next_pos = north(current_pos);
    }
    let top_y = current_pos.1;

    // find the height
    let mut height = 1u32;
    next_pos = south(current_pos);
    while !maze.check_wall(next_pos) {
        current_pos = next_pos;
        height += 1;
        next_pos = south(current_pos);
    }

    // find the width
    let mut width = 1u32;
    next_pos = east(current_pos);
    while !maze.check_wall(next_pos) {
        current_pos = next_pos;
        width += 1;
        next_pos = east(current_pos);
    }

    Chamber {
        top_left: (top_x, top_y),
        height,
        width,
    }
}

fn north(position: Position) -> Position {
    (position.0, position.1 - 1)
}

fn east(position: Position) -> Position {
    (position.0 + 1, position.1)
}

fn south(position: Position) -> Position {
    (position.0, position.1 + 1)
}

fn west(position: Position) -> Position {
    (position.0 - 1, position.1)
}

// Random functions

fn pick_random_u32(min: u32, max: u32) -> u32 {
    let between = Range::new(min, max);
    let mut rng = super::rand::thread_rng();

    between.ind_sample(&mut rng)
}
