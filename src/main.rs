use MazeGenerator::{Draw::grid_draw, Generator::*, Maze::Grid::Grid};
use core::panic;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        panic!("invalid args")
    }

    let mode = &args[1];
    let width: usize = args[2].parse().expect("width must be a positive integer");
    let height: usize = args[3].parse().expect("height must be a positive integer");

    let grid: Grid;
    let path;

    let grid;
    match mode.as_str() {
        "e" => {
            grid = Ellers(Grid::new(width, height));
            path = &args[4];
        }
        "g" => {
            grid = Growing_Tree(
                Grid::new(width, height),
                args[4]
                    .parse()
                    .expect("weight must be a decimal between 0 and 1"),
            );
            path = &args[5];
        }
        _ => panic!("invalid algorithm"),
    }

    fs::write(path, grid_draw(&grid)).expect("error generating maze");
}
