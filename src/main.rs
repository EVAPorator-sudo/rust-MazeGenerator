use MazeGenerator::{Draw::grid_draw, Generator::*, Maze::Grid::Grid};
use std::fs;

fn main() {
    let grid = Ellers(Grid::new(1000, 1000));
    let svg = grid_draw(&grid);
    fs::write("maze_image_e.svg", svg).expect("failed to write SVG");

    let grid = Growing_Tree(Grid::new(1000, 1000), 0.8);
    let svg = grid_draw(&grid);
    fs::write("maze_image_gt.svg", svg).expect("failed to write SVG");
}
