pub struct Grid {
    width: i32,
    height: i32,
    row_list: Vec<Row>,
}

pub struct Row {
    width: i32,
    y_positon: i32,
    cell_list: Vec<Cell>,
}

pub struct Cell {
    position: [i32; 2],
    walls: [i8; 3],
}

impl Cell {
    pub fn new(XPos: i32, YPos: i32) -> Self {
        Cell {
            position: [XPos, YPos],
            walls: [0, 0, 0],
        }
    }
}

impl Row {
    pub fn new(Width: i32, y_position: i32) -> Self {
        let mut cell_List: Vec<Cell> = Vec::new();

        for index in 0..(Width - 1) {
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
    pub fn new(Width: i32, Height: i32) -> Self {
        let mut row_List: Vec<Row> = Vec::new();

        for index in 0..(Width - 1) {
            row_List.push(Row::new(Width, index));
        }

        Grid {
            width: Width,
            height: Height,
            row_list: row_List,
        }
    }
}
