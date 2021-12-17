use serde::{Deserialize, Serialize};

use crate::{error::ImageError, graphics::Color, Context};

pub extern crate firecore_font_builder as font;

pub use font::FontId;

pub fn insert_font(
    ctx: &mut Context,
    font_sheet: &font::FontSheet<Vec<u8>>,
) -> Result<(), ImageError> {
    ctx.text.add_font_sheet(font_sheet)
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub pages: Vec<MessagePage>,

    #[serde(default = "Message::default_color")]
    pub color: Color,
}

impl Message {
    pub const BLACK: Color = Color::rgb(20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0);
    pub const WHITE: Color = Color::rgb(240.0 / 255.0, 240.0 / 255.0, 240.0 / 255.0);

    fn default_color() -> Color {
        Color::GRAY
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct MessagePage {
    pub lines: Vec<String>,
    pub wait: Option<f32>,
}

impl From<Vec<String>> for MessagePage {
    fn from(lines: Vec<String>) -> Self {
        Self { lines, wait: None }
    }
}
