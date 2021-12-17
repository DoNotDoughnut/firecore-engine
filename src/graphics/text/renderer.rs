use crate::{
    graphics::{Image, Texture},
    text::font::{FontId, FontSheet},
    utils::HashMap,
};

use firecore_font_builder::CustomChar;
use image::ImageError;

use super::{
    font::{CharTextures, Font, FontDimensions, Fonts},
    DrawParams,
};

pub(crate) struct TextRenderer {
    fonts: Fonts,
    button: Texture,
    cursor: Texture,
}
impl TextRenderer {
    pub fn new() -> Result<Self, ImageError> {
        Ok(Self {
            fonts: Default::default(),
            button: Texture::crate_new(include_bytes!("../../../assets/button.png"))?,
            cursor: Texture::crate_new(include_bytes!("../../../assets/cursor.png"))?,
        })
    }

    pub fn add_font_sheet(&mut self, font_sheet: &FontSheet<Vec<u8>>) -> Result<(), ImageError> {
        self.fonts.insert(
            font_sheet.data.id,
            Font {
                width: font_sheet.data.width,
                height: font_sheet.data.height,
                chars: iterate_fontsheet(
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

pub(crate) fn iterate_fontsheet(
    chars: &str,
    font_width: FontDimensions,
    font_height: FontDimensions,
    custom: &[CustomChar],
    sheet: Image,
) -> Result<CharTextures, ImageError> {
    let mut customchars: HashMap<char, (FontDimensions, Option<FontDimensions>)> = custom
        .into_iter()
        .map(|cchar| (cchar.id, (cchar.width, cchar.height)))
        .collect();

    let chars: Vec<char> = chars.chars().collect();
    let sheet_width = sheet.width();
    let sheet_height = sheet.height(); // - font_height as u32;

    let mut charmap = HashMap::with_capacity(chars.len());

    let mut counter: usize = 0;
    let mut x = 0;
    let mut y = 0;

    'yloop: while y < sheet_height {
        while x < sheet_width {
            charmap.insert(
                chars[counter],
                if let Some(cchar) = customchars.remove(&chars[counter]) {
                    Texture::crate_from_image(
                        &Image::from(sheet.region(
                            x,
                            y,
                            cchar.0 as _,
                            cchar.1.unwrap_or(font_height) as _,
                        ))
                        .0,
                    )
                } else {
                    Texture::crate_from_image(
                        &Image::from(sheet.region(x, y, font_width as _, font_height as _)).0,
                    )
                },
            );
            x += font_width as u32;
            counter += 1;
            if counter >= chars.len() {
                break 'yloop;
            }
        }
        x = 0;
        y += font_height as u32;
    }

    Ok(charmap)
}
