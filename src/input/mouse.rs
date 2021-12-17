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
