use crate::font::{Font, FontId, Fonts, SerializedFonts};

use hashbrown::HashMap;
use tetra::{
    graphics::{DrawParams, ImageData, Texture, Color},
    math::Vec2,
    Result, Context,
};

pub struct TextRenderer {
    pub fonts: Fonts,
    pub button: Texture,
    pub cursor: Texture,
}

impl TextRenderer {
    pub fn new(ctx: &mut Context, serialized_fonts: SerializedFonts) -> Result<Self> {
        let mut fonts = HashMap::with_capacity(serialized_fonts.fonts.len());
        for font_sheet in serialized_fonts.fonts {
            fonts.insert(
                font_sheet.data.id,
                Font {
                    width: font_sheet.data.width,
                    height: font_sheet.data.height,
                    chars: crate::font::iterate_fontsheet(
                        ctx,
                        font_sheet.data.chars,
                        font_sheet.data.width,
                        font_sheet.data.height,
                        font_sheet.data.custom,
                        ImageData::from_file_data(&font_sheet.sheet)?,
                    )?,
                },
            );
        }

        Ok(Self {
            fonts,
            button: Texture::from_file_data(ctx, include_bytes!("../../assets/button.png"))?,
            cursor: Texture::from_file_data(ctx, include_bytes!("../../assets/cursor.png"))?,
        })
    }

    pub fn draw_text_left(
        &self,
        ctx: &mut Context,
        font: &FontId,
        text: &str,
        color: Color,
        x: f32,
        y: f32,
    ) {
        if let Some(font) = self.fonts.get(font) {
            font.draw_text_left(ctx, text, color, x, y);
        }
    }

    pub fn draw_text_right(
        &self,
        ctx: &mut Context,
        font: &FontId,
        text: &str,
        color: Color,
        x: f32,
        y: f32,
    ) {
        if let Some(font) = self.fonts.get(font) {
            font.draw_text_right(ctx, text, color, x, y);
        }
    }

    pub fn draw_text_center(
        &self,
        ctx: &mut Context,
        font: &FontId,
        text: &str,
        color: Color,
        x: f32,
        y: f32,
        center_vertical: bool,
    ) {
        if let Some(font) = self.fonts.get(font) {
            font.draw_text_center(ctx, text, color, x, y, center_vertical);
        }
    }

    pub fn draw_button(&self, ctx: &mut Context, font: &FontId, text: &str, x: f32, y: f32) {
        if let Some(font) = self.fonts.get(font) {
            self.button.draw(
                ctx,
                DrawParams::position(
                    DrawParams::default(),
                    Vec2::new(x + font.text_pixel_length(text) as f32, y + 2.0),
                ),
            );
        }
    }

    pub fn draw_cursor(&self, ctx: &mut Context, x: f32, y: f32) {
        self.cursor.draw(
            ctx,
            DrawParams::position(DrawParams::default(), Vec2::new(x, y)),
        );
    }

    pub fn text_len(&self, font: &FontId, text: &str) -> f32 {
        if let Some(font) = self.fonts.get(font) {
            font.text_pixel_length(text)
        } else {
            0.0
        }
    }
}
