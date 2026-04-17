//! A single row of cells within the maze grid.

use crate::directions;
use crate::directions::*;

use super::Cell::*;

/// A single horizontal row of [`Cell`]s within the [`crate::Maze::Grid::Grid`].
///
/// Rows are indexed from top to bottom, with `y_positon` indicating
/// the row's vertical position within the grid.
pub struct Row {
    /// The number of [`Cell`]s in this row.
    pub width: usize,
    /// The y position of this row within the [`crate::Maze::Grid::Grid`].
    pub y_positon: usize,
    /// The [`Cell`]s contained within this row.
    pub cell_list: Vec<Cell>,
}

impl Row {
    /// Constructor for the [`Row`] struct.
    ///
    /// # Arguments
    ///
    /// * `Width` - The number of [`Cell`]s to create in this row
    /// * `y_position` - The y position of this row within the [`crate::Maze::Grid::Grid`]
    ///
    /// # Returns
    ///
    /// A new [`Row`] containing `Width` [`Cell`]s, all with walls intact.
    ///
    /// # Examples
    ///
    /// ```
    /// use MazeGenerator::Maze::Row::Row;
    ///
    /// let row = Row::new(5, 2);
    /// assert_eq!(row.width, 5);
    /// assert_eq!(row.y_positon, 2);
    /// assert_eq!(row.cell_list.len(), 5);
    /// ```
    pub fn new(Width: usize, y_position: usize) -> Self {
        let mut cell_List: Vec<Cell> = Vec::new();

        for index in 0..Width {
            cell_List.push(Cell::new(index, y_position));
        }

        Row {
            width: Width,
            y_positon: y_position,
            cell_list: cell_List,
        }
    }

    /// Returns a mutable reference to a [`Cell`] at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The horizontal index of the target [`Cell`] within the row
    ///
    /// # Returns
    ///
    /// A mutable reference to the [`Cell`] at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    fn get_mut_cell(&mut self, index: usize) -> &mut Cell {
        self.cell_list.get_mut(index).unwrap()
    }

    /// Returns a reference to a [`Cell`] at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The horizontal index of the target [`Cell`] within the row
    ///
    /// # Returns
    ///
    /// A reference to the [`Cell`] at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    pub fn get_cell(&mut self, index: usize) -> &Cell {
        self.cell_list.get(index).unwrap()
    }

    /// Merges two adjacent cells within the row by removing the wall between them.
    ///
    /// Each wall is shared between two cells. Merging in a given direction
    /// removes the wall on both sides of the boundary where applicable.
    ///
    /// # Arguments
    ///
    /// * `cell_index` - The index of the starting [`Cell`]
    /// * `direction` - The direction of the neighbouring [`Cell`] to merge with
    ///
    /// # Panics
    ///
    /// Panics if merging would go out of bounds or if `up` is used.
    ///
    /// # Examples
    ///
    /// ```
    /// use MazeGenerator::Maze::Row::Row;
    /// use MazeGenerator::directions;
    ///
    /// let mut row = Row::new(5, 0);
    /// row.Merge(2, &directions::right);
    ///
    /// assert!(!row.get_cell(2).walls[2]);
    /// assert!(!row.get_cell(3).walls[0]);
    /// ```
    pub fn Merge(&mut self, cell_index: usize, direction: &directions) {
        match direction {
            left => {
                self.get_mut_cell(cell_index).walls[0] = false;
                self.get_mut_cell(cell_index - 1).walls[2] = false;
            }

            right => {
                self.get_mut_cell(cell_index).walls[2] = false;
                self.get_mut_cell(cell_index + 1).walls[0] = false;
            }

            down => {
                self.get_mut_cell(cell_index).walls[1] = false;
            }

            up => {
                panic!("row cannot merge upwards")
            }
        }
    }
}
