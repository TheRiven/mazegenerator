extern crate rand;

use std::collections::HashMap;

mod maze;
mod mazebuilder;
mod imagecontrol;

pub fn create_and_save_maze(maze_height: u64, maze_width: u64) -> HashMap<(u64, u64), maze::Node> {
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

    let maze = mazebuilder::generate_maze(height, width);
    println!("Maze Generated");

    save_maze(height, width, &maze);

    maze
}

fn save_maze(height: u64, width: u64, maze: &HashMap<(u64, u64), maze::Node>) {
    println!("Saving image with height {} and width {}", height, width);
    imagecontrol::generate_image(height as u32, width as u32, maze);
    println!("Image saved!");
}
