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

    const cases: [[usize; 2]; 6] = [[0, 0], [10, 10], [50, 10], [10, 50], [50, 0], [10, 50]];

    #[test]
    fn cell_constructor() {
        for case in cases {
            Cell::new(case[0], case[1]);
        }
    }

    #[test]
    fn row_constructor() {
        for case in cases {
            Row::new(case[0], case[1]);
        }
    }

    #[test]
    fn grid_constructor() {
        for case in cases {
            Grid::new(case[0], case[1]);
        }
    }

    #[test]
    fn cell_assignment() {
        for case in cases {
            let cell = Cell::new(case[0], case[1]);

            assert_eq!(cell.position, case);
            assert_eq!(cell.walls, [true; 3]);
        }
    }

    #[test]
    fn row_assignment() {
        for case in cases {
            let row = Row::new(case[0], case[1]);

            assert_eq!(row.width, case[0]);
            assert_eq!(row.y_positon, case[1]);
            assert_eq!(row.cell_list.len(), case[0]);

            if (case[0] != 0 && case[1] != 0) {
                let test_cell = row.cell_list.get(0).unwrap();
                let expected_coordinates = [0, case[1]];
                assert_eq!(test_cell.position, expected_coordinates);
            }
        }
    }

    #[test]
    fn grid_assignment() {
        for case in cases {
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
}
