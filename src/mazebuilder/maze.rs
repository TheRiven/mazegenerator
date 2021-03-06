pub struct Maze {
    height: u32,
    width: u32,
    maze_data: Vec<Cell>,
}

impl Maze {
    pub fn new(height: u32, width: u32) -> Maze {
        let mut maze_data = Vec::with_capacity((height * width) as usize);

        for y in 0..height {
            for x in 0..width {
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

    pub fn get_cell(&self, x: u32, y: u32) -> Option<(&Cell)> {
        let cell = self.maze_data.get((x + y * self.width) as usize);

        cell
    }

    pub fn get_cell_neighbours(&self, cell: &Cell) -> Vec<(&Cell, Direction)> {
        let mut neighbours = Vec::new();

        if cell.y < self.height - 2 {
            if let Some(s) = self.get_cell(cell.x, cell.y + 2) {
                neighbours.push((s, Direction::South));
            }
        }

        if cell.x < self.width - 2 {
            if let Some(e) = self.get_cell(cell.x + 2, cell.y) {
                neighbours.push((e, Direction::East));
            }
        }

        if cell.y > 1 {
            if let Some(n) = self.get_cell(cell.x, cell.y - 2) {
                neighbours.push((n, Direction::North));
            }
        }
        if cell.x > 1 {
            if let Some(w) = self.get_cell(cell.x - 2, cell.y) {
                neighbours.push((w, Direction::West));
            }
        }

        neighbours
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Cell {
    pub x: u32,
    pub y: u32,
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Cell {
        Cell { x, y }
    }
}
