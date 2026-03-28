use crate::{Maze::Cell::Cell, directions};

use super::Row::*;
use crate::directions::*;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub row_list: Vec<Row>,
}

impl Grid {
    pub fn new(Width: usize, Height: usize) -> Self {
        let mut row_List: Vec<Row> = Vec::new();

        for index in 0..Height {
            row_List.push(Row::new(Width, index));
        }

        Grid {
            width: Width,
            height: Height,
            row_list: row_List,
        }
    }

    pub fn Merge(&mut self, cell: &mut Cell, direction: directions) {
        match direction {
            left => {
                cell.walls[0] = false;
                let row = self.row_list.get_mut(cell.position[1]).unwrap();
                let adjacent_cell = row.cell_list.get_mut(cell.position[0] - 1).unwrap();
                adjacent_cell.walls[2] = false;
            }

            right => {
                cell.walls[2] = false;
                let row = self.row_list.get_mut(cell.position[1]).unwrap();
                let adjacent_cell = row.cell_list.get_mut(cell.position[0] + 1).unwrap();
                adjacent_cell.walls[0] = false;
            }

            down => {
                cell.walls[1] = false;
            }

            up => {
                let row = self.row_list.get_mut(cell.position[1] - 1).unwrap();
                let adjacent_cell = row.cell_list.get_mut(cell.position[0]).unwrap();
                adjacent_cell.walls[1] = false;
            }
        }
    }
}
