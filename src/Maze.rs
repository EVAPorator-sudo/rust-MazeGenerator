pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub row_list: Vec<Row>,
}

pub struct Row {
    pub width: usize,
    pub y_positon: usize,
    pub cell_list: Vec<Cell>,
}

pub struct Cell {
    pub position: [usize; 2],
    pub walls: [i8; 3],
}

impl Cell {
    pub fn new(XPos: usize, YPos: usize) -> Self {
        Cell {
            position: [XPos, YPos],
            walls: [0, 0, 0],
        }
    }
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
