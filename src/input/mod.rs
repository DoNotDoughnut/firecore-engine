use quad_gamepad::ControllerContext;

use crate::EngineError;

pub mod gamepad;
pub mod keyboard;
pub mod mouse;

pub mod controls;

pub(crate) struct InputContext {
    gamepad: ControllerContext,
    controls: controls::ControlsContext,
}

impl InputContext {

    pub fn new() -> Result<Self, EngineError> {
        Ok(Self {
            gamepad: ControllerContext::new().ok_or(EngineError::GamepadContext)?,
            controls: Default::default(),
        })
    }

}

// pub type DebugBind = tetra::input::Key;

// pub fn debug_pressed(ctx: &tetra::Context, bind: DebugBind) -> bool {
//     tetra::input::is_key_pressed(ctx, bind)
// }

// pub fn debug_down(ctx: &tetra::Context, bind: DebugBind) -> bool {
//     tetra::input::is_key_down(ctx, bind)
// }
