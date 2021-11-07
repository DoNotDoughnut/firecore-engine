pub use macroquad::prelude::Rect as Rectangle;

pub use macroquad::prelude::{DVec2, IVec2, Vec2};

use macroquad::prelude::{const_dvec2, const_ivec2, const_vec2};

pub const fn dvec2(x: f64, y: f64) -> DVec2 {
    const_dvec2!([x, y])
}

pub const fn ivec2(x: i32, y: i32) -> IVec2 {
    const_ivec2!([x, y])
}

pub const fn vec2(x: f32, y: f32) -> Vec2 {
    const_vec2!([x, y])
}

// #[repr(transparent)]
// pub struct Rectangle(pub(crate) Rect);

// impl Rectangle {
//     pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
//         Self(Rect { x, y, w, h })
//     }

// }
