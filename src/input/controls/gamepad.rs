use enum_map::EnumMap;
use std::collections::HashSet;

use crate::{
    input::gamepad::{self, button::GamepadButton},
    Context,
};

use super::Control;

pub type ButtonSet = HashSet<GamepadButton>;
pub type ButtonMap = EnumMap<Control, GamepadButton>;

pub fn pressed(ctx: &Context, control: Control) -> bool {
    gamepad::button::pressed(ctx, 0, ctx.input.controls.controller[control])
}

pub fn down(ctx: &Context, control: Control) -> bool {
    gamepad::button::down(ctx, 0, ctx.input.controls.controller[control])
}

pub fn default_button_map() -> ButtonMap {
    enum_map::enum_map! {
        Control::A => GamepadButton::A,
        Control::B => GamepadButton::B,
        Control::Up => GamepadButton::DpadUp,
        Control::Down => GamepadButton::DpadDown,
        Control::Left => GamepadButton::DpadLeft,
        Control::Right => GamepadButton::DpadRight,
        Control::Start => GamepadButton::Start,
        Control::Select => GamepadButton::Back,
    }
}

pub fn set_button_map(ctx: &mut Context, buttons: ButtonMap) {
    ctx.input.controls.controller = buttons;
}

pub fn get_bind(ctx: &Context, control: Control) -> GamepadButton {
    ctx.input.controls.controller[control]
}

pub fn get_bind_mut(ctx: &mut Context, control: Control) -> &mut GamepadButton {
    &mut ctx.input.controls.controller[control]
}
