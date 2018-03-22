extern crate mazegenerator;

fn main() {
    println!("Generating Maze...");
    let height = 4001;
    let width = 4001;
    let maze = mazegenerator::generate_maze(height, width);
    println!("Maze Generated", );

    println!("Saving image...", );
    mazegenerator::generate_image(height as u32, width as u32, maze);
    println!("Image saved!", );
}
