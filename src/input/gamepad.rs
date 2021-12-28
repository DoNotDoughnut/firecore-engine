pub mod button {

    use crate::Context;

    pub use quad_gamepad::GamepadButton;

    pub fn pressed(ctx: &Context, gamepad: usize, button: GamepadButton) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let state = ctx.input.gamepad.state(gamepad);
            state.digital_state[button as usize] && !state.digital_state_prev[button as usize]
        }
        #[cfg(target_arch = "wasm32")]
        {
            false
        }
    }

    pub fn down(ctx: &Context, gamepad: usize, button: GamepadButton) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            ctx.input.gamepad.state(gamepad).digital_state[button as usize]
        }
        #[cfg(target_arch = "wasm32")]
        {
            false
        }
    }
}
