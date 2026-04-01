use std::collections::HashSet;

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

    pub fn get_cell(&self, coords: &[usize; 2]) -> &Cell {
        self.row_list
            .get(coords[1])
            .unwrap()
            .cell_list
            .get(coords[0])
            .unwrap()
    }

    fn get_mut_cell(&mut self, coords: &[usize; 2]) -> &mut Cell {
        self.row_list
            .get_mut(coords[1])
            .unwrap()
            .cell_list
            .get_mut(coords[0])
            .unwrap()
    }

    pub fn Merge(&mut self, coords: [usize; 2], direction: &directions) {
        match direction {
            left => {
                self.get_mut_cell(&coords).walls[0] = false;
                self.get_mut_cell(&[coords[0] - 1, coords[1]]).walls[2] = false;
            }

            right => {
                self.get_mut_cell(&coords).walls[2] = false;
                self.get_mut_cell(&[coords[0] + 1, coords[1]]).walls[0] = false;
            }

            down => {
                self.get_mut_cell(&coords).walls[1] = false;
            }

            up => {
                self.get_mut_cell(&[coords[0], coords[1] - 1]).walls[1] = false;
            }
        }
    }

    pub fn get_all_cells(&self) -> Vec<[usize; 2]> {
        let mut cells = Vec::new();

        for row in self.row_list.iter() {
            for cell in row.cell_list.iter() {
                cells.push(cell.position);
            }
        }

        cells
    }

    pub fn find_movable_neighbours(&self, current: &[usize; 2]) -> Vec<[usize; 2]> {
        let mut neighbours = Vec::new();
        let walls = self.get_cell(current).walls;
        if !walls[0] {
            neighbours.push(self.get_cell(&[current[0] - 1, current[1]]).position);
        }
        if !walls[1] {
            neighbours.push(self.get_cell(&[current[0], current[1] + 1]).position);
        }
        if !walls[2] {
            neighbours.push(self.get_cell(&[current[0] + 1, current[1]]).position);
        }
        if current[1] > 0 {
            if !self.get_cell(&[current[0], current[1] - 1]).walls[1] {
                neighbours.push(self.get_cell(&[current[0], current[1] - 1]).position);
            }
        }
        neighbours
    }

    pub fn find_neighbours(
        &self,
        coords: &[usize; 2],
        visited: &HashSet<[usize; 2]>,
    ) -> Vec<directions> {
        let mut neighbours: Vec<directions> = Vec::new();

        if coords[0] > 0 {
            if !visited.contains(&[coords[0] - 1, coords[1]]) {
                neighbours.push(left);
            }
        }
        if coords[0] < self.width - 1 {
            if !visited.contains(&[coords[0] + 1, coords[1]]) {
                neighbours.push(right);
            }
        }
        if coords[1] > 0 {
            if !visited.contains(&[coords[0], coords[1] - 1]) {
                neighbours.push(up);
            }
        }
        if coords[1] < self.height - 1 {
            if !visited.contains(&[coords[0], coords[1] + 1]) {
                neighbours.push(down);
            }
        }
        neighbours
    }
}
