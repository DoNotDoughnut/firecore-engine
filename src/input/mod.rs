use crate::EngineError;

pub mod gamepad;
pub mod keyboard;
pub mod mouse;


pub(crate) struct InputContext {
    #[cfg(not(target_arch = "wasm32"))]
    gamepad: quad_gamepad::ControllerContext,
}

impl InputContext {
    pub fn new() -> Result<Self, EngineError> {
        Ok(Self {
            #[cfg(not(target_arch = "wasm32"))]
            gamepad: quad_gamepad::ControllerContext::new().ok_or(EngineError::GamepadContext)?,
        })
    }
}

// pub type DebugBind = tetra::input::Key;

// pub fn debug_pressed(ctx: &tetra::Context, bind: DebugBind) -> bool {
//     tetra::input::is_key_pressed(ctx, eng, bind)
// }

// pub fn debug_down(ctx: &tetra::Context, bind: DebugBind) -> bool {
//     tetra::input::is_key_down(ctx, bind)
// }
