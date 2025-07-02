
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BCell {
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
    pub head: Point,
    pub tail: Segment,
    pub size: u32,
}

#[derive(Clone, Debug)]
pub struct Segment {
    pub next: Option<Box<Segment>>,
    pub coord: Point,
}
