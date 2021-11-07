use crate::{text::FontId, Context};

mod color;
mod image;
pub mod text;
mod texture;

pub mod scaling;

pub use self::color::*;
pub use self::image::*;
pub use self::texture::*;

// pub const GRAY: Color = Color::rgb(0.51, 0.51, 0.51);
// pub const RED: Color = Color::rgb(0.90, 0.16, 0.22);
// pub const DARKBLUE: Color = Color::rgb(0.00, 0.32, 0.67);

// pub const ZERO: Vec2 = Vec2::new(0.0, 0.0);

// #[inline]
// pub fn draw_bottom(ctx: &mut Context, texture: &Texture, x: f32, y: f32) {
//     texture.draw(ctx, position(x, y - texture.height() as f32));
// }

// #[inline]
// pub fn draw_o(ctx: &mut Context, texture: Option<&Texture>, x: f32, y: f32) {
//     if let Some(texture) = texture {
//         texture.draw(ctx, position(x, y));
//     }
// }

// #[inline]
// pub fn draw_o_bottom(ctx: &mut Context, texture: Option<&Texture>, x: f32, y: f32) {
//     if let Some(texture) = texture {
//         draw_bottom(ctx, texture, x, y);
//     }
// }

pub fn clear(_: &mut Context, color: Color) {
    macroquad::prelude::clear_background(color.0);
}

pub fn draw_rectangle(_: &mut Context, x: f32, y: f32, w: f32, h: f32, color: Color) {
    macroquad::prelude::draw_rectangle(x, y, w, h, color.0)
}

pub fn draw_rectangle_lines(
    _: &mut Context,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    thickness: f32,
    color: Color,
) {
    macroquad::prelude::draw_rectangle_lines(x, y, w, h, thickness, color.0)
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
    macroquad::prelude::draw_line(x1, y1, x2, y2, thickness, color.0)
}

#[allow(unused_variables)]
pub fn draw_circle(_: &mut Context, x: f32, y: f32, r: f32, color: Color) {
    // todo!("draw circle")
    macroquad::prelude::draw_circle(x, y, r, color.0);
}

use crate::text::TextColor;

const TEXT_GRAY: Color = Color::rgb(0.51, 0.51, 0.51);
const TEXT_RED: Color = Color::rgb(0.90, 0.16, 0.22);
const TEXT_WHITE: Color = Color::rgb(240.0 / 255.0, 240.0 / 255.0, 240.0 / 255.0);
const TEXT_BLACK: Color = Color::rgb(20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0);
const TEXT_BLUE: Color = Color::rgb(48.0 / 255.0, 80.0 / 255.0, 200.0 / 255.0);

impl From<TextColor> for Color {
    fn from(color: TextColor) -> Self {
        match color {
            TextColor::White => TEXT_WHITE,
            TextColor::Gray => TEXT_GRAY,
            TextColor::Black => TEXT_BLACK,
            TextColor::Red => TEXT_RED,
            TextColor::Blue => TEXT_BLUE,
        }
    }
    
}

pub fn draw_text_left(
    ctx: &mut Context,
    font: &FontId,
    text: &str,
    x: f32,
    y: f32,
    params: DrawParams,
) {
    ctx.text.draw_text_left(font, text, x, y, params)
}

pub fn draw_text_right(
    ctx: &mut Context,
    font: &FontId,
    text: &str,
    x: f32,
    y: f32,
    params: DrawParams,
) {
    ctx.text.draw_text_right(font, text, x, y, params)
}

pub fn draw_text_center(
    ctx: &mut Context,
    font: &FontId,
    text: &str,
    center_vertical: bool,
    x: f32,
    y: f32,
    params: DrawParams,
) {
    ctx.text.draw_text_center(
        font,
        text,
        center_vertical,
        x,
        y,
        params,
    )
}

pub fn draw_button_for_text(ctx: &mut Context, font: &FontId, text: &str, x: f32, y: f32, params: DrawParams) {
    ctx.text
        .draw_button_for_text(font, text, x, y, params)
}

pub fn draw_cursor(ctx: &mut Context, x: f32, y: f32, params: DrawParams) {
    ctx.text.draw_cursor(x, y, params)
}

pub fn text_len(ctx: &Context, font: &FontId, text: &str) -> f32 {
    ctx.text.text_len(font, text)
}

// pub fn fade_in_out(
//     ctx: &mut Context,
//     texture: &Texture,
//     x: f32,
//     y: f32,
//     accumulator: f32,
//     end_time: f32,
//     fade_time: f32,
// ) {
//     let position = position(x, y);
//     if accumulator < fade_time {
//         texture.draw(
//             ctx,
//             position.color(Color::rgba(1.0, 1.0, 1.0, accumulator / fade_time)),
//         );
//     } else if accumulator < end_time - fade_time {
//         texture.draw(ctx, position)
//     } else if accumulator < end_time {
//         texture.draw(
//             ctx,
//             position.color(Color::rgba(
//                 1.0,
//                 1.0,
//                 1.0,
//                 (end_time - accumulator) / fade_time,
//             )),
//         );
//     }
// }

// pub fn fade_in(
//     ctx: &mut Context,
//     texture: &Texture,
//     x: f32,
//     y: f32,
//     accumulator: f32,
//     fade_time: f32,
// ) {
//     let position = position(x, y);
//     texture.draw(
//         ctx,
//         if accumulator < fade_time {
//             position.color(Color::rgba(1.0, 1.0, 1.0, accumulator / fade_time))
//         } else {
//             position
//         },
//     );
// }
