use crate::text::font::FontId;

use super::{Context, DrawParams};

pub(crate) mod font;
pub(crate) mod renderer;

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
    ctx.text
        .draw_text_center(font, text, center_vertical, x, y, params)
}

pub fn draw_button_for_text(
    ctx: &mut Context,
    font: &FontId,
    text: &str,
    x: f32,
    y: f32,
    params: DrawParams,
) {
    ctx.text.draw_button_for_text(font, text, x, y, params)
}

pub fn draw_cursor(ctx: &mut Context, x: f32, y: f32, params: DrawParams) {
    ctx.text.draw_cursor(x, y, params)
}

pub fn text_len(ctx: &Context, font: &FontId, text: &str) -> f32 {
    ctx.text.text_len(font, text)
}
