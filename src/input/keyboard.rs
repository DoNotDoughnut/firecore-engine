pub use macroquad::prelude::KeyCode as Key;

use crate::Context;

#[allow(unused_variables)]
pub fn pressed(ctx: &Context, key: Key) -> bool {
    macroquad::prelude::is_key_pressed(key)
}

#[allow(unused_variables)]
pub fn down(ctx: &Context, key: Key) -> bool {
    macroquad::prelude::is_key_down(key)
}

#[allow(unused_variables)]
pub fn get_char_queue(ctx: &Context) -> Option<char> {
    macroquad::prelude::get_char_pressed()
}
