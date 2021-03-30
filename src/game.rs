pub struct game {
    pub board: Board,
}
pub enum Cell {
    Wall,
    Hall,
}

pub enum Coordinate {
    pub X: u32,
    pub y: u32
}

pub struct Board {
    pub dim_x: u32,
    pub dim_y: u32,
    pub cells: Vec<Vec<Cell>>,
    pub food: Vec<Coordinate>,
}
