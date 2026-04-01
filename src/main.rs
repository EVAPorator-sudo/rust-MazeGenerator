use MazeGenerator::{Draw::*, Generator::*, Maze::Grid::Grid};
use core::panic;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        panic!("invalid args")
    }

    let width: usize = args[2].parse().expect("width must be a positive integer");
    let height: usize = args[3].parse().expect("height must be a positive integer");

    let grid = match args[1].as_str() {
        "--e" => {
            if args.len() < 5 {
                panic!("invalid number of parameters")
            }
            Ellers(Grid::new(width, height))
        }
        "--g" => {
            if args.len() < 6 {
                panic!("invalid number of parameters")
            }
            Growing_Tree(
                Grid::new(width, height),
                args[4]
                    .parse()
                    .expect("weight must be a decimal between 0 and 1"),
            )
        }
        _ => panic!("invalid algorithm"),
    };

    let path = args.last().unwrap();

    if path.contains(".svg") {
        fs::write(args.last().unwrap(), grid_draw_svg(&grid)).expect("error writing maze to disk");
    } else if path.contains(".png") || path.contains(".jpg") {
        grid_draw_img(&grid)
            .save(path)
            .expect("error writing maze to disk");
    } else {
        panic!("invalid file extension");
    }
}
