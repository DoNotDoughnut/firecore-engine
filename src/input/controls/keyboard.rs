use enum_map::EnumMap;

use crate::{input::keyboard::{self, Key}, Context};

use super::Control;

// pub type KeySet = HashSet<Key>;
pub type KeyMap = EnumMap<Control, Key>;

pub fn pressed(ctx: &Context, control: Control) -> bool {
    let key = ctx.controls.keyboard[control];
    keyboard::is_key_pressed(ctx, key)
}

pub fn down(ctx: &Context, control: Control) -> bool {
    let key = ctx.controls.keyboard[control];
    keyboard::is_key_down(ctx, key)
    // .iter()
    // .any(|key| input::is_key_down(ctx, *key))
}

pub fn default_key_map() -> KeyMap {
    enum_map::enum_map! {
        Control::A => Key::X,
        Control::B => Key::Z,
        Control::Up => Key::Up,
        Control::Down => Key::Down,
        Control::Left => Key::Left,
        Control::Right => Key::Right,
        Control::Start => Key::A,
        Control::Select => Key::S,
    }
}

pub fn set_key_map(ctx: &mut Context, keys: KeyMap) {
    ctx.controls.keyboard = keys;
}

// fn keyset(codes: &[Key]) -> KeySet {
//     let mut set = HashSet::with_capacity(codes.len());
//     for code in codes {
//         set.insert(*code);
//     }
//     set
// }
