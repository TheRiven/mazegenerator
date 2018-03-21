pub struct Maze {
    height: u64,
    width: u64,
    maze_data: Vec<Cell>,
}

impl Maze {
    pub fn new(height: u64, width: u64) -> Maze {
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

    pub fn get_cell(&self, x: u64, y: u64) -> Option<&Cell> {
        let cell = self.maze_data.get((x + y * self.width) as usize);

        cell
    }

    pub fn get_cell_neighbours(&self, cell: &Cell) -> Vec<&Cell> {
        let mut neighbours = Vec::new();

        if let Some(n) = self.get_cell(cell.x, cell.y + 1) {
            neighbours.push(n);
        }
        if let Some(e) = self.get_cell(cell.x + 1, cell.y) {
            neighbours.push(e);
        }

        if cell.y != 0 {
            if let Some(s) = self.get_cell(cell.x, cell.y - 1) {
                neighbours.push(s);
            }
        }
        if cell.x != 0 {
            if let Some(w) = self.get_cell(cell.x - 1, cell.y) {
                neighbours.push(w);
            }
        }

        neighbours
    }
}

pub struct Cell {
    pub x: u64,
    pub y: u64,
}

impl Cell {
    pub fn new(x: u64, y: u64) -> Cell {
        Cell { x, y }
    }
}

pub struct Node {
    x: u64,
    y: u64,
    wall: bool,
}

impl Node {
    pub fn new(x: u64, y: u64, wall: bool) -> Node {
        Node { x, y, wall }
    }

    pub fn get_pos(&self) -> (u64, u64) {
        (self.x, self.y)
    }

    pub fn is_wall(&self) -> bool {
        self.wall
    }
}
