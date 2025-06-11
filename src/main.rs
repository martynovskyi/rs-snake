use nannou::prelude::*;
    
struct Model {
    pub lines: i32,
    pub columns: i32,
    pub head: Point,
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model { 
    Model {
        lines: 20,
        columns: 20,
        head: Point {x:9, y:11},
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let cell_side = 20.0;
    let draw = _app.draw();
    draw.background().color(PLUM);
    let mut x = 0.0;
    for col in 0.._model.columns {
        let mut y = 0.0;
        for line in 0.._model.lines {
            draw.rect()
                .no_fill()
                .stroke_color(GRAY)
                .stroke_weight(1.0)
                .w(cell_side)
                .h(cell_side)
                .x_y(x,y);
            if _model.head.x == col && _model.head.y == line {
                draw.rect()
                .color(BLACK)
                .w(cell_side - 4.0)
                .h(cell_side - 4.0)
                    .x_y(x + 1.0, y + 1.0);
            } 
            y += cell_side;
        }
        x += cell_side;
    }
    draw.to_frame(_app, &_frame).unwrap();
}
