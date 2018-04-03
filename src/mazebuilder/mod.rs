extern crate rand;
mod maze;
mod dfs;
mod kruskal;

use std::collections::HashSet;

pub enum Generator {
    DFS { height: u32, width: u32 },
    Kruskal { height: u32, width: u32 },
}

pub fn generate_maze(gen: Generator) -> HashSet<(u32, u32)> {
    let maze = match gen {
        Generator::DFS { height, width } => dfs::recursive_backtracker(height, width),
        Generator::Kruskal { height, width } => kruskal::kruskal(height, width),
    };

    maze
}
