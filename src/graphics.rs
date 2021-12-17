pub(self) use crate::Context;

mod image;
mod shapes;
pub(crate) mod text;
mod texture;
mod window;

pub use self::image::*;
pub use self::shapes::*;
pub use self::text::*;
pub use self::texture::*;
pub use self::window::*;

pub type Color = macroquad::prelude::Color;

// pub fn fade_in_out(
//     ctx: &mut Context,
//     texture: &Texture,
//     x: f32,
//     y: f32,
//     accumulator: f32,
//     end_time: f32,
//     fade_time: f32,
// ) {
//     let position = position(x, y);
//     if accumulator < fade_time {
//         texture.draw(
//             ctx,
//             position.color(Color::rgba(1.0, 1.0, 1.0, accumulator / fade_time)),
//         );
//     } else if accumulator < end_time - fade_time {
//         texture.draw(ctx, position)
//     } else if accumulator < end_time {
//         texture.draw(
//             ctx,
//             position.color(Color::rgba(
//                 1.0,
//                 1.0,
//                 1.0,
//                 (end_time - accumulator) / fade_time,
//             )),
//         );
//     }
// }

// pub fn fade_in(
//     ctx: &mut Context,
//     texture: &Texture,
//     x: f32,
//     y: f32,
//     accumulator: f32,
//     fade_time: f32,
// ) {
//     let position = position(x, y);
//     texture.draw(
//         ctx,
//         if accumulator < fade_time {
//             position.color(Color::rgba(1.0, 1.0, 1.0, accumulator / fade_time))
//         } else {
//             position
//         },
//     );
// }
