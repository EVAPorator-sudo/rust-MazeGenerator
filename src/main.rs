use MazeGenerator::{
    Draw::grid_draw,
    Generator::Ellers,
    Maze::{Cell::Cell, Grid::Grid, Row::Row},
};

fn main() {
    let grid = Ellers(Grid::new(50, 50));

    grid_draw(&grid);
}
