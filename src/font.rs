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

pub type FontSheetImage = FontSheet<Vec<u8>>;
pub type FontSheetFile = FontSheet<String>;

#[derive(Deserialize, Serialize)]
pub struct SerializedFonts {
    pub fonts: Vec<FontSheetImage>,
}

use hashbrown::HashMap;
use tetra::{
    graphics::{ImageData, Rectangle, Texture, Color, DrawParams},
    math::Vec2,
    Result, TetraContext,
};

pub type Fonts = HashMap<FontId, Font>;
type CharTextures = HashMap<char, Texture>;

pub struct Font {
    pub width: SizeInt,
    pub height: SizeInt,
    pub chars: CharTextures,
}

impl Font {
    pub fn draw_text_left(&self, ctx: &mut TetraContext, text: &str, color: Color, x: f32, y: f32) {
        let mut len = 0;
        for character in text.chars() {
            len += if let Some(texture) = self.chars.get(&character) {
                texture.draw(
                    ctx,
                    DrawParams::position(DrawParams::default(), Vec2::new(x + len as f32, y))
                        .color(color),
                );
                texture.width()
            } else {
                self.width as _
            };
        }
    }

    pub fn draw_text_right(&self, ctx: &mut TetraContext, text: &str, color: Color, x: f32, y: f32) {
        let mut len = 0;
        let x = x - self.text_pixel_length(text);
        for character in text.chars() {
            len += if let Some(texture) = self.chars.get(&character) {
                texture.draw(
                    ctx,
                    DrawParams::position(DrawParams::default(), Vec2::new(x + len as f32, y))
                        .color(color),
                );
                texture.width()
            } else {
                self.width as _
            };
        }
    }

    pub fn draw_text_center(&self, ctx: &mut TetraContext, text: &str, color: Color, x: f32, y: f32, center_vertical: bool) {
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
                    texture.draw(
                        ctx,
                        DrawParams::position(
                            DrawParams::default(),
                            Vec2::new(x - x_offset + len, y - y_offset),
                        )
                        .color(color),
                    );
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

pub(crate) fn iterate_fontsheet(
    ctx: &mut TetraContext,
    chars: String,
    font_width: SizeInt,
    font_height: SizeInt,
    custom: Vec<CustomChar>,
    sheet: ImageData,
) -> Result<CharTextures> {
    let mut customchars: HashMap<char, (SizeInt, Option<SizeInt>)> = custom
        .into_iter()
        .map(|cchar| (cchar.id, (cchar.width, cchar.height)))
        .collect();

    let chars: Vec<char> = chars.chars().collect();
    let sheet_width = sheet.width() as _;
    let sheet_height = sheet.height() as _; // - font_height as u32;

    let mut charmap = HashMap::with_capacity(chars.len());

    let mut counter: usize = 0;
    let mut x = 0;
    let mut y = 0;

    'yloop: while y < sheet_height {
        while x < sheet_width {
            charmap.insert(
                chars[counter],
                if let Some(cchar) = customchars.remove(&chars[counter]) {
                    Texture::from_image_data(
                        ctx,
                        &sheet.region(Rectangle::new(
                            x,
                            y,
                            cchar.0 as _,
                            cchar.1.unwrap_or(font_height) as _,
                        )),
                    )
                } else {
                    Texture::from_image_data(
                        ctx,
                        &sheet.region(Rectangle::new(x, y, font_width as _, font_height as _)),
                    )
                }?,
            );
            x += font_width as i32;
            counter += 1;
            if counter >= chars.len() {
                break 'yloop;
            }
        }
        x = 0;
        y += font_height as i32;
    }

    Ok(charmap)
}
