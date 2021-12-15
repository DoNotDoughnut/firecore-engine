pub mod audio;
pub mod error;
pub mod graphics;
pub mod gui;
pub mod input;
pub mod math;
pub mod util;

pub mod text;

mod context;

// pub use macroquad::main;

pub use self::{context::*, error::EngineError};

pub mod log {
    pub use macroquad::miniquad::{debug, error, info, log::Level, trace, warn};
}

pub extern crate macroquad as inner;

pub fn quit(ctx: &mut Context) {
    ctx.running = false;
}

pub fn debug(ctx: &mut Context, debug: bool) {
    ctx.debug = debug;
}

pub fn run<
    LOAD,
    LOADFUNC: FnOnce(&mut Context) -> LOAD + 'static,
    S: State,
    SFUNC: FnOnce(&mut Context, LOAD) -> S + 'static,
>(
    args: ContextBuilder<impl Into<String>>,
    load: LOADFUNC,
    state: SFUNC,
) {
    macroquad::Window::from_config(args.into(), async move {
        macroquad::prelude::prevent_quit();

        // todo!("game pad support");

        let mut ctx = Context::new()
            .unwrap_or_else(|err| panic!("Could not initialize Context with error {}", err));

        macroquad::prelude::clear_background(graphics::Color::BLACK);
        macroquad::prelude::draw_text("Loading...", 5.0, 5.0, 20.0, graphics::Color::WHITE);

        let data = (load)(&mut ctx);

        let mut state = (state)(&mut ctx, data);

        state.start(&mut ctx);

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
