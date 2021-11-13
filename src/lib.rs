pub mod audio;
pub mod context;
pub mod error;
pub mod graphics;
pub mod gui;
pub mod input;
pub mod math;
pub mod state;
pub mod util;

pub mod text;

// pub use macroquad::main;

use std::ops::DerefMut;

pub extern crate async_trait;

pub use self::{
    context::{Context, ContextBuilder, DefaultContext},
    error::EngineError,
    state::State,
};

// pub fn build(
//     builder: &mut tetra::ContextBuilder,
//     fonts: font::SerializedFonts,
// ) -> tetra::Result<Context> {
//     Context::new(builder
//         .timestep(tetra::time::Timestep::Variable)
//         .build()?,
//         fonts
//     )
// }

pub fn quit(ctx: &mut Context) {
    ctx.running = false;
}

pub fn run<
    C: DerefMut<Target = Context>,
    RUNSTATE: State<C> + 'static,
    F: std::future::Future<Output = C> + 'static,
    CF: FnOnce(Context) -> F + 'static,
    SF: FnOnce(&mut C) -> RUNSTATE + 'static,
>(
    args: ContextBuilder<impl Into<String>>,
    ctx: CF,
    state: SF,
) {
    macroquad::Window::from_config(args.into(), async move {
        macroquad::prelude::prevent_quit();

        // log::trace!("to - do: gamepad support");

        let context = Context::new().unwrap_or_else(|err| panic!("Could not initialize Context with error {}", err));

        let mut ctx = (ctx)(context).await;

        let mut state = (state)(&mut ctx);

        state.start(&mut ctx).await;

        loop {

            if let Some(scaler) = ctx.scaler.as_mut() {
                scaler.update();
            }

            state.update(&mut ctx, macroquad::prelude::get_frame_time());

            state.draw(&mut ctx);

            if macroquad::prelude::is_quit_requested() || !ctx.running {
                state.end(&mut ctx);
                break;
            }

            macroquad::prelude::next_frame().await;
        }
    });
}
