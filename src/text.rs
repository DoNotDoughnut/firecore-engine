pub use firecore_text::*;

use std::ops::Deref;

use image::ImageError;
use serde::{Deserialize, Serialize};

pub type FontId = u8;
pub(crate) type SizeInt = u8;

#[derive(Debug, Deserialize, Serialize)]
pub struct FontSheetData {
    pub id: FontId,
    pub width: SizeInt,
    pub height: SizeInt,
    pub chars: String,
    pub custom: Vec<CustomChar>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomChar {
    pub id: char,
    pub width: SizeInt,
    pub height: Option<SizeInt>,
}

#[derive(Deserialize, Serialize)]
pub struct FontSheet<S> {
    pub sheet: S,
    pub data: FontSheetData,
}

use hashbrown::HashMap;

use crate::{Context, graphics::{DrawParams, Image, Texture}};

pub type Fonts = HashMap<FontId, Font>;
type CharTextures = HashMap<char, Texture>;

pub struct Font {
    pub width: SizeInt,
    pub height: SizeInt,
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
                Some(texture) => texture.width() as SizeInt,
                None => self.width,
            })
            .sum::<SizeInt>()
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

pub fn insert_font(ctx: &mut Context, font_sheet: &FontSheet<impl Deref<Target = [u8]>>) -> Result<(), ImageError> {
    ctx.text.add_font_sheet(font_sheet)
}


pub(crate) fn iterate_fontsheet(
    chars: &str,
    font_width: SizeInt,
    font_height: SizeInt,
    custom: &[CustomChar],
    sheet: Image,
) -> Result<CharTextures, ImageError> {
    let mut customchars: HashMap<char, (SizeInt, Option<SizeInt>)> = custom
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
