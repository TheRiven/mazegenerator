extern crate mazegenerator;

fn main() {
    let height = 4096;
    let width = 4096;

    let height = if height % 2 == 0 {
        println!("+1 to height!");
        height + 1
    } else {
        height
    };

    let width = if width % 2 == 0 {
        println!("+1 to width!");
        width + 1
    } else {
        width
    };

    let maze = mazegenerator::create_maze(height, width);

    mazegenerator::save_maze(height, width, maze);
}
