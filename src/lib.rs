pub mod audio;
pub mod error;
pub mod fs;
pub mod graphics;
pub mod gui;
pub mod input;
pub mod math;
pub mod text;
pub mod utils;

mod context;

use std::future::Future;

pub use self::{context::*, error::EngineError};

pub mod log {
    pub use macroquad::miniquad::{debug, error, info, log::Level, trace, warn};
}

#[deprecated]
pub extern crate macroquad;

pub fn run<
    OPEN,
    OPENFUNC: Future<Output = OPEN> + 'static,
    LOAD,
    LOADFUNC: FnOnce(&mut Context, OPEN) -> LOAD + 'static,
    S: State,
    SFUNC: FnOnce(&mut Context, LOAD) -> S + 'static,
>(
    args: ContextBuilder<impl Into<String>>,
    open: OPENFUNC,
    load: LOADFUNC,
    state: SFUNC,
) {
    macroquad::Window::from_config(args.into(), async move {
        macroquad::prelude::prevent_quit();

        macroquad::prelude::clear_background(graphics::Color::BLACK);
        macroquad::prelude::draw_text("Loading...", 5.0, 5.0, 20.0, graphics::Color::WHITE);

        let open = open.await;

        let mut ctx = Context::new()
            .unwrap_or_else(|err| panic!("Could not initialize Context with error {}", err));

        let data = (load)(&mut ctx, open);

        let mut state = (state)(&mut ctx, data);

        state.start(&mut ctx);

        loop {
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
