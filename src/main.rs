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
    pub board: [[Option<GS>; 50]; 50], 
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
    let mut board = [[Option::<GS>::None; 50]; 50];
    board[start_x][start_y] = Option::Some(GS::head('u'));

    place_food(&mut board);

    Model {
        lines: 50,
        columns: 50,
        step_delay: 0.8,
        board: board,
        snake: Snake {
            ft: _app.time,
            direction: 'u',
            head_x: start_x as i32,
            head_y: start_y as i32,
            tail_x: start_x as i32,
            tail_y: start_y as i32,
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

fn place_food(board: &mut [[Option<GS>; 50]; 50]){
    board[24][8] = Option::Some(GS::food('0'));
    board[24][18] = Option::Some(GS::food('0'));
}

fn move_snake(snake: &mut Snake, board: &mut [[Option<GS>; 50]; 50]) {
    // handle current head location
    let mut head_x = snake.head_x as usize;
    let mut head_y = snake.head_y as usize;
    
    let head = board[head_x][head_y].unwrap();
    match head.cell {
        CellType::FHead => {
            println!("snake growing");
            board[head_x][head_y] = Option::Some(GS::fbody(head.direction));
        },
        _ => {println!("Current head: {:?}", board[head_x][head_y]); }
    }

    // move head
    match snake.direction {
            'u' => snake.head_y -= 1, 
            'd' => snake.head_y += 1, 
            'l' => snake.head_x -= 1,
            'r' => snake.head_x += 1, 
            _ => println!("Not possible")
    }

    println!("old head {}:{}-{:?}-{}", head_x, head_y, head.cell, head.direction);
    // handle new head location
    head_x = snake.head_x as usize;
    head_y = snake.head_y as usize;

    println!("new head {}:{}", head_x, head_y);
    match board[head_x][head_y] {
        None => board[head_x][head_y] = Option::Some(GS::head(snake.direction)),
        Some(gs) => {
            match gs.cell {
                CellType::Food => {
                    board[head_x][head_y] = Option::Some(GS::fhead(snake.direction));
                    snake.size +=1;
                },
                _ => {}
            }
        }
    }

    // handle tail move
    let tail_x = snake.tail_x as usize;
    let tail_y = snake.tail_y as usize;


    if snake.size == 1 {
        board[tail_x][tail_y] = Option::None;
        snake.tail_x = snake.head_x;
        snake.tail_y = snake.head_y;
    } else if let Some(tail) = board[tail_x][tail_y] {
        let mut new_tail_x = tail_x;
        let mut new_tail_y = tail_y;

        // move tail
        match tail.direction {
            'u' => new_tail_y -= 1, 
            'd' => new_tail_y += 1, 
            'l' => new_tail_x -= 1,
            'r' => new_tail_x += 1, 
            _ => println!("Not possible")
        }
        let new_tail_cell = board[new_tail_x][new_tail_y].unwrap();

        println!("old tail {}:{}-{:?}-{}", tail_x, tail_y, tail.cell, tail.direction);
        println!("new tail {}:{}-{:?}-{}", new_tail_x, new_tail_y, new_tail_cell.cell, new_tail_cell.direction);
        match tail.cell {
            CellType::FBody => board[tail_x][tail_y] = Option::Some(GS::tail(new_tail_cell.direction)),
            CellType::FHead => {},
            _ => {
                println!("clean tail");
                board[tail_x][tail_y] = Option::None;
            }
        }
        board[new_tail_x][new_tail_y] = Option::Some(GS::tail(new_tail_cell.direction));
        snake.tail_x = new_tail_x as i32;
        snake.tail_y = new_tail_y as i32;

    } else {
       // snake.tail_x = snake.head_x;
       // snake.tail_y = snake.head_y;
    }
    println!("Snake: {:?}", snake);
} 

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw = _app.draw();
    let wr = _app.window_rect();
    let snake = &_model.snake;
    let board = &_model.board;
    draw.background().color(PLUM);


    if snake.head_x < 0
    || snake.head_x >= _model.columns as i32 
    || snake.head_y < 0
    || snake.head_y >= _model.lines as i32 {
        println!("GAME OVER");
    }

    let mut cell = Rect::from_w_h(CELL_SIZE, CELL_SIZE).top_left_of(wr);
    let mut head = cell;
    for line in 0.._model.lines {
        for col in 0.._model.columns {
            let x = usize::from_u32(col).unwrap();
            let y = usize::from_u32(line).unwrap();

            match board[x][y] {
                None => draw_cell(&draw, &cell, CellType::EmptyCell),
                Some(gs) => draw_cell(&draw, &cell, gs.cell),
            }
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
