extern crate rand;

use std::collections::HashSet;
use std::time::Instant;

mod mazebuilder;
mod mazesolver;
mod imagecontrol;

pub fn create_and_save_maze(maze_height: u32, maze_width: u32) -> HashSet<(u32, u32)> {
    println!(
        "Generating Maze with height {} and width {}",
        maze_height, maze_width
    );

    let height = if maze_height % 2 == 0 {
        maze_height + 1
    } else {
        maze_height
    };

    let width = if maze_width % 2 == 0 {
        maze_width + 1
    } else {
        maze_width
    };

    // Setup Timer
    let timer = Instant::now();

    let maze = mazebuilder::generate_maze(mazebuilder::Generator::DFS { height, width });
    println!("Maze Generated in {:?}", timer.elapsed());

    save_maze(height, width, &maze);
    solve_maze(height, width, &maze);

    maze
}

pub fn solve_maze(height: u32, width: u32, maze: &HashSet<(u32, u32)>) {
    println!("Solving Maze...");

    let height = if height % 2 == 0 { height + 1 } else { height };

    let width = if width % 2 == 0 { width + 1 } else { width };

    let start_point = (1, 1);
    let end_point = (height - 2, width - 2);

    println!(
        "Finding path from point {:?} to {:?}",
        start_point, end_point
    );

    let timer = Instant::now();
    let path = mazesolver::breadth_first_search(&start_point, end_point, maze);

    if path == None {
        println!("Something went wrong and no path was found!");
    } else {
        println!("We have a path!");
    }

    println!("Maze solved in {:?}", timer.elapsed());

    save_solved_maze(height, width, maze, path.unwrap());
}

fn save_maze(height: u32, width: u32, maze: &HashSet<(u32, u32)>) {
    println!("Saving image with height {} and width {}", height, width);
    let timer = Instant::now();
    imagecontrol::generate_image(height as u32, width as u32, maze);
    println!("Image saved in {:?}", timer.elapsed());
}

fn save_solved_maze(height: u32, width: u32, maze: &HashSet<(u32, u32)>, path: Vec<&(u32, u32)>) {
    println!("Saving image with height {} and width {}", height, width);
    let timer = Instant::now();
    imagecontrol::generate_solved_image(height as u32, width as u32, maze, path);
    println!("Image saved in {:?}", timer.elapsed());
}
