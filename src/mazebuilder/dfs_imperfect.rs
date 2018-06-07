use rand::distributions::{IndependentSample, Range};
use std::collections::{HashMap, HashSet};

enum Cell {
    Node { position: Position },
    Wall { position: Position },
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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

        Maze { map }
    }
}

pub fn generate(height: u32, width: u32) -> HashSet<(u32, u32)> {
    // Create a Maze of cells
    let mut maze = Maze::new(height, width);

    // Create a Stack for backtracking
    let mut stack: Vec<Position> = Vec::new();

    // Flag for tracking when we have hit the end of path and started
    // backtracking
    let mut backtracking = false;

    // List of path end cells for creating new paths at the end
    let mut end_nodes: Vec<Position> = Vec::new();

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
            stack.push(point.position);

            // Let the backtracker know to store the next end point.
            backtracking = false;
        } else {
            // if no position is found then backtrack and try again.
            let next = stack.pop().expect("Nothing Left in the stack!");

            // add this position to the list of ends for creating more paths.
            if backtracking == false {
                end_nodes.push(point.position);
                backtracking = true;
            }

            point = Point { position: next };
        }
    }

    println!("end nodes found: {}", end_nodes.len());
    // Go through all nodes in the backtracker list and connect them with other nodes
    // to create multiple paths through the maze
    maze = connect_end_nodes(maze, end_nodes);

    convert_nodes_to_visited(maze)
}

fn connect_end_nodes(mut maze: Maze, end_nodes: Vec<Position>) -> Maze {
    // For each end node
    // Check which direction has already been used to get to this end node
    // pick from the remaining directions that contain walls
    // change the wall between them into a node

    for node in end_nodes {
        let mut position;

        // Inner Scope to contain references
        {
            // Get surrounding walls
            let mut walls = get_walls_for_position(&maze, node);

            // Pick one of those walls to be removed
            let chosen = pick_random_position(walls);
            position = match chosen {
                Cell::Wall { position } => *position,
                Cell::Node { position } => *position,
            };
        }

        // Change the wall into a node
        maze.map.insert(position, Cell::Node { position: position });
    }

    maze
}

fn get_wall_for_position(maze: &Maze, direction: Direction, position: Position) -> Option<&Cell> {
    if let Some(pos) = get_direction(direction, position) {
        let wall = find_wall_position(Point { position }, pos);
        if let Some(cell) = maze.map.get(&wall) {
            return Some(cell);
        }
    }

    None
}

fn get_walls_for_position(maze: &Maze, node: Position) -> Vec<&Cell> {
    let mut walls = Vec::new();

    if let Some(north) = get_wall_for_position(maze, Direction::North, node) {
        walls.push(north);
    }

    if let Some(east) = get_wall_for_position(maze, Direction::East, node) {
        walls.push(east);
    }

    if let Some(south) = get_wall_for_position(maze, Direction::South, node) {
        walls.push(south);
    }

    if let Some(west) = get_wall_for_position(maze, Direction::West, node) {
        walls.push(west);
    }

    // Keep only the walls
    walls.retain(|cell| match cell {
        Cell::Node { position: _ } => false,
        Cell::Wall { position: _ } => true,
    });

    walls
}

fn move_to_position(mut maze: Maze, point: Point, target: Position) -> (Maze, Point) {
    // work out the middle position to get the wall cell
    let wall_cell_position = find_wall_position(point, target);

    // Change both cells to be nodes
    maze.map.insert(target, Cell::Node { position: target });
    maze.map.insert(
        wall_cell_position,
        Cell::Node {
            position: wall_cell_position,
        },
    );

    // Change position to be at the new node
    let point = Point { position: target };

    (maze, point)
}

fn find_wall_position(start: Point, target: Position) -> Position {
    let movement_x = target.x as i32 - start.position.x as i32;
    let movement_y = target.y as i32 - start.position.y as i32;

    let wall_cell_position = Position {
        x: (start.position.x as i32 + (movement_x / 2)) as u32,
        y: (start.position.y as i32 + (movement_y / 2)) as u32,
    };

    wall_cell_position
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

    // Remove the cells that we cannot move to
    next_positions.retain(|cell| match cell {
        Cell::Node { position: _ } => false,
        Cell::Wall { position: _ } => true,
    });

    if next_positions.len() == 0 {
        return None;
    }

    let next = pick_random_position(next_positions);

    match next {
        Cell::Node { position } => Some(*position),
        Cell::Wall { position } => Some(*position),
    }
}

fn get_cell_from_direction(maze: &Maze, position: Position, dir: Direction) -> Option<&Cell> {
    if let Some(target_position) = get_direction(dir, position) {
        if let Some(cell) = maze.map.get(&target_position) {
            return Some(cell);
        }
    }

    None
}

fn get_direction(dir: Direction, position: Position) -> Option<Position> {
    match dir {
        Direction::North => {
            if position.y as i32 - 2 < 1 {
                return None;
            }

            Some(Position {
                x: position.x,
                y: position.y - 2,
            })
        }
        Direction::East => Some(Position {
            x: position.x + 2,
            y: position.y,
        }),
        Direction::South => Some(Position {
            x: position.x,
            y: position.y + 2,
        }),
        Direction::West => {
            if position.x as i32 - 2 < 1 {
                return None;
            }

            Some(Position {
                x: position.x - 2,
                y: position.y,
            })
        }
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
    fn test_create_maze() {
        let maze = Maze::new(21, 21);
        assert_eq!(maze.map.len(), 441);
    }

    #[test]
    fn test_get_next_position() {
        let maze = Maze::new(21, 21);
        let position = Position { x: 1, y: 1 };
        let test =
            get_next_position(&maze, position).expect("Failed to unwrap get next position result!");

        assert_eq!(check_positions(test), true);
    }

    fn check_positions(test: Position) -> bool {
        match test {
            Position { x: 3, y: 1 } => true,
            Position { x: 1, y: 3 } => true,
            _ => false,
        }
    }

    #[test]
    fn test_move_to_position() {
        let maze = Maze::new(21, 21);
        let point = Point {
            position: Position { x: 1, y: 1 },
        };
        let position = Position { x: 3, y: 1 };

        let (new_maze, new_point) = move_to_position(maze, point, position);

        assert_eq!(new_point.position, position);

        let test = new_maze
            .map
            .get(&position)
            .expect("test_move_to_position -- unable to find new node!");
        assert_eq!(
            match test {
                Cell::Node { position: _ } => true,
                Cell::Wall { position: _ } => false,
            },
            true
        );

        let test2 = new_maze
            .map
            .get(&Position { x: 2, y: 1 })
            .expect("test_move_to_position -- unable to find new node!");
        assert_eq!(
            match test2 {
                Cell::Node { position: _ } => true,
                Cell::Wall { position: _ } => false,
            },
            true
        );
    }

    #[test]
    fn test_find_wall_position() {
        let point = Point {
            position: Position { x: 1, y: 1 },
        };
        let position = Position { x: 3, y: 1 };

        let wall_pos = find_wall_position(point, position);

        assert_eq!(wall_pos.x, 2);
        assert_eq!(wall_pos.y, 1);
    }

    #[test]
    fn test_random_position() {
        let cell_a = Cell::Node {
            position: Position { x: 20, y: 20 },
        };
        let cell_b = Cell::Node {
            position: Position { x: 20, y: 20 },
        };
        let cell_c = Cell::Node {
            position: Position { x: 20, y: 20 },
        };

        let list = vec![&cell_a, &cell_b, &cell_c];

        let _test = pick_random_position(list);
    }

    #[test]
    fn test_get_direction() {
        let position = Position { x: 1, y: 1 };
        let north_test = get_direction(Direction::North, position);
        let east_test = get_direction(Direction::East, position);
        let south_test = get_direction(Direction::South, position);
        let west_test = get_direction(Direction::West, position);

        assert_eq!(north_test, None);
        assert_eq!(east_test.unwrap(), Position { x: 3, y: 1 });
        assert_eq!(south_test.unwrap(), Position { x: 1, y: 3 });
        assert_eq!(west_test, None);
    }

    #[test]
    fn test_convert_nodes() {
        let maze = Maze::new(21, 21);
        let test = convert_nodes_to_visited(maze);

        assert_eq!(test.len(), 1);
    }

}
