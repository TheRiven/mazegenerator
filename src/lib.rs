struct Maze {
    height: u64,
    width: u64,
    maze_data: Vec<Cell>,
}

struct Cell {
    visited: bool,
    x: u64,
    y: u64,
}

impl Cell {
    fn new(x: u64, y: u64) -> Cell {
        Cell {
            visited: false,
            x,
            y,
        }
    }

    fn visit(&mut self) {
        self.visited = true;
    }
}

impl Maze {
    fn new(height: u64, width: u64) -> Maze {
        let mut maze_data = Vec::with_capacity((height * width) as usize);

        for x in 0..width {
            for y in 0..height {
                let cell = Cell::new(x, y);
                maze_data.push(cell);
            }
        }

        Maze {
            height,
            width,
            maze_data,
        }
    }

    fn get_cell(&self, x: u64, y: u64) -> Option<&Cell> {
        let cell = self.maze_data.get((x + y * self.width) as usize);

        cell
    }

    fn get_cell_neighbours(&self, cell: &Cell) -> Vec<Option<&Cell>> {
        let north = self.get_cell(cell.x, cell.y + 1);
        let east = self.get_cell(cell.x + 1, cell.y);
        let south = self.get_cell(cell.x, cell.y - 1);
        let west = self.get_cell(cell.x - 1, cell.y);

        vec![north, east, south, west]
    }
}

pub fn generate_maze(height: u64, width: u64) {
    // Generate a maze with the given width and height
    let mut maze = Maze::new(height, width);

    // Stack for backtracking
    let stack: Vec<&Cell> = Vec::new();

    // Get the inital cell and mark it as visited.
    let mut current = maze.get_cell(0, 0).unwrap();
    current.visit(); 

    // Get the current cells neighbours and remove any that have already
    // been visited
    let mut neighbours = maze.get_cell_neighbours(current);
    neighbours.retain(|c| {
        if let &Some(cell) = c {
            !cell.visited
        } else {
            false
        }
    });

    // pick a random neighbour

    // Push the current cell to the stack

    // Remove the wall between the current and chosen cell

    // Make the chosen cell the new current cell and mark as visted
}
