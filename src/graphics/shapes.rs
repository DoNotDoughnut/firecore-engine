use super::{Color, Context};

#[allow(unused_variables)]
pub fn clear(ctx: &mut Context, color: Color) {
    macroquad::prelude::clear_background(color);
}

#[allow(unused_variables)]
pub fn draw_rectangle(ctx: &mut Context, x: f32, y: f32, w: f32, h: f32, color: Color) {
    macroquad::prelude::draw_rectangle(x, y, w, h, color)
}

#[allow(unused_variables)]
pub fn draw_rectangle_lines(
    ctx: &mut Context,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    thickness: f32,
    color: Color,
) {
    macroquad::prelude::draw_rectangle_lines(x, y, w, h, thickness, color)
}

/// Deprecated
pub fn draw_straight_line(
    ctx: &mut Context,
    x: f32,
    y: f32,
    len: f32,
    horizontal: bool,
    thickness: f32,
    color: Color,
) {
    match horizontal {
        true => draw_line(ctx, x, x, x + len, y, thickness, color),
        false => draw_line(ctx, x, y, x, y + len, thickness, color),
    }
}

pub fn draw_line(
    _: &mut Context,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    thickness: f32,
    color: Color,
) {
    macroquad::prelude::draw_line(x1, y1, x2, y2, thickness, color)
}

#[allow(unused_variables)]
pub fn draw_circle(_: &mut Context, x: f32, y: f32, r: f32, color: Color) {
    // todo!("draw circle")
    macroquad::prelude::draw_circle(x, y, r, color);
}
