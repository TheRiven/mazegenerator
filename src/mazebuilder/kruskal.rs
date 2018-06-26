use rand::distributions::{IndependentSample, Range};
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::{Rc, Weak};
use std::time::Instant;

#[derive(Eq, Hash, PartialEq)]
struct Cell {
    pub x: u32,
    pub y: u32,
}

struct Wall {
    pub cell_a: Weak<Cell>,
    pub cell_b: Weak<Cell>,
    pub x: u32,
    pub y: u32,
}

pub fn kruskal(height: u32, width: u32) -> HashSet<(u32, u32)> {
    // List of cells in the maze
    let mut cells: HashSet<Rc<Cell>> = HashSet::new();

    // Map of Cell sets
    let mut cell_sets: HashMap<u32, Vec<Rc<Cell>>> = HashMap::new();
    // Map of cells with their sets
    let mut cell_set_list: HashMap<Rc<Cell>, u32> = HashMap::new();

    let timer = Instant::now();

    println!("Kruskal - Generating Cells");
    // Create the Cells and sets
    let mut id = 1;
    let mut y = 1u32;
    while y < height {
        let mut x = 1u32;
        while x < width {
            let new_cell = Rc::new(Cell { x, y });
            cells.insert(new_cell.clone());
            cell_sets.insert(id, vec![new_cell.clone()]);
            cell_set_list.insert(new_cell, id);
            x += 2;
            id += 1;
        }
        y += 2;
    }

    println!("Kruskal - Cell Generation Done in {:?}", timer.elapsed());

    println!("Kruskal - Generating Walls");
    // List of walls
    let timer = Instant::now();
    let mut walls = create_wall_list(&cells);

    println!("Kruskal - Wall Generation Done in {:?}", timer.elapsed());

    // So long as there are walls left to look at,
    // select a random wall and get the cells it connects
    // if the cells belong to diffrent sets, turn the wall into a cell
    // and join the two sets together
    println!("Kruskal - Merging Cells Sets");
    let timer = Instant::now();
    while walls.len() > 0 {
        let wall = pick_random_wall(&mut walls);

        let cell_a = wall
            .cell_a
            .upgrade()
            .expect("Found a wall without a Cell_a");
        let cell_b = wall
            .cell_b
            .upgrade()
            .expect("Found a wall without a Cell_b");

        let cell_a_set = cell_set_list[&cell_a];
        let cell_b_set = cell_set_list[&cell_b];

        if cell_a_set != cell_b_set {
            // Join the sets together here and then add the wall (now cell)
            // to it.
            let unified_set_id =
                join_cell_sets(cell_a_set, cell_b_set, &mut cell_sets, &mut cell_set_list);
            let wall_cell = Rc::new(Cell {
                x: wall.x,
                y: wall.y,
            });
            cells.insert(wall_cell.clone());
            cell_set_list.insert(wall_cell.clone(), unified_set_id);
            cell_sets
                .get_mut(&unified_set_id)
                .expect("No Cell set for this ID found!")
                .push(wall_cell);
        }
    }

    println!("Kruskal - Set Merging Done in {:?}", timer.elapsed());

    // All sets should have been joined together at this point, so
    // we should now have a map of cells that can be saved and solved
    // for compatability we currently need to return as a hashset of tuples
    let mut visited: HashSet<(u32, u32)> = HashSet::new();

    for cell in cells.drain() {
        visited.insert((cell.x, cell.y));
    }

    println!("Path generation finished.");

    visited
}

fn create_wall_list(cells: &HashSet<Rc<Cell>>) -> Vec<Wall> {
    // List of walls
    let mut walls: Vec<Wall> = Vec::new();

    // Keep track of what has been visited
    let mut visited = HashSet::new();

    // For each cell, Find its neighbours and create a wall between them
    // then move to the neighbours and repeat untill no more walls can be made.
    let mut cell_stack: VecDeque<&Rc<Cell>> = VecDeque::new();
    let first_cell = cells
        .get(&Cell { x: 1, y: 1 })
        .expect("Unable to find cell 1, 1 in cells set");
    cell_stack.push_back(first_cell);

    println!("Cells: {}", cells.len());

    while cell_stack.len() > 0 {
        let current = cell_stack
            .pop_front()
            .expect("No Cell Found in cell_stack!");
        let cell_neighbours = find_cell_neighbours(&current, &cells);
        visited.insert(current);

        cell_neighbours.into_iter().for_each(|cell| {
            if !visited.contains(cell) {
                let mut x = (cell.x - current.x) as i32;
                if x > 0 {
                    x = x - 1
                };
                if x < 0 {
                    x = x + 1
                };
                let mut y = (cell.y - current.y) as i32;
                if y > 0 {
                    y = y - 1
                };
                if y < 0 {
                    y = y + 1
                };

                let x = (current.x as i32 + x) as u32;
                let y = (current.y as i32 + y) as u32;

                let new_wall = Wall {
                    cell_a: Rc::downgrade(&current),
                    cell_b: Rc::downgrade(&cell),
                    x,
                    y,
                };
                walls.push(new_wall);
                if cell_stack.contains(&cell) == false {
                    cell_stack.push_back(cell);
                }
            }
        });
    }

    println!("Walls Generated: {}", walls.len());

    walls
}

fn pick_random_wall(walls: &mut Vec<Wall>) -> Wall {
    let between = Range::new(0, walls.len());
    let mut rng = super::rand::thread_rng();
    let random = between.ind_sample(&mut rng) as usize;
    let chosen = walls.remove(random);

    chosen
}

fn find_cell_neighbours<'a>(cell: &Cell, cells: &'a HashSet<Rc<Cell>>) -> Vec<&'a Rc<Cell>> {
    let x = cell.x;
    let y = cell.y;

    let mut neighbour_list: Vec<&Rc<Cell>> = Vec::new();

    if y > 1 {
        if let Some(north) = cells.get(&Cell { x: x, y: y - 2 }) {
            neighbour_list.push(north);
        }
    }

    if let Some(east) = cells.get(&Cell { x: x + 2, y: y }) {
        neighbour_list.push(east);
    }

    if let Some(south) = cells.get(&Cell { x: x, y: y + 2 }) {
        neighbour_list.push(south);
    }

    if x > 1 {
        if let Some(west) = cells.get(&Cell { x: x - 2, y: y }) {
            neighbour_list.push(west);
        }
    }

    neighbour_list
}

fn join_cell_sets(
    set_a: u32,
    set_b: u32,
    set_map: &mut HashMap<u32, Vec<Rc<Cell>>>,
    cell_map: &mut HashMap<Rc<Cell>, u32>,
) -> u32 {
    let mut set_old = set_map
        .remove(&set_b)
        .expect("Unable to find Set B to remove from sets map!");

    for cell in &mut set_old {
        cell_map
            .entry(cell.clone())
            .and_modify(|value| *value = set_a);
    }

    set_map
        .get_mut(&set_a)
        .expect("Unable to find Set A to update from sets map!")
        .append(&mut set_old);

    set_a
}
