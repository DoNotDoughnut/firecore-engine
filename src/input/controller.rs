use crate::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GamepadButton {
    A,
    B,
    X,
    Y,
    Up,
    Down,
    Left,
    Right,
    LeftShoulder,
    LeftTrigger,
    LeftStick,
    RightShoulder,
    RightTrigger,
    RightStick,
    Start,
    Back,
    Guide,
}

pub fn is_gamepad_button_pressed(ctx: &Context, gamepad_id: usize, button: GamepadButton) -> bool {
    // if let Some(pad) = get_gamepad(ctx, gamepad_id) {
    //     pad.buttons_pressed.contains(&button)
    // } else {
    //     false
    // }

    false
}

pub fn is_gamepad_button_down(ctx: &Context, gamepad_id: usize, button: GamepadButton) -> bool {
    // if let Some(pad) = get_gamepad(ctx, gamepad_id) {
    //     pad.buttons_down.contains(&button)
    // } else {
    //     false
    // }
    false
}
