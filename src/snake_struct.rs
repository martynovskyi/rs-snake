
#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Snake {
    pub ft: f32,
    pub direction: char,
    pub head: Point,
}
