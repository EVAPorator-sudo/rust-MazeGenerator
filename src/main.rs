use MazeGenerator::{Draw::*, Generator::*, Maze::Grid::Grid};
use core::panic;
use std::{env, fs, io::stdin, result};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!(
            "Welcome to MazeGenerator !\n what algorithm would you like to use ?\n1. Ellers\n2. Growing Tree"
        );
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("please enter an input");
        match input.as_str().trim() {
            "1" => args.push("--e".to_string()),

            "2" => args.push("--g".to_string()),

            _ => panic!("invalid input. please enter 1 or 2."),
        }
        println!("what are the dimensions of the maze ?\nenter a width between 1 and 1000");
        input.clear();
        stdin()
            .read_line(&mut input)
            .expect("please enter an input");
        let result: usize = input
            .trim()
            .parse()
            .expect("invalid input. please enter an integer");
        if result > 1000 || result < 1 {
            panic!("invalid input. please enter a value between 1 and 1000")
        }
        args.push(input.trim().to_string());
        println!("enter a height between 1 and 1000");
        input.clear();
        stdin()
            .read_line(&mut input)
            .expect("please enter an input");
        let result: usize = input
            .trim()
            .parse()
            .expect("invalid input. please enter an integer");
        if result > 1000 || result < 1 {
            panic!("invalid input. please enter a value between 1 and 1000")
        }
        args.push(input.trim().to_string());
        if args[1] == "--g" {
            println!("please enter a Growing Tree weight between 0 and 1");
            input.clear();
            stdin()
                .read_line(&mut input)
                .expect("please enter an input");
            let result: f32 = input
                .trim()
                .parse()
                .expect("invalid input. please enter a decimal");
            if result < 0.0 || result > 1.0 {
                panic!("invalid input. please enter a value between 0 and 1")
            }
            args.push(input.trim().to_string());
        }
        println!("please enter the path you want to output to\ne.g output.png");
        input.clear();
        stdin()
            .read_line(&mut input)
            .expect("please enter an input");
        args.push(input.trim().to_string());
    }
    handle(args);
}

fn handle(args: Vec<String>) {
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
