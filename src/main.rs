extern crate mazegenerator;

fn main() {

    let mut args : Vec<String> = vec![];

    for (i, v) in std::env::args().enumerate() { 
        if i != 0 {
            args.push(v);
        }
    }

    if args.len() < 2 {
        println!("Not enough arguments, make sure to include a Width and Height, in that order.");
        std::process::exit(1);
    }

    let height = match args[0].parse::<u64>() {
        Ok(num) => num,
        Err(err) => {
            println!("{}", err);
            println!("Make sure you have entered a number for height! defaulting to 128.");
            128
        } 
    };

    let width = match args[1].parse::<u64>() {
        Ok(num) => num,
        Err(err) => {
            println!("{}", err);
            println!("Make sure you have entered a number for width! defaulting to 128.");
            128
        } 
    };

    let height = if height % 2 == 0 {
        height + 1
    } else {
        height
    };

    let width = if width % 2 == 0 {
        width + 1
    } else {
        width
    };

    let maze = mazegenerator::create_maze(height, width);

    mazegenerator::save_maze(height, width, maze);
}
