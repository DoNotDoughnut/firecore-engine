pub mod button {

    use crate::Context;

    pub use quad_gamepad::GamepadButton;

    pub fn pressed(ctx: &Context, gamepad: usize, button: GamepadButton) -> bool {
        let state = ctx.input.gamepad.state(gamepad);
        state.digital_state[button as usize] && !state.digital_state_prev[button as usize]
    }

    pub fn down(ctx: &Context, gamepad: usize, button: GamepadButton) -> bool {
        ctx.input.gamepad.state(gamepad).digital_state[button as usize]
    }
}
