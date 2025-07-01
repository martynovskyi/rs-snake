use nannou::prelude::*;

mod snake_struct;
use snake_struct::*;

mod draw;
use draw::draw_cell;
use draw::CELL_SIZE;

struct Model {
    pub lines: u32,
    pub columns: u32,
    pub step_delay: f32,
    pub board: [[BCell; 50]; 50], 
    pub snake: Snake,
    pub debug: bool,
}

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
    const start_x: usize = 24;
    const start_y: usize = 24;
    let mut board = [[BCell::EmptyCell; 50]; 50];
    board[start_x][start_y] = BCell::Head;

    place_food(&mut board);

    Model {
        lines: 50,
        columns: 50,
        step_delay: 0.8,
        board: board,
        snake: Snake {
            ft: _app.time,
            direction: 'u',
            head: Point {
                x: start_x as i32,
                y: start_y as i32,
            },
            tail: Segment {
                next: Option::None,
                coord: Point {
                    x: start_x as i32, 
                    y: start_y as i32,
                }
            },
            size: 1,
        },
        debug: false,
    }
}

fn event(_app: &App, _model: &mut Model, _e: Event) {
    let snake = &mut _model.snake;
    let board = &mut _model.board;
   
    if (_app.time - snake.ft) > _model.step_delay {
        move_snake(snake, board);
        snake.ft = _app.time;
    }
}

fn place_food(board: &mut [[BCell; 50]; 50]){
    board[24][8] = BCell::Food;
}

fn move_snake(snake: &mut Snake, board: &mut [[BCell; 50]; 50]) {
    let tail_val: BCell = board[snake.tail.coord.x as usize][snake.tail.coord.y as usize];

    println!("tail val {:?}", tail_val);

    if tail_val == BCell::FBody{
        board[snake.tail.coord.x as usize][snake.tail.coord.y as usize] = BCell::Body;
        // append segment
    } else {
        println!("clean tail {:?}",snake.tail.coord);
        board[snake.tail.coord.x as usize][snake.tail.coord.y as usize] = BCell::EmptyCell;
    }

    match snake.direction {
            'u' => snake.head.y -= 1, 
            'd' => snake.head.y += 1, 
            'l' => snake.head.x -= 1,
            'r' => snake.head.x += 1, 
            _ => println!("Not possible")
    }
    board[snake.head.x as usize][snake.head.y as usize] = BCell::Head;

    if !snake.tail.next.is_some() {
        snake.tail.coord = snake.head.clone();
    } 
    println!("Snake: {:?}", snake);
} 

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw = _app.draw();
    let wr = _app.window_rect();
    let snake = &_model.snake;
    let board = &_model.board;
    draw.background().color(PLUM);


    if snake.head.x < 0
    || snake.head.x >= _model.columns as i32 
    || snake.head.y < 0
    || snake.head.y >= _model.lines as i32 {
        println!("GAME OVER");
    }

    let mut cell = Rect::from_w_h(CELL_SIZE, CELL_SIZE).top_left_of(wr);
    let mut head = cell;
    for line in 0.._model.lines {
        for col in 0.._model.columns {
            let x = usize::from_u32(col).unwrap();
            let y = usize::from_u32(line).unwrap();
            draw_cell(&draw, &cell, &board[x][y]);
            if _model.debug {
                // debug coordiantes
                draw.text(&format!("{},{}", line, col))
                    .color(BLACK)
                    .font_size(8)
                    .xy(cell.xy());
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


fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::O => {
            model.debug = !model.debug;
        }
        Key::Up | Key::W => {
            if model.snake.direction == 'u' {
                move_snake(&mut model.snake, &mut model.board);
                model.snake.ft = app.time;
            }
            if model.snake.direction != 'd' {
                model.snake.direction = 'u';
            }
        }
        Key::Down | Key::S => { 
            if model.snake.direction == 'd' {
                move_snake(&mut model.snake, &mut model.board);
                model.snake.ft = app.time;
            }
            if model.snake.direction != 'u' {
                model.snake.direction = 'd';
            }
        }
        Key::Right | Key::D => {
            if model.snake.direction == 'r' {
                move_snake(&mut model.snake, &mut model.board);
                model.snake.ft = app.time;
            }
            if model.snake.direction != 'l' {
                model.snake.direction = 'r';
            }
        }
        Key::Left | Key::A => {
            if model.snake.direction == 'l' {
                move_snake(&mut model.snake, &mut model.board);
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
