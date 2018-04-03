use std::collections::{HashMap, HashSet};
use rand::distributions::{IndependentSample, Range};
use super::maze::Direction;

pub fn kruskal(height: u32, width: u32) -> HashSet<(u32, u32)> {
    // List of walls
    let mut walls: Vec<(u32, u32)> = Vec::new();

    let mut y = 1u32;
    while y < height {
        let mut x = 2u32;
        while x < width {
            walls.push((x, y));
            x += 2;
        }

        y += 1;
    }

    // Set of cells
    let mut cells: HashMap<(u32, u32), u32> = HashMap::new();

    let mut y = 1u32;
    let mut id = 1;
    while y < height {
        let mut x = 1u32;
        while x < width {
            cells.insert((x, y), id);
            x += 2;
            id += 1;
        }

        y += 1;
    }

    // Keep track of what has now been visited
    let visited = HashSet::new();

    // For each wall in some random order
    while walls.len() > 0 {
        let (wall, index) = pick_random_wall(&walls);

        // Get cells that neighbour this wall
        let neighbours = get_wall_neighbours(wall, &cells);

        // If 4 neighbours are avalible, pick a north/south or east/west pair
        // if only 3 are available then select the appropriate pair
        let neighbours = select_pairs(neighbours);

        // if the cells divided by this wall belong to distinct sets:
        let side1 = cells
            .remove(&neighbours[0])
            .expect("unable to find set id for neighbour '0'!");

        let side2 = cells
            .remove(&neighbours[1])
            .expect("unable to find set id for neighbour '1'!");

        if side1 != side2 {
            // remove the current wall
            let old_wall = walls.remove(index);

            // Join the sets together
            let checked_cells = vec![neighbours[0], neighbours[1], old_wall];
            checked_cells.iter().for_each(|c| {
                cells.insert(*c, side1);
            });
        } else {
            let checked_cells = vec![neighbours[0], neighbours[1]];
            checked_cells.iter().for_each(|c| {
                cells.insert(*c, side1);
            });
        }
    }

    visited
}

fn pick_random_wall(walls: &Vec<(u32, u32)>) -> ((u32, u32), usize) {
    let between = Range::new(0, walls.len());
    let mut rng = super::rand::thread_rng();
    let random = between.ind_sample(&mut rng) as usize;
    let chosen = walls.get(random).unwrap();

    (*chosen, random)
}

fn get_wall_neighbours(
    wall: (u32, u32),
    cells: &HashMap<(u32, u32), u32>,
) -> Vec<((u32, u32), Direction)> {
    let north = (wall.0, wall.1 - 1);
    let east = (wall.0 + 1, wall.1);
    let south = (wall.0, wall.1 + 1);
    let west = (wall.0 - 1, wall.1);

    let mut neighbours: Vec<((u32, u32), Direction)> = Vec::new();

    if cells.contains_key(&north) {
        neighbours.push((north, Direction::North));
    }
    if cells.contains_key(&east) {
        neighbours.push((east, Direction::East));
    }
    if cells.contains_key(&south) {
        neighbours.push((south, Direction::South));
    }
    if cells.contains_key(&west) {
        neighbours.push((west, Direction::West));
    }

    neighbours
}

fn pick_random_direction() -> u32 {
    let between = Range::new(0, 2);
    let mut rng = super::rand::thread_rng();
    let random = between.ind_sample(&mut rng) as u32;

    random
}

fn select_pairs(mut neighbours: Vec<((u32, u32), Direction)>) -> Vec<(u32, u32)> {
    if neighbours.len() == 4 {
        let random_dir = pick_random_direction();
        // 0 = North/South
        if random_dir == 0 {
            neighbours.retain(|cell| {
                let mut test = false;

                if cell.1 == Direction::North || cell.1 == Direction::South {
                    test = true;
                }

                test
            });
        }

        // 1 = East/West
        if random_dir == 1 {
            neighbours.retain(|cell| {
                let mut test = false;
                if cell.1 == Direction::East || cell.1 == Direction::West {
                    test = true;
                }
                test
            });
        }
    } else {
        let mut north = false;
        let mut south = false;
        let mut east = false;
        let mut west = false;
        for cell in &neighbours {
            match cell.1 {
                Direction::North => north = true,
                Direction::East => east = true,
                Direction::South => south = true,
                Direction::West => west = true,
            }
        }

        if north && south {
            neighbours.retain(|cell| {
                let mut test = false;

                if cell.1 == Direction::North || cell.1 == Direction::South {
                    test = true;
                }

                test
            });
        }

        if east && west {
            neighbours.retain(|cell| {
                let mut test = false;
                if cell.1 == Direction::East || cell.1 == Direction::West {
                    test = true;
                }
                test
            });
        }
    }

    let mut adjacent: Vec<(u32, u32)> = Vec::new();

    neighbours.iter().for_each(|c| adjacent.push(c.0));

    adjacent
}
