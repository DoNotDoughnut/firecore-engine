pub use gilrs::GamepadId;

use crate::Context;

pub fn connected(ctx: &Context, gamepad: GamepadId) -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        ctx.input.gamepad.connected_gamepad(gamepad).is_some()
    }
    #[cfg(target_arch = "wasm32")]
    {
        false
    }
}

pub mod axis {

    use super::GamepadId;
    use crate::Context;

    pub use gilrs::Axis;

    pub fn direction(ctx: &Context, gamepad: GamepadId, axis: Axis) -> Option<f32> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            ctx.input
                .gamepad
                .connected_gamepad(gamepad)
                .map(|gamepad| gamepad.axis_data(axis).map(|data| data.value()))
                .flatten()
        }

        #[cfg(target_arch = "wasm32")]
        {
            None
        }
    }
}

pub mod button {

    use super::GamepadId;
    use crate::Context;

    pub use gilrs::Button;

    pub fn pressed(ctx: &Context, gamepad: GamepadId, button: Button) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            ctx.input
                .gamepad
                .connected_gamepad(gamepad)
                .map(|gamepad| {
                    gamepad
                        .button_data(button)
                        .map(|button| button.is_pressed())
                })
                .flatten()
                .unwrap_or_default()
        }
        #[cfg(target_arch = "wasm32")]
        {
            false
        }
    }

    pub fn down(ctx: &Context, gamepad: GamepadId, button: Button) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            ctx.input
                .gamepad
                .connected_gamepad(gamepad)
                .map(|gamepad| {
                    gamepad
                        .button_data(button)
                        .map(|button| button.is_repeating() || button.is_pressed())
                })
                .flatten()
                .unwrap_or_default()
        }
        #[cfg(target_arch = "wasm32")]
        {
            false
        }
    }
}
