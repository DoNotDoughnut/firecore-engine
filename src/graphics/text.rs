use std::ops::Deref;

use crate::{
    text::{Font, FontId, FontSheet, Fonts},
    graphics::{Image, Texture},
};

use image::ImageError;

use super::DrawParams;

pub(crate) struct TextRenderer {
    pub fonts: Fonts,
    button: Texture,
    cursor: Texture,
}

impl TextRenderer {
    pub fn new() -> Result<Self, ImageError> {
        Ok(Self {
            fonts: Default::default(),
            button: Texture::crate_new(include_bytes!("../../assets/button.png"))?,
            cursor: Texture::crate_new(include_bytes!("../../assets/cursor.png"))?,
        })
    }

    pub fn add_font_sheet(
        &mut self,
        font_sheet: &FontSheet<impl Deref<Target = [u8]>>,
    ) -> Result<(), ImageError> {
        self.fonts.insert(
            font_sheet.data.id,
            Font {
                width: font_sheet.data.width,
                height: font_sheet.data.height,
                chars: crate::text::iterate_fontsheet(
                    &font_sheet.data.chars,
                    font_sheet.data.width,
                    font_sheet.data.height,
                    &font_sheet.data.custom,
                    Image::new(&font_sheet.sheet)?,
                )?,
            },
        );
        Ok(())
    }

    pub fn draw_text_left(&self, font: &FontId, text: &str, x: f32, y: f32, params: DrawParams) {
        if let Some(font) = self.fonts.get(font) {
            font.draw_text_left(text, x, y, params);
        }
    }

    pub fn draw_text_right(&self, font: &FontId, text: &str, x: f32, y: f32, params: DrawParams) {
        if let Some(font) = self.fonts.get(font) {
            font.draw_text_right(text, x, y, params);
        }
    }

    pub fn draw_text_center(
        &self,
        font: &FontId,
        text: &str,
        center_vertical: bool,
        x: f32,
        y: f32,
        params: DrawParams,
    ) {
        if let Some(font) = self.fonts.get(font) {
            font.draw_text_center(text, center_vertical, x, y, params);
        }
    }

    pub fn draw_button_for_text(
        &self,
        font: &FontId,
        text: &str,
        x: f32,
        y: f32,
        params: DrawParams,
    ) {
        if let Some(font) = self.fonts.get(font) {
            self.draw_button(x + font.text_pixel_length(text) as f32, y + 2.0, params)
        }
    }

    pub fn draw_button(&self, x: f32, y: f32, params: DrawParams) {
        self.button.crate_draw(x, y, params);
    }

    pub fn draw_cursor(&self, x: f32, y: f32, params: DrawParams) {
        self.cursor.crate_draw(x, y, params);
    }

    pub fn text_len(&self, font: &FontId, text: &str) -> f32 {
        if let Some(font) = self.fonts.get(font) {
            font.text_pixel_length(text)
        } else {
            0.0
        }
    }
}
