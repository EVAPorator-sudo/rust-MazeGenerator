#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(nonstandard_style)]

pub mod Maze;

pub enum directions {
    left,
    right,
    up,
    down,
}

#[cfg(test)]
mod MazeTests {

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
        grid.Merge([5, 5], crate::directions::left);
        grid.Merge([5, 5], crate::directions::right);
        grid.Merge([5, 5], crate::directions::up);
        grid.Merge([5, 5], crate::directions::down);
    }
}
