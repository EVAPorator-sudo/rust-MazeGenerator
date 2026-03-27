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
