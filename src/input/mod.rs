use crate::EngineError;

pub mod keyboard;
pub mod mouse;

#[cfg(all(not(target_arch = "wasm32"), feature = "gamepad"))]
pub mod gamepad;


pub(crate) struct InputContext {
    #[cfg(all(not(target_arch = "wasm32"), feature = "gamepad"))]
    gamepad: gilrs::Gilrs,
}

impl InputContext {
    pub fn new() -> Result<Self, EngineError> {
        Ok(Self {
            #[cfg(all(not(target_arch = "wasm32"), feature = "gamepad"))]
            gamepad: gilrs::GilrsBuilder::new().set_update_state(false).build().map_err(EngineError::Gamepad)?,
        })
    }

    pub fn update(&mut self) {
        #[cfg(all(not(target_arch = "wasm32"), feature = "gamepad"))]
        while let Some(ev) = self.gamepad.next_event() {
            self.gamepad.update(&ev)
        }
    }

}

// pub type DebugBind = tetra::input::Key;

// pub fn debug_pressed(ctx: &tetra::Context, bind: DebugBind) -> bool {
//     tetra::input::is_key_pressed(ctx, eng, bind)
// }

// pub fn debug_down(ctx: &tetra::Context, bind: DebugBind) -> bool {
//     tetra::input::is_key_down(ctx, bind)
// }
