#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(nonstandard_style)]

pub mod Draw;
pub mod Generator;
pub mod Maze;
pub mod Solver;
pub enum directions {
    left,
    right,
    up,
    down,
}

#[cfg(test)]
mod draw_test {
    use crate::{
        Draw::*,
        Generator::{Ellers, Growing_Tree},
        Maze::Grid::Grid,
    };
    use std::fs;

    const CASES: [[usize; 2]; 6] = [
        [0, 0],
        [10, 10],
        [100, 100],
        [1000, 1000],
        [1000, 10],
        [10, 1000],
    ];

    #[test]
    fn no_path() {
        for case in CASES {
            let grid = Ellers(Grid::new(case[0], case[1]));
            let img = grid_draw_img(&grid);
            let svg = grid_draw_svg(&grid);
            img.save("maze_image_e.png").expect("failed to write png");
            img.save("maze_image_e.jpg").expect("failed to write jpg");
            fs::write("maze_image_e.svg", svg).expect("failed to write SVG");

            let grid = Growing_Tree(Grid::new(case[0], case[1]), 0.5);
            let img = grid_draw_img(&grid);
            let svg = grid_draw_svg(&grid);
            img.save("maze_image_gt.png").expect("failed to write png");
            img.save("maze_image_gt.jpg").expect("failed to write jpg");
            fs::write("maze_image_gt.svg", svg).expect("failed to write SVG");
        }
    }
}

#[cfg(test)]
mod algorithm_tests {
    const CASES: [[usize; 2]; 6] = [[0, 0], [10, 10], [50, 10], [10, 50], [50, 0], [10, 50]];

    use crate::Generator::{Ellers, Growing_Tree};
    use crate::Maze::Grid::Grid;
    use crate::Solver::{Astar, dijkstra};

    #[test]
    fn eller_test() {
        for case in CASES {
            Ellers(Grid::new(case[0], case[1]));
        }
    }

    #[test]
    fn growing_tree_test() {
        for case in CASES {
            Growing_Tree(Grid::new(case[0], case[1]), 0.5);
        }
    }

    #[test]
    fn dijkstra_test() {
        let cases: [[usize; 2]; 4] = [[2, 2], [10, 10], [100, 100], [1000, 1000]];
        for case in cases {
            let grid = Ellers(Grid::new(case[0], case[1]));
            dijkstra([0, 0], [case[1] - 1, case[1] - 1], &grid);
        }
    }

    #[test]
    fn Astar_test() {
        let cases: [[usize; 2]; 4] = [[2, 2], [10, 10], [100, 100], [1000, 1000]];
        for case in cases {
            let grid = Ellers(Grid::new(case[0], case[1]));
            Astar([0, 0], [case[1] - 1, case[1] - 1], &grid);
        }
    }
}

#[cfg(test)]
mod maze_tests {

    use crate::Maze::{Cell::Cell, Grid::Grid, Row::Row};

    const CASES: [[usize; 2]; 6] = [[0, 0], [10, 10], [50, 10], [10, 50], [50, 0], [10, 50]];

    #[test]
    fn cell_constructor() {
        for case in CASES {
            Cell::new(case[0], case[1]);
        }
    }

    #[test]
    fn row_constructor() {
        for case in CASES {
            Row::new(case[0], case[1]);
        }
    }

    #[test]
    fn grid_constructor() {
        for case in CASES {
            Grid::new(case[0], case[1]);
        }
    }

    #[test]
    fn cell_assignment() {
        for case in CASES {
            let cell = Cell::new(case[0], case[1]);

            assert_eq!(cell.position, case);
            assert_eq!(cell.walls, [true; 3]);
        }
    }

    #[test]
    fn row_assignment() {
        for case in CASES {
            let row = Row::new(case[0], case[1]);

            assert_eq!(row.width, case[0]);
            assert_eq!(row.y_positon, case[1]);
            assert_eq!(row.cell_list.len(), case[0]);

            if case[0] != 0 && case[1] != 0 {
                let test_cell = row.cell_list.get(0).unwrap();
                let expected_coordinates = [0, case[1]];
                assert_eq!(test_cell.position, expected_coordinates);
            }
        }
    }

    #[test]
    fn grid_assignment() {
        for case in CASES {
            let grid = Grid::new(case[0], case[1]);

            assert_eq!(grid.width, case[0]);
            assert_eq!(grid.height, case[1]);
            assert_eq!(grid.row_list.len(), case[1]);

            if case[0] != 0 && case[1] != 0 {
                let test_row = grid.row_list.get(0).unwrap();
                assert_eq!(test_row.width, case[0]);
                assert_eq!(test_row.y_positon, 0);
            }
        }
    }

    #[test]
    fn merge_test() {
        let mut grid = Grid::new(10, 10);
        grid.Merge([5, 5], &crate::directions::left);
        grid.Merge([5, 5], &crate::directions::right);
        grid.Merge([5, 5], &crate::directions::up);
        grid.Merge([5, 5], &crate::directions::down);
    }
}
