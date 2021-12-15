use enum_map::EnumMap;
use std::collections::HashSet;

use crate::{
    input::controller::{self as input, GamepadButton},
    Context,
};

use super::Control;

pub type ButtonSet = HashSet<GamepadButton>;
pub type ButtonMap = EnumMap<Control, GamepadButton>;

pub fn pressed(ctx: &Context, control: Control) -> bool {
    input::is_gamepad_button_pressed(ctx, 0, ctx.controls.controller[control])
}

pub fn down(ctx: &Context, control: Control) -> bool {
    input::is_gamepad_button_down(ctx, 0, ctx.controls.controller[control])
}

pub fn default_button_map() -> ButtonMap {
    enum_map::enum_map! {
        Control::A => GamepadButton::A,
        Control::B => GamepadButton::B,
        Control::Up => GamepadButton::Up,
        Control::Down => GamepadButton::Down,
        Control::Left => GamepadButton::Left,
        Control::Right => GamepadButton::Right,
        Control::Start => GamepadButton::Start,
        Control::Select => GamepadButton::Back,
    }
}

pub fn set_button_map(ctx: &mut Context, buttons: ButtonMap) {
    ctx.controls.controller = buttons;
}

pub fn get_bind(ctx: &Context, control: Control) -> GamepadButton {
    ctx.controls.controller[control]
}

pub fn get_bind_mut(ctx: &mut Context, control: Control) -> &mut GamepadButton {
    &mut ctx.controls.controller[control]
}
