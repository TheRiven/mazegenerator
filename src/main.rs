extern crate mazegenerator;
use std::io;

fn main() {
    // Check if the user wants to generate and solve a maze, or
    // just generate one.
    let mode = get_mode_option();

    // Get the size of the maze to be generated
    let (height, width) = get_maze_size();

    match mode {
        1 => {
            let maze = mazegenerator::create_and_save_maze(height, width);
            mazegenerator::solve_maze(height, width, &maze);
        }
        2 => {
            let _maze = mazegenerator::create_and_save_maze(height, width);
        }
        _ => {}
    }
}

fn get_mode_option() -> u32 {
    let mut input = String::new();

    println!("Do you want to:");
    println!("1. Generate a maze and solve it.");
    println!("2. Generate a maze.");    
    io::stdin()
        .read_line(&mut input)
        .expect("get_mode_option -- unable to parse console input!");

    let option = match input.trim().parse::<u32>() {
        Ok(num) => num,
        Err(err) => {
            println!("Please enter a number to select the mode! Error: {}", err);
            std::process::exit(1);
        }
    };

    option
}

fn get_maze_size() -> (u32, u32) {
    let mut input = String::new();

    println!("Please enter a height for the maze.");
    io::stdin()
        .read_line(&mut input)
        .expect("get_maze_size -- unable to parse console input!");
    
    let height = match input.trim().parse::<u32>() {
        Ok(num) => num,
        Err(err) => {
            println!("Please enter a number, Error: {}", err);
            std::process::exit(1);
        }
    };

    let mut input = String::new();

    println!("Please enter a width for the maze.");
    io::stdin()
        .read_line(&mut input)
        .expect("get_maze_size -- unable to parse console input!");
    
    let width = match input.trim().parse::<u32>() {
        Ok(num) => num,
        Err(err) => {
            println!("Please enter a number, Error: {}", err);
            std::process::exit(1);
        }
    };

    (height, width)
}
