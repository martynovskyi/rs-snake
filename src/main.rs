use nannou::prelude::*;

mod snake_struct;
use snake_struct::Snake;
use snake_struct::Point;

struct Model {
    pub lines: i32,
    pub columns: i32,
    pub snake: Snake,
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
        .size(1000, 1080)
        .title("Snake")
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {
        lines: 50,
        columns: 50,
        snake: Snake {
            ft: _app.time,
            direction: 'u',
            head: Point {
                x: 24,
                y: 24,
            },
        },
        debug: false,
    }
}

fn event(_app: &App, _model: &mut Model, _e: Event) {
   // println!("Event {:?}", _e);
    
    let snake = &mut _model.snake;
    if (_app.time - snake.ft) > 0.8 {
    snake.ft = _app.time;
    match snake.direction {
            'u' => { snake.head.y -= 1; 
            }
            'd' => { snake.head.y += 1; 
            }
            'l' => { snake.head.x -= 1; 
            }
            'r' => { snake.head.x += 1; 
            }
            _ => println!("Not possible")
    }


    println!("Move {:?}", snake);
    }
}


fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw = _app.draw();
    let wr = _app.window_rect();
    let snake = &_model.snake;
    draw.background().color(PLUM);

    if snake.head.x < 0
    || snake.head.x >= _model.columns 
    || snake.head.y < 0
    || snake.head.y >= _model.lines {
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

            if snake.head.x == col && snake.head.y == line {
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
            if model.snake.direction == 'u' {
                model.snake.head.y -= 1;
                model.snake.ft = app.time;
            }
            if model.snake.direction != 'd' {
                model.snake.direction = 'u';
            }
        }
        Key::Down => { 
            if model.snake.direction == 'd' {
                model.snake.head.y += 1;
                model.snake.ft = app.time;
            }
            if model.snake.direction != 'u' {
                model.snake.direction = 'd';
            }
        }
        Key::Right => {
            if model.snake.direction == 'r' {
                model.snake.head.x += 1;
                model.snake.ft = app.time;
            }
            if model.snake.direction != 'l' {
                model.snake.direction = 'r';
            }
        }
        Key::Left => {
            if model.snake.direction == 'l' {
                model.snake.head.x -= 1;
                model.snake.ft = app.time;
            }
            if model.snake.direction != 'r' {
                model.snake.direction = 'l';
            }
        }

        _other_key => {}
    }
    println!("{}: {:?} -> {:?}", app.time, key, model.snake); 
}
