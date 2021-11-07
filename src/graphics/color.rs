type Inner = macroquad::prelude::Color;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Color(pub(crate) Inner);

impl Color {

    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self(Inner { r, g, b, a  })
    }

}