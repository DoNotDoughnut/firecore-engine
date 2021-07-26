use enum_map::EnumMap;
use tetra::input::{self, Key};

use crate::{input::Control, Context};

// pub type KeySet = HashSet<Key>;
pub type KeyMap = EnumMap<Control, Key>;

pub fn pressed(ctx: &Context, control: Control) -> bool {
    input::is_key_pressed(ctx, ctx.game.controls.keyboard[control])
}

pub fn down(ctx: &Context, control: Control) -> bool {
    input::is_key_down(ctx, ctx.game.controls.keyboard[control])
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

// fn keyset(codes: &[Key]) -> KeySet {
//     let mut set = HashSet::with_capacity(codes.len());
//     for code in codes {
//         set.insert(*code);
//     }
//     set
// }
