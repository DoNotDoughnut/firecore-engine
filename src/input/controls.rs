use enum_map::Enum;
use serde::{Deserialize, Serialize};
use crate::Context;

pub mod controller;
pub mod keyboard;
// pub mod touchscreen;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize, Enum)]
pub enum Control {
    A,
    B,
    Up,
    Down,
    Left,
    Right,
    Start,
    Select,
}

pub struct GameControls {
    pub keyboard: keyboard::KeyMap,
    pub controller: controller::ButtonMap,
}

impl Default for GameControls {
    fn default() -> Self {
        GameControls {
            keyboard: keyboard::default_key_map(),
            controller: controller::default_button_map(),
        }
    }
}

pub fn pressed(ctx: &Context, control: Control) -> bool {
    if keyboard::pressed(ctx, control) {
        return true;
    }
    if controller::pressed(ctx, control) {
        return true;
    }
    // if let Some(controls) = unsafe{touchscreen::TOUCHSCREEN.as_ref()} {
    //     if controls.pressed(&control) {
    //         return true;
    //     }
    // }
    false
}

pub fn down(ctx: &Context, control: Control) -> bool {
    if keyboard::down(ctx, control) {
        return true;
    }
    if controller::down(ctx, control) {
        return true;
    }
    // if let Some(controls) = unsafe{touchscreen::TOUCHSCREEN.as_ref()} {
    //     if controls.down(&control) {
    //         return true;
    //     }
    // }
    false
}