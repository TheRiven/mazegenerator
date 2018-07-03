use rand::distributions::{IndependentSample, Range};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

type Cell = (u32, u32);

struct Wall {
    pub cell_a: Cell,
    pub cell_b: Cell,
    pub x: u32,
    pub y: u32,
}

pub fn kruskal(height: u32, width: u32) -> HashSet<Cell> {
    // List of cells in the maze
    let mut cells: HashSet<Cell> = HashSet::new();

    // Map of Cell sets
    let mut cell_sets: HashMap<Cell, u32> = HashMap::new();

    let timer = Instant::now();

    println!("Kruskal - Generating Cells");
    // Create the Cells and sets
    let mut id = 1;
    let mut y = 1u32;
    while y < height {
        let mut x = 1u32;
        while x < width {
            let new_cell = (x, y);
            cells.insert(new_cell);
            cell_sets.insert(new_cell, id);
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
    let start_walls_count = walls.len() as f32;
    while walls.len() > 0 {
        let wall = pick_random_wall(&mut walls);

        let percent_done = 100f32 - (walls.len() as f32 / start_walls_count) * 100f32;
        if (percent_done % 5f32) == 0f32 {
            println!("{}% Done", percent_done);
        }

        let cell_a = wall.cell_a;
        let cell_b = wall.cell_b;

        let cell_a_set = cell_sets[&cell_a];
        let cell_b_set = cell_sets[&cell_b];

        if cell_a_set != cell_b_set {
            // Join the sets together here and then add the wall (now cell)
            // to it.
            cell_sets = join_cell_sets(cell_a_set, cell_b_set, cell_sets);
            let wall_cell = (wall.x, wall.y);
            cells.insert(wall_cell.clone());
            cell_sets.insert(wall_cell, cell_a_set);
        }
    }

    println!("Kruskal - Set Merging Done in {:?}", timer.elapsed());

    // All sets should have been joined together at this point, so
    // we should now have a map of cells that can be saved and solved
    // for compatability we currently need to return as a hashset of tuples
    let mut visited: HashSet<(u32, u32)> = HashSet::new();

    for cell in cells.drain() {
        visited.insert((cell.0, cell.1));
    }

    println!("Path generation finished.");

    visited
}

fn create_wall_list(cells: &HashSet<Cell>) -> Vec<Wall> {
    // List of walls
    let mut walls: Vec<Wall> = Vec::new();

    // Keep track of what has been visited
    let mut visited = HashSet::new();

    // For each cell, Find its neighbours and create a wall between them
    // then move to the neighbours and repeat untill no more walls can be made.
    let mut cell_stack: VecDeque<Cell> = VecDeque::new();
    let first_cell = cells
        .get(&(1, 1))
        .expect("Unable to find cell 1, 1 in cells set");
    cell_stack.push_back(*first_cell);

    println!("Cells: {}", cells.len());

    while cell_stack.len() > 0 {
        let current = cell_stack
            .pop_front()
            .expect("No Cell Found in cell_stack!");
        let cell_neighbours = find_cell_neighbours(&current, &cells);
        visited.insert(current);

        cell_neighbours.into_iter().for_each(|cell| {
            if !visited.contains(&cell) {
                let mut x = (cell.0 - current.0) as i32;
                if x > 0 {
                    x = x - 1
                };
                if x < 0 {
                    x = x + 1
                };
                let mut y = (cell.1 - current.1) as i32;
                if y > 0 {
                    y = y - 1
                };
                if y < 0 {
                    y = y + 1
                };

                let x = (current.0 as i32 + x) as u32;
                let y = (current.1 as i32 + y) as u32;

                let new_wall = Wall {
                    cell_a: current,
                    cell_b: cell,
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

fn find_cell_neighbours<'a>(cell: &Cell, cells: &'a HashSet<Cell>) -> Vec<Cell> {
    let x = cell.0;
    let y = cell.1;

    let mut neighbour_list: Vec<Cell> = Vec::new();

    if y > 1 {
        if let Some(north) = cells.get(&(x, y - 2)) {
            neighbour_list.push(*north);
        }
    }

    if let Some(east) = cells.get(&(x + 2, y)) {
        neighbour_list.push(*east);
    }

    if let Some(south) = cells.get(&(x, y + 2)) {
        neighbour_list.push(*south);
    }

    if x > 1 {
        if let Some(west) = cells.get(&(x - 2, y)) {
            neighbour_list.push(*west);
        }
    }

    neighbour_list
}

fn join_cell_sets(set_a: u32, set_b: u32, mut sets: HashMap<Cell, u32>) -> HashMap<Cell, u32> {
    // find all of the cells that are in the other set and bring them into the first
    for (_, value) in sets.iter_mut() {
        if *value == set_b {
            *value = set_a;
        }
    }

    sets
}
