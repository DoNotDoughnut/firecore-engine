pub use macroquad::prelude::KeyCode as Key;

pub use macroquad::prelude::get_char_pressed;

use crate::Context;

pub fn is_key_pressed(_: &Context, key: Key) -> bool {
    macroquad::prelude::is_key_pressed(key)
}

pub fn is_key_down(_: &Context, key: Key) -> bool {
    macroquad::prelude::is_key_down(key)
}