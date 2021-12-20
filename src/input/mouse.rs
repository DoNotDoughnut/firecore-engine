pub use macroquad::input::MouseButton;

use crate::Context;

#[allow(unused_variables)]
pub fn pressed(ctx: &Context, button: MouseButton) -> bool {
    macroquad::prelude::is_mouse_button_pressed(button)
}

#[allow(unused_variables)]
pub fn down(ctx: &Context, button: MouseButton) -> bool {
    macroquad::prelude::is_mouse_button_down(button)
}

/// Probably does not return scaled coordinates
#[allow(unused_variables)]
pub fn position(ctx: &Context) -> crate::math::Vec2 {
    macroquad::prelude::mouse_position().into()
}
