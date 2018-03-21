extern crate mazegenerator;

fn main() {
    println!("Hello, world!");
    println!("Generating Maze...");
    let _maze = mazegenerator::generate_maze(30, 30);
    println!("Maze Generated", );
}
