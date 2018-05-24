use std::collections::HashSet;

// location tracking object
struct Person {
    x: u32,
    y: u32,
    facing: Direction,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

// Entry function
pub fn left_first<'a>(
    start: &'a (u32, u32),
    end: (u32, u32),
    maze: &'a HashSet<(u32, u32)>,
) -> Option<Vec<&'a (u32, u32)>> {
    // Create a person at the start of the maze
    let mut person = Person {
        x: start.0,
        y: start.1,
        facing: Direction::South,
    };

    let at_end = true;
    let mut path: Vec<&'a (u32, u32)> = Vec::new();

    // While the person is not at the end of the maze
    while at_end {
        // Add the current node to the path.
        path.push(
            maze.get(&(person.x, person.y))
                .expect("Tried to get maze position using player, but no cell found!"),
        );

        // Check if we are at the end of the maze
        if (person.x, person.y) == end {
            return Some(path);
        }

        // Find the next position to move to.
        let (next_step, new_facing) = find_next_step(&maze, &person);

        // Move the person to that position
        person = move_person(person, next_step, new_facing);
    }

    None
}

fn get_direction(
    maze: &HashSet<(u32, u32)>,
    person: &Person,
    direction: Direction,
) -> Option<(u32, u32)> {
    match direction {
        Direction::North => (get_node(maze, person.x, person.y - 1)),
        Direction::East => (get_node(maze, person.x + 1, person.y)),
        Direction::South => (get_node(maze, person.x, person.y + 1)),
        Direction::West => (get_node(maze, person.x - 1, person.y)),
    }
}

fn get_node(maze: &HashSet<(u32, u32)>, x: u32, y: u32) -> Option<(u32, u32)> {
    let node = maze.get(&(x, y));

    match node {
        Some(value) => Some(*value),
        None => None,
    }
}

fn move_person(_person: Person, position: (u32, u32), new_facing: Direction) -> Person {
    Person {
        x: position.0,
        y: position.1,
        facing: new_facing,
    }
}

fn look_left(maze: &HashSet<(u32, u32)>, person: &Person) -> Option<((u32, u32), Direction)> {
    match person.facing {
        Direction::North => get_cell_from_direction(maze, person, Direction::West),
        Direction::East => get_cell_from_direction(maze, person, Direction::North),
        Direction::South => get_cell_from_direction(maze, person, Direction::East),
        Direction::West => get_cell_from_direction(maze, person, Direction::South),
    }
}

fn look_right(maze: &HashSet<(u32, u32)>, person: &Person) -> Option<((u32, u32), Direction)> {
    match person.facing {
        Direction::North => get_cell_from_direction(maze, person, Direction::East),
        Direction::East => get_cell_from_direction(maze, person, Direction::South),
        Direction::South => get_cell_from_direction(maze, person, Direction::West),
        Direction::West => get_cell_from_direction(maze, person, Direction::North),
    }
}

fn look_back(maze: &HashSet<(u32, u32)>, person: &Person) -> Option<((u32, u32), Direction)> {
    match person.facing {
        Direction::North => get_cell_from_direction(maze, person, Direction::South),
        Direction::East => get_cell_from_direction(maze, person, Direction::West),
        Direction::South => get_cell_from_direction(maze, person, Direction::North),
        Direction::West => get_cell_from_direction(maze, person, Direction::East),
    }
}

fn get_cell_from_direction(
    maze: &HashSet<(u32, u32)>,
    person: &Person,
    dir: Direction,
) -> Option<((u32, u32), Direction)> {
    match get_direction(&maze, &person, dir) {
        Some(position) => Some((position, dir)),
        None => None,
    }
}

fn find_next_step(maze: &HashSet<(u32, u32)>, person: &Person) -> ((u32, u32), Direction) {
    // Look left, if there is a path then that is the way to move,
    // if not look foward and move if there is a path.
    // if there is nothing left or foward, try right.
    // finaly if there is nothing else, go back.
    if let Some(left) = look_left(maze, person) {
        return left;
    };

    if let Some(forward) = get_direction(maze, person, person.facing) {
        return (forward, person.facing);
    };

    if let Some(right) = look_right(maze, person) {
        return right;
    };

    look_back(maze, person).expect("find_next_step -- No Path back from current location found!")
}
