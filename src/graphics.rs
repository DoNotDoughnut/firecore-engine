pub(self) use crate::Context;

mod image;
mod shapes;
mod texture;
mod window;

pub use self::image::*;
pub use self::shapes::*;
pub use self::texture::*;
pub use self::window::*;

pub type Color = macroquad::prelude::Color;
