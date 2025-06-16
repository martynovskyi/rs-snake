use nannou::prelude::*;

mod snake_struct;
use snake_struct::Point;

struct Model {
    pub lines: i32,
    pub columns: i32,
    pub head: Point,
    pub debug: bool,
}


const CELL_SIZE: f32 = 20.0;

fn main() {
    nannou::app(model)
        .event(event)
        .run();
}

fn model(_app: &App) -> Model { 
    _app.new_window()
        .size(1080, 1080)
        .title("Snake")
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {
        lines: 50,
        columns: 50,
        head: Point {x:20, y:9},
        debug: false,
    }
}

fn event(_app: &App, _model: &mut Model, _e: Event) {
}


fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw = _app.draw();
    let wr = _app.window_rect();
    let snake = &_model.head;
    draw.background().color(PLUM);

    if snake.x < 0 || snake.x >= _model.columns || snake.y < 0 || snake.y >= _model.lines {
        println!("GAME OVER");
    }

    let mut cell = Rect::from_w_h(CELL_SIZE, CELL_SIZE).top_left_of(wr);
    let mut head = cell;
    for line in 0.._model.lines {
        for col in 0.._model.columns {
            draw_cell(&draw, &cell);
            if _model.debug {
             // debug coordiantes
                draw.text(&format!("{},{}", line, col))
                    .color(BLACK)
                    .font_size(8)
                    .xy(cell.xy());
            }

            if snake.x == col && snake.y == line {
                draw_snake_el(&draw, &cell);
            }
        
            if col == _model.columns - 1 {
               cell = head.below(head);
            } else {
               cell = cell.right_of(cell);
            }
        }
        head = cell; 
    }
    draw.to_frame(_app, &_frame).unwrap();
}

fn draw_cell(draw: &Draw, cell: &Rect) {
    draw.rect()
        .no_fill()
        .stroke_color(GRAY)
        .stroke_weight(1.0)
        .wh(cell.wh())
        .xy(cell.xy());
}

fn draw_snake_el(draw: &Draw, cell: &Rect){
    draw.rect()
        .color(BLACK)
        .w(CELL_SIZE - 4.0)
        .h(CELL_SIZE - 4.0)
        .xy(cell.xy());
}


fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::D => {
            model.debug = !model.debug;
        }
        Key::Up => {
            model.head.y -= 1;
        }
        Key::Down => {
            model.head.y += 1;
        }
        Key::Right => {
            model.head.x += 1;
        }
        Key::Left => {
            model.head.x -= 1;
        }
        _other_key => {}
    }
    println!("{:?} -> {:?}", key, model.head); 
}
