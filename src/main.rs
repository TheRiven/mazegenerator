extern crate mazegenerator;

fn main() {
    let mut args: Vec<String> = vec![];

    for (i, v) in std::env::args().enumerate() {
        if i != 0 {
            args.push(v);
        }
    }

    if args.len() < 2 {
        println!("Not enough arguments, make sure to include a Height and Width, in that order.");
        std::process::exit(1);
    }

    let height = match args[0].parse::<u32>() {
        Ok(num) => num,
        Err(err) => {
            println!("{}", err);
            println!("Make sure you have entered a number for height! defaulting to 128.");
            128
        }
    };

    let width = match args[1].parse::<u32>() {
        Ok(num) => num,
        Err(err) => {
            println!("{}", err);
            println!("Make sure you have entered a number for width! defaulting to 128.");
            128
        }
    };

    let _maze = mazegenerator::create_and_save_maze(height, width);
}
