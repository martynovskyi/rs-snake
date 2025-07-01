use nannou::prelude::*;
use crate::snake_struct::BCell;

pub const CELL_SIZE: f32 = 20.0;

pub fn draw_cell(draw: &Draw, cell: &Rect, item: &BCell) {
    draw.rect()
        .no_fill()
        .stroke_color(GRAY)
        .stroke_weight(1.0)
        .wh(cell.wh())
        .xy(cell.xy());

    match item {
        BCell::Head => draw_snake_el(draw, cell),
        BCell::Food => draw_food(draw, cell),
        _ => {}
    }
}

fn draw_food(draw: &Draw, cell: &Rect) {
    draw.rect()
        .color(RED)
        .h(CELL_SIZE - 10.0)
        .w(CELL_SIZE - 10.0)
        .rotate(0.78)
        .xy(cell.xy());
}

fn draw_snake_el(draw: &Draw, cell: &Rect) {
    draw.rect()
        .color(BLACK)
        .w(CELL_SIZE - 4.0)
        .h(CELL_SIZE - 4.0)
        .xy(cell.xy());
}
