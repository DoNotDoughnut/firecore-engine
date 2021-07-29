use tetra::{
    graphics::{Color, DrawParams, Texture},
    math::Vec2,
    Context,
};

use crate::{font::FontId, context::EngineContext};

mod text;
mod texture;

pub use self::text::*;
pub use texture::*;

pub const LIGHTGRAY: Color = Color::rgb(0.78, 0.78, 0.78);
pub const GRAY: Color = Color::rgb(0.51, 0.51, 0.51);
pub const RED: Color = Color::rgb(0.90, 0.16, 0.22);
pub const DARKBLUE: Color = Color::rgb(0.00, 0.32, 0.67);

pub fn byte_texture(ctx: &mut Context, bytes: &[u8]) -> Texture {
    Texture::from_file_data(ctx, bytes).unwrap()
}

pub const ZERO: Vec2<f32> = Vec2::new(0.0, 0.0);

pub const fn position(x: f32, y: f32) -> DrawParams {
    DrawParams {
        position: Vec2::new(x, y),
        scale: Vec2::new(1.0, 1.0),
        origin: ZERO,
        rotation: 0.0,
        color: Color::WHITE,
    }
}

#[inline]
pub fn flip_x(params: DrawParams) -> DrawParams {
    params.scale(Vec2::new(-1.0, 1.0))
}

#[inline]
pub fn flip_y(params: DrawParams) -> DrawParams {
    params.scale(Vec2::new(1.0, -1.0))
}

#[inline]
pub fn draw_bottom(ctx: &mut Context, texture: &Texture, x: f32, y: f32) {
    texture.draw(ctx, position(x, y - texture.height() as f32));
}

#[inline]
pub fn draw_o(ctx: &mut Context, texture: Option<&Texture>, x: f32, y: f32) {
    if let Some(texture) = texture {
        texture.draw(ctx, position(x, y));
    }
}

#[inline]
pub fn draw_o_bottom(ctx: &mut Context, texture: Option<&Texture>, x: f32, y: f32) {
    if let Some(texture) = texture {
        draw_bottom(ctx, texture, x, y);
    }
}

pub fn draw_rectangle(ctx: &mut EngineContext, x: f32, y: f32, w: f32, h: f32, color: Color) {
    tetra::graphics::set_texture(&mut ctx.tetra, &ctx.white);
    tetra::graphics::push_quad(
        &mut ctx.tetra,
        x,
        y,
        x + w,
        y + h,
        0.0,
        0.0,
        1.0,
        1.0,
        &DrawParams::default().color(color),
    )
}

pub fn draw_rectangle_lines(
    ctx: &mut EngineContext,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    thickness: f32,
    color: Color,
) {
    draw_line(ctx, x, y + 1.0, w, true, thickness, color);
    draw_line(ctx, x + 1.0, y, h, false, thickness, color);
    draw_line(ctx, x, y + h, w, true, thickness, color);
    draw_line(ctx, x + w, y, h, false, thickness, color);
}

pub fn draw_line(
    ctx: &mut EngineContext,
    x: f32,
    y: f32,
    len: f32,
    horizontal: bool,
    thickness: f32,
    color: Color,
) {
    let (x, y, w, h) = if horizontal {
        (x, y - thickness / 2.0, len, thickness)
    } else {
        (x - thickness / 2.0, y, thickness, len)
    };
    draw_rectangle(ctx, x, y, w, h, color)
}

#[allow(unused_variables)]
pub fn draw_circle(ctx: &mut EngineContext, x: f32, y: f32, r: f32, color: Color) {
    // todo!("draw circle")
}

use ::text::TextColor;

const TEXT_GRAY: Color = Color::rgb(0.51, 0.51, 0.51);
const TEXT_RED: Color = Color::rgb(0.90, 0.16, 0.22);
const TEXT_WHITE: Color = Color::rgb(240.0 / 255.0, 240.0 / 255.0, 240.0 / 255.0);
const TEXT_BLACK: Color = Color::rgb(20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0);
const TEXT_BLUE: Color = Color::rgb(48.0 / 255.0, 80.0 / 255.0, 200.0 / 255.0);

pub fn text_color(color: TextColor) -> Color {
    match color {
        TextColor::White => TEXT_WHITE,
        TextColor::Gray => TEXT_GRAY,
        TextColor::Black => TEXT_BLACK,
        TextColor::Red => TEXT_RED,
        TextColor::Blue => TEXT_BLUE,
    }
}

pub fn draw_text_left(ctx: &mut EngineContext, font: &FontId, text: &str, color: TextColor, x: f32, y: f32) {
    ctx.text_renderer.draw_text_left(&mut ctx.tetra, font, text, text_color(color), x, y)
}

pub fn draw_text_right(ctx: &mut EngineContext, font: &FontId, text: &str, color: TextColor, x: f32, y: f32) {
    ctx.text_renderer.draw_text_right(&mut ctx.tetra, font, text, text_color(color), x, y)
}

pub fn draw_text_center(
    ctx: &mut EngineContext,
    font: &FontId,
    text: &str,
    color: TextColor,
    x: f32,
    y: f32,
    center_vertical: bool,
) {
    ctx.text_renderer.draw_text_center(&mut ctx.tetra, font, text, text_color(color), x, y, center_vertical)
}

pub fn draw_button(ctx: &mut EngineContext, font: &FontId, text: &str, x: f32, y: f32) {
    ctx.text_renderer.draw_button(&mut ctx.tetra, font, text, x, y)
}

pub fn draw_cursor(ctx: &mut EngineContext, x: f32, y: f32) {
    ctx.text_renderer.draw_cursor(&mut ctx.tetra, x, y)
}

pub fn text_len(ctx: &EngineContext, font: &FontId, text: &str) -> f32 {
    ctx.text_renderer.text_len(font, text)
}

pub fn fade_in_out(
    ctx: &mut Context,
    texture: &Texture,
    x: f32,
    y: f32,
    accumulator: f32,
    end_time: f32,
    fade_time: f32,
) {
    let position = position(x, y);
    if accumulator < fade_time {
        texture.draw(
            ctx,
            position.color(Color::rgba(1.0, 1.0, 1.0, accumulator / fade_time)),
        );
    } else if accumulator < end_time - fade_time {
        texture.draw(ctx, position)
    } else if accumulator < end_time {
        texture.draw(
            ctx,
            position.color(Color::rgba(
                1.0,
                1.0,
                1.0,
                (end_time - accumulator) / fade_time,
            )),
        );
    }
}

pub fn fade_in(
    ctx: &mut Context,
    texture: &Texture,
    x: f32,
    y: f32,
    accumulator: f32,
    fade_time: f32,
) {
    let position = position(x, y);
    texture.draw(
        ctx,
        if accumulator < fade_time {
            position.color(Color::rgba(1.0, 1.0, 1.0, accumulator / fade_time))
        } else {
            position
        },
    );
}