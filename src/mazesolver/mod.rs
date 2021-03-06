mod bfs;
mod leftturn;
mod a_star;

use std::collections::HashSet;

pub enum Solver {
    BFS,
    LeftTurn,
    AStar,
}

pub fn solve_maze<'a>(
    solver: Solver,
    start: (u32, u32),
    end: (u32, u32),
    maze: &'a HashSet<(u32, u32)>,
) -> Option<Vec<&'a (u32, u32)>> {
    let path = match solver {
        Solver::BFS => bfs::breadth_first_search(start, end, maze),
        Solver::LeftTurn => leftturn::left_first(start, end, maze),
        Solver::AStar => a_star::solve(start, end, maze),
    };

    path
}
