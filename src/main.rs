use MazeGenerator::{Draw::*, Generator::*, Maze::Grid::Grid};
use core::panic;
use std::{env, fs, io::stdin};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        args.push(select_algorithm());
        args.push(select_dimension("width"));
        args.push(select_dimension("height"));

        if args[1] == "--g" {
            args.push(select_weight());
        }

        args.push(select_output_path());
    }

    handle(args);
}

fn select_algorithm() -> String {
    loop {
        println!("Welcome to MazeGenerator!\nChoose algorithm:\n1. Ellers\n2. Growing Tree");
        let input = read_input();
        match input.trim() {
            "1" => return "--e".to_string(),
            "2" => return "--g".to_string(),
            _ => println!("Invalid input, please enter 1 or 2."),
        }
    }
}

fn select_dimension(name: &str) -> String {
    loop {
        println!("Enter {} (1–1000):", name);
        let input = read_input();
        if let Ok(value) = input.trim().parse::<usize>() {
            if value >= 1 && value <= 1000 {
                return value.to_string();
            }
        }
        println!("Invalid input. Please enter a number between 1 and 1000.");
    }
}

fn select_weight() -> String {
    loop {
        println!("Enter Growing Tree weight (0.0–1.0):");
        let input = read_input();
        if let Ok(value) = input.trim().parse::<f32>() {
            if (0.0..=1.0).contains(&value) {
                return value.to_string();
            }
        }
        println!("Invalid input. Please enter a decimal between 0 and 1.");
    }
}

fn select_output_path() -> String {
    println!("Enter output path (e.g., output.png or output.svg):");
    read_input().trim().to_string()
}

fn read_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn handle(args: Vec<String>) {
    if args.len() < 5 {
        panic!("Invalid arguments");
    }

    let width: usize = args[2].parse().expect("Width must be a positive integer");
    let height: usize = args[3].parse().expect("Height must be a positive integer");

    let grid = match args[1].as_str() {
        "--e" => Ellers(Grid::new(width, height)),
        "--g" => {
            if args.len() < 6 {
                panic!("Missing weight for Growing Tree");
            }
            Growing_Tree(
                Grid::new(width, height),
                args[4]
                    .parse()
                    .expect("Weight must be a decimal between 0 and 1"),
            )
        }
        _ => panic!("Invalid algorithm"),
    };

    let path = args.last().unwrap();

    if path.ends_with(".svg") {
        fs::write(path, grid_draw_svg(&grid)).expect("Error writing maze to disk");
    } else if path.ends_with(".png") || path.ends_with(".jpg") {
        grid_draw_img(&grid)
            .save(path)
            .expect("Error writing maze to disk");
    } else {
        panic!("Invalid file extension");
    }
}