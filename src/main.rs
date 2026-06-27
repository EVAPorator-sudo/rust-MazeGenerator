use MazeGenerator::{
    Draw::*,
    Generator::*,
    Maze::Grid::Grid,
    Solver::{Astar, dijkstra},
};

use std::{env, fs, io::stdin};

// args layout:
// [0] binary name
// [1] flags e.g. "--ed" (algorithm + solve algorithm)
// [2] width
// [3] height
// [4] weighting (Growing Tree only)
// [4/5..] start x, start y, end x, end y (if solving)
// last: output path
// if no input is supplied the user will be asked to input each arg
fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        args.push(select_algorithm());
        args[1].push_str(select_solve_algorithm().as_str());
        args.push(select_dimension("width"));
        args.push(select_dimension("height"));

        if args[1].contains("--g") {
            args.push(select_weight());
        }

        if args[1].contains("d") || args[1].contains("a") {
            let coords =
                select_solution_coordinates(args[2].parse().unwrap(), args[3].parse().unwrap());

            for coord in coords {
                args.push(coord);
            }
        }

        args.push(select_output_path());
    }

    handle(args);
}

fn select_algorithm() -> String {
    loop {
        println!(
            "Welcome to MazeGenerator!\nChoose algorithm:\n1. Ellers\n2. Growing Tree\n3. Ellers (multithreaded)\n4. Kruskals"
        );
        let input = read_input();
        match input.trim() {
            "1" => return "--e".to_string(),
            "2" => return "--g".to_string(),
            "3" => return "--em".to_string(),
            "4" => return "--k".to_string(),
            _ => println!("Invalid input, please enter 1, 2, 3 or 4."),
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

fn select_solve_algorithm() -> String {
    loop {
        println!("Solution algorithm:\n0. no Solution\n1. Dijkstra's\n2. A*");
        match read_input().trim() {
            "0" => return "".to_string(),
            "1" => return "d".to_string(),
            "2" => return "a".to_string(),
            _ => println!("Invalid input, please enter 0, 1 or 2"),
        }
    }
}

fn select_solution_coordinates(width: usize, height: usize) -> [String; 4] {
    loop {
        println!("Enter start X:");
        let x1 = read_input().trim().parse::<usize>();

        println!("Enter start Y:");
        let y1 = read_input().trim().parse::<usize>();

        println!("Enter end X:");
        let x2 = read_input().trim().parse::<usize>();

        println!("Enter end Y:");
        let y2 = read_input().trim().parse::<usize>();

        if let (Ok(x1), Ok(y1), Ok(x2), Ok(y2)) = (x1, y1, x2, y2) {
            if x1 < width && y1 < height && x2 < width && y2 < height {
                return [
                    x1.to_string(),
                    y1.to_string(),
                    x2.to_string(),
                    y2.to_string(),
                ];
            }
        }

        println!(
            "Invalid input. Coordinates must be within (0..{}, 0..{})",
            width - 1,
            height - 1
        );
    }
}

fn handle(args: Vec<String>) {
    if args.len() < 5 {
        panic!("Invalid arguments");
    }

    let width: usize = args[2].parse().expect("Width must be a positive integer");
    let height: usize = args[3].parse().expect("Height must be a positive integer");

    let chars: Vec<char> = args[1].chars().collect();

    let is_multithreaded = chars.get(3) == Some(&'m');
    let solve_char_index = if is_multithreaded { 4 } else { 3 };

    let coord_starting;

    let img_extensions = [
        "png", "jpg", "jpeg", "bmp", "gif", "ico", "tiff", "tif", "webp", "tga", "qoi", "svg",
    ];

    let path = args.last().unwrap();
    let extension = path.split('.').last().unwrap_or("");

    if !img_extensions.contains(&extension) {
        panic!("invalid file extension");
    }

    validate_dimensions(width, height, extension);

    let grid = match chars[2] {
        'g' => {
            coord_starting = 5;
            Growing_Tree(
                Grid::new(width, height),
                args[4]
                    .parse()
                    .expect("Weighting must be a decimal between 0 and 1"),
            )
        }
        'e' => {
            coord_starting = 4;
            if is_multithreaded {
                multi_thread_ellers(Grid::new(width, height))
            } else {
                Ellers(Grid::new(width, height))
            }
        }
        'k' => {
            coord_starting = 4;
            Kruskal(Grid::new(width, height))
        }
        _ => panic!("invalid algorithm"),
    };

    if chars.len() > solve_char_index {
        let start: [usize; 2] = [
            args[coord_starting]
                .parse()
                .expect("Please enter a valid x coordinate in the maze"),
            args[coord_starting + 1]
                .parse()
                .expect("Please enter a valid y coordinate in the maze"),
        ];

        let end: [usize; 2] = [
            args[coord_starting + 2]
                .parse()
                .expect("Please enter a valid x coordinate in the maze"),
            args[coord_starting + 3]
                .parse()
                .expect("Please enter a valid y coordinate in the maze"),
        ];

        let solution = match chars[solve_char_index] {
            'd' => dijkstra(start, end, &grid),
            'a' => Astar(start, end, &grid),
            _ => panic!("invalid algorithm"),
        };

        if path.ends_with(".svg") {
            fs::write(path, solve_draw_svg(&grid, &solution)).expect("Error writing maze to disk");
        } else {
            solve_draw_img(&grid, &solution)
                .save(path)
                .expect("Error writing maze to disk");
        }
    } else {
        if path.ends_with(".svg") {
            fs::write(path, grid_draw_svg(&grid)).expect("Error writing maze to disk");
        } else {
            grid_draw_img(&grid)
                .save(path)
                .expect("Error writing maze to disk");
        }
    }
}

fn get_format_limits(extension: &str) -> Option<(u32, u32)> {
    match extension {
        "png" => Some((u32::MAX, u32::MAX)),
        "jpg" | "jpeg" => Some((65535, 65535)),
        "webp" => Some((16383, 16383)),
        "bmp" => Some((u32::MAX, u32::MAX)),
        "gif" => Some((65535, 65535)),
        "tiff" | "tif" => Some((u32::MAX, u32::MAX)),
        "ico" => Some((256, 256)),
        "tga" => Some((65535, 65535)),
        "qoi" => Some((u32::MAX, u32::MAX)),
        "svg" => Some((u32::MAX, u32::MAX)),
        _ => None,
    }
}

fn validate_dimensions(width: usize, height: usize, extension: &str) {
    let img_width = (width * 20 + 2 * 1) as u32;
    let img_height = (height * 20 + 2 * 1) as u32;
    let limits = get_format_limits(extension).unwrap();

    if limits.0 < img_width || limits.1 < img_height {
        panic!("Dimensions exceed file extension limits");
    }
}
