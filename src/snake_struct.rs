
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CellType {
    EmptyCell,
    Head,
    FHead,
    Body,
    FBody,
    Tail,
    Food,
}

#[derive(Clone,Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Snake {
    pub ft: f32,
    pub direction: char,
    pub head_x: i32, 
    pub head_y: i32, 
    pub tail_x: i32, 
    pub tail_y: i32, 
    pub size: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct GS {
    pub cell: CellType,
    pub direction: char,
}
