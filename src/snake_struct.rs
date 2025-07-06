
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

impl GS {
    pub fn head(direction: char) -> Self {
        GS {
            cell: CellType::Head,
            direction,
        }
    }
    
    pub fn fhead(direction: char) -> Self {
        GS {
            cell: CellType::FHead,
            direction,
        }
    }
    
    pub fn body(direction: char) -> Self {
        GS {
            cell: CellType::Body,
            direction,
        }
    }
    
    pub fn fbody(direction: char) -> Self {
        GS {
            cell: CellType::FBody,
            direction,
        }
    }

    pub fn tail(direction: char) -> Self {
        GS {
            cell: CellType::Tail,
            direction,
        }
    }

    pub fn food(direction: char) -> Self {
        GS {
            cell: CellType::Food,
            direction,
        }
    }
}
