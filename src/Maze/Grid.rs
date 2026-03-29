use std::num::ParseFloatError;

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

    pub fn get_cell(&self, coords: [usize; 2]) -> &Cell {
        self.row_list
            .get(coords[1])
            .unwrap()
            .cell_list
            .get(coords[0])
            .unwrap()
    }

    fn get_mut_cell(&mut self, coords: [usize; 2]) -> &mut Cell {
        self.row_list
            .get_mut(coords[1])
            .unwrap()
            .cell_list
            .get_mut(coords[0])
            .unwrap()
    }

    pub fn Merge(&mut self, coords: [usize; 2], direction: directions) {
        match direction {
            left => {
                self.get_mut_cell(coords).walls[0] = false;
                self.get_mut_cell([coords[0] - 1, coords[1]]).walls[2] = false;
            }

            right => {
                self.get_mut_cell(coords).walls[2] = true;
                self.get_mut_cell([coords[0] + 1, coords[1]]).walls[0] = false;
            }

            down => {
                self.get_mut_cell(coords).walls[1] = false;
            }

            up => {
                self.get_mut_cell([coords[0], coords[1] - 1]).walls[1] = false;
            }
        }
    }
}
