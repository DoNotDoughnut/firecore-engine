pub use macroquad::prelude::KeyCode as Key;

use crate::Context;

pub fn is_key_pressed(_: &Context, key: Key) -> bool {
    macroquad::prelude::is_key_pressed(key)
}

pub fn is_key_down(_: &Context, key: Key) -> bool {
    macroquad::prelude::is_key_down(key)
}

pub fn get_text_input(_: &Context) -> &str {
    macroquad::prelude::get_text_input()
}
