use rand::distributions::{IndependentSample, Range};
use std::collections::{HashMap, HashSet};

enum Cell {
    Node { position: Position },
    Wall { position: Position },
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Point {
    position: Position,
}

struct Maze {
    height: u32,
    width: u32,
    map: HashMap<Position, Cell>,
}

impl Maze {
    fn new(height: u32, width: u32) -> Maze {
        let mut map = HashMap::new();

        for x in 0..width {
            for y in 0..height {
                let position = Position { x, y };
                let cell = Cell::Wall { position };

                map.insert(position, cell);
            }
        }

        let start_pos = Position { x: 1, y: 1 };
        let start_node = Cell::Node {
            position: start_pos,
        };
        map.insert(start_pos, start_node);

        Maze { height, width, map }
    }
}

pub fn generate(height: u32, width: u32) -> HashSet<(u32, u32)> {
    // Create a Maze of cells
    let mut maze = Maze::new(height, width);

    // Create a Stack for backtracking
    let mut stack: Vec<Position> = Vec::new();

    // Set point in the maze to start from and add it to the stack
    let mut point = Point {
        position: Position { x: 1, y: 1 },
    };
    stack.push(point.position);

    // While there are cells in the stack
    while stack.len() > 0 {
        // Try to find a position to move to
        if let Some(next) = get_next_position(&maze, point.position) {
            // if a position has been found then move to that position and remove the 
            // wall between there and the old position.

            // Set the new position as the current point and add it to the stack.
            let (new_maze, new_point) = move_to_position(maze, point, next);
            maze = new_maze;
            point = new_point;
        } else {
            // if no position is found then backtrack and try again.
            let next = stack.pop().expect("Nothing Left in the stack!");
            point = Point {position: next};  
            // add this position to the list of ends for creating more paths.
        }
              
    }

    convert_nodes_to_visited(maze)
}

fn move_to_position(maze: Maze, point: Point, target: Position) -> (Maze, Point) {
    let movement_x =  target.x - point.position.x;
    let movement_y = target.y - point.position.y;


    (maze, point)
}

fn get_next_position(maze: &Maze, position: Position) -> Option<Position> {
    // Store all possible next positions
    let mut next_positions = Vec::new();

    // Pick a random direction
    if let Some(cell) = get_cell_from_direction(maze, position, Direction::North) {
        next_positions.push(cell);
    };

    if let Some(cell) = get_cell_from_direction(maze, position, Direction::East) {
        next_positions.push(cell);
    };

    if let Some(cell) = get_cell_from_direction(maze, position, Direction::South) {
        next_positions.push(cell);
    };

    if let Some(cell) = get_cell_from_direction(maze, position, Direction::West) {
        next_positions.push(cell);
    };

    if next_positions.len() == 0 {
        return None;
    }

    let next = pick_random_position(next_positions);

    match next {
        Cell::Node{position} => Some(*position),
        Cell::Wall{position} => Some(*position),
    }
}

fn get_cell_from_direction(maze: &Maze, position: Position, dir: Direction) -> Option<&Cell> {
    let target_position = get_direction(dir, position);

    if target_position.x > maze.width || target_position.y > maze.height {
        return None;
    }

    if target_position.x < 1 || target_position.y < 1 {
        return None;
    }

    if let Some(cell) = maze.map.get(&target_position) {
        return Some(cell);
    }

    None
}

fn get_direction(dir: Direction, position: Position) -> Position {
    match dir {
        Direction::North => Position {
            x: position.x,
            y: position.y - 2,
        },
        Direction::East => Position {
            x: position.x + 2,
            y: position.y,
        },
        Direction::South => Position {
            x: position.x,
            y: position.y + 2,
        },
        Direction::West => Position {
            x: position.x - 2,
            y: position.y,
        },
    }
}

fn convert_nodes_to_visited(maze: Maze) -> HashSet<(u32, u32)> {
    let mut visted = HashSet::new();

    for (_, cell) in maze.map {
        match cell {
            Cell::Node { position } => visted.insert((position.x, position.y)),
            Cell::Wall { position: _ } => false,
        };
    }

    visted
}

fn pick_random_position(next_positions: Vec<&Cell>) -> &Cell {
    let between = Range::new(0, next_positions.len());
    let mut rng = super::rand::thread_rng();
    let random = between.ind_sample(&mut rng) as usize;
    let chosen = next_positions[random];

    (chosen)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_maze() {
        let maze = Maze::new(21, 21);
        assert_eq!(maze.map.len(), 441);
    }

    #[test]
    fn get_pos1() {
        let maze = Maze::new(21, 21);
        let position = Position {x: 1, y: 1};
        let test = get_next_position(&maze, position).expect("Failed to unwrap get next position result!");

        assert!(
            match test {
                Position {x: 4, y: 1} => true,
                Position {x: 1, y: 4} => true,
                _ => false,
            }
        )
    }

}
