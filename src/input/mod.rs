pub mod controller;
pub mod keyboard;

pub mod controls;

pub use controls::{down, pressed, Control};

// pub type DebugBind = tetra::input::Key;

// pub fn debug_pressed(ctx: &tetra::Context, bind: DebugBind) -> bool {
//     tetra::input::is_key_pressed(ctx, bind)
// }

// pub fn debug_down(ctx: &tetra::Context, bind: DebugBind) -> bool {
//     tetra::input::is_key_down(ctx, bind)
// }
