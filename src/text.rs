use std::ops::Deref;

use image::ImageError;

pub use firecore_font_builder::*;
pub type FontDimensions = u8;

use std::collections::HashMap;

use crate::{
    graphics::{DrawParams, Image, Texture},
    Context,
};

pub type Fonts = HashMap<FontId, Font>;
type CharTextures = HashMap<char, Texture>;

pub struct Font {
    pub width: FontDimensions,
    pub height: FontDimensions,
    pub chars: CharTextures,
}

impl Font {
    pub fn draw_text_left(&self, text: &str, x: f32, y: f32, params: DrawParams) {
        let mut len = 0.0;
        for character in text.chars() {
            len += if let Some(texture) = self.chars.get(&character) {
                texture.crate_draw(x + len, y, params);
                texture.width()
            } else {
                self.width as _
            };
        }
    }

    pub fn draw_text_right(&self, text: &str, x: f32, y: f32, params: DrawParams) {
        let mut len = 0.0;
        let x = x - self.text_pixel_length(text);
        for character in text.chars() {
            len += if let Some(texture) = self.chars.get(&character) {
                texture.crate_draw(x + len, y, params);
                texture.width()
            } else {
                self.width as _
            };
        }
    }

    pub fn draw_text_center(
        &self,
        text: &str,
        center_vertical: bool,
        x: f32,
        y: f32,
        params: DrawParams,
    ) {
        let mut len = 0.0;

        let x_offset = (text
            .chars()
            .map(|ref character| match self.chars.get(character) {
                Some(texture) => texture.width() as FontDimensions,
                None => self.width,
            })
            .sum::<FontDimensions>()
            >> 1) as f32;

        let y_offset = if center_vertical {
            (self.height >> 1) as f32
        } else {
            0.0
        };

        for character in text.chars() {
            len += match self.chars.get(&character) {
                Some(texture) => {
                    texture.crate_draw(x - x_offset + len, y - y_offset, params);
                    texture.width() as f32
                }
                None => self.width as f32,
            };
        }
    }

    pub fn text_pixel_length(&self, text: &str) -> f32 {
        text.chars()
            .map(|character| match self.chars.get(&character) {
                Some(texture) => texture.width() as f32,
                None => self.width as f32,
            })
            .sum()
    }
}

pub fn insert_font(
    ctx: &mut Context,
    font_sheet: &FontSheet<impl Deref<Target = [u8]>>,
) -> Result<(), ImageError> {
    ctx.text.add_font_sheet(font_sheet)
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
