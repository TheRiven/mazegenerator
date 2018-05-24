extern crate rand;

use std::collections::HashSet;
use std::time::Instant;

mod imagecontrol;
mod mazebuilder;
mod mazesolver;

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

    let generator = select_maze_generator(height, width);
    let maze = mazebuilder::generate_maze(generator);
    println!("Maze Generated in {:?}", timer.elapsed());

    save_maze(height, width, &maze);

    maze
}

pub fn solve_maze(height: u32, width: u32, maze: &HashSet<(u32, u32)>) {
    println!("Solving Maze...");

    let height = if height % 2 == 0 { height + 1 } else { height };

    let width = if width % 2 == 0 { width + 1 } else { width };

    let start_point = (1, 1);
    let end_point = (width - 2, height - 2);

    println!(
        "Finding path from point {:?} to {:?}",
        start_point, end_point
    );

    let timer = Instant::now();
    let path = mazesolver::solve_maze(select_maze_solver(), &start_point, end_point, maze);

    if path == None {
        println!("Something went wrong and no path was found!");
    } else {
        println!("We have a path!");
        println!("Maze solved in {:?}", timer.elapsed());
        save_solved_maze(height, width, maze, path.unwrap());
    }    
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

fn select_maze_generator(height: u32, width: u32) -> mazebuilder::Generator {
    use std::io;
    let mut input = String::new();

    println!("Which maze generator do you want to use?");
    println!("1. Depth First Search,");
    println!("2. Kruskal,");
    io::stdin()
        .read_line(&mut input)
        .expect("select_maze_generator -- unable to parse console input!");

    let option = match input.trim().parse::<u32>() {
        Ok(num) => num,
        Err(err) => {
            println!("Please enter a number! Error: {}", err);
            std::process::exit(1);
        }
    };

    match option {
        1 => mazebuilder::Generator::DFS {height, width},
        2 => mazebuilder::Generator::Kruskal {height, width},
        _ => {
            println!("unrecognised option {}, defaulting to DFS", option);
            mazebuilder::Generator::DFS {height, width}
        }
    }
}

fn select_maze_solver() -> mazesolver::Solver {
    use std::io;
    let mut input = String::new();

    println!("Which maze solver do you want to use?");
    println!("1. Breadth First Search,");
    println!("2. Left-turn,");
    io::stdin()
        .read_line(&mut input)
        .expect("select_maze_solver -- unable to parse console input!");

    let option = match input.trim().parse::<u32>() {
        Ok(num) => num,
        Err(err) => {
            println!("Please enter a number! Error: {}", err);
            std::process::exit(1);
        }
    };

    match option {
        1 => mazesolver::Solver::BFS,
        2 => mazesolver::Solver::LeftTurn,
        _ => {
            println!("unrecognised option {}, defaulting to BFS", option);
            mazesolver::Solver::BFS
        }
    }
}
