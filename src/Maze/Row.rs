use super::Cell::*;

pub struct Row {
    pub width: usize,
    pub y_positon: usize,
    pub cell_list: Vec<Cell>,
}

impl Row {
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
}
