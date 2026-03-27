use super::Row::*;

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
}
