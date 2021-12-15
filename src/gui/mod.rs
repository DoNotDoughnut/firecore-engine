mod bar;
mod panel;
mod text;

pub use self::{bar::*, panel::*, text::*};

pub struct TextColor;

use crate::graphics::Color;

impl TextColor {
    pub const GRAY: Color = Color::rgb(0.51, 0.51, 0.51);
    pub const RED: Color = Color::rgb(0.90, 0.16, 0.22);
    pub const WHITE: Color = Color::rgb(240.0 / 255.0, 240.0 / 255.0, 240.0 / 255.0);
    pub const BLACK: Color = Color::rgb(20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0);
    pub const BLUE: Color = Color::rgb(48.0 / 255.0, 80.0 / 255.0, 200.0 / 255.0);
}

// pub struct StaticList<D, const SIZE: usize> {
//     pub options: [D; SIZE],
//     pub cursor: usize,
// }

// pub struct MultiStaticList<D: Array> {
//     pub options:
//     pub cursor: usize,
// }
