pub mod audio;
pub mod error;
pub mod fs;
pub mod graphics;
pub mod input;
pub mod math;

mod context;

use std::future::Future;

pub use self::{context::*, error::EngineError};

pub mod log {
    pub use macroquad::miniquad::{debug, error, info, log::Level, trace, warn};
}

pub mod utils {

    pub use macroquad::prelude::{HashMap, HashSet};

    pub fn seed() -> u64 {
        (time() * 10000000.0) as u64
    }

    pub fn time() -> f64 {
        macroquad::miniquad::date::now()
    }
}

#[deprecated]
pub extern crate macroquad;

// .

#[allow(unused_variables)]
pub trait State<U: UserContext = ()> {
    fn start(&mut self, ctx: &mut Context, userctx: &mut U) {}

    fn update(&mut self, ctx: &mut Context, userctx: &mut U, delta: f32) {}

    fn draw(&mut self, ctx: &mut Context, userctx: &mut U) {}

    fn end(&mut self, ctx: &mut Context, userctx: &mut U) {}
}

pub fn run<
    U: UserContext,
    OPEN,
    OPENFUNC: Future<Output = OPEN> + 'static,
    LOAD,
    LOADFUNC: FnOnce(&mut Context, &mut U, OPEN) -> LOAD + 'static,
    S: State<U>,
    SFUNC: FnOnce(&mut Context, &mut U, LOAD) -> S + 'static,
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

        let mut userctx = U::new(&mut ctx)
            .unwrap_or_else(|err| panic!("Cannot initialize user context with error {}", err));

        let data = (load)(&mut ctx, &mut userctx, open);

        let mut state = (state)(&mut ctx, &mut userctx, data);

        state.start(&mut ctx, &mut userctx);

        loop {

            ctx.input.update();

            state.update(&mut ctx, &mut userctx, macroquad::prelude::get_frame_time());

            state.draw(&mut ctx, &mut userctx);

            if macroquad::prelude::is_quit_requested() || !ctx.running {
                state.end(&mut ctx, &mut userctx);
                break;
            }

            macroquad::prelude::next_frame().await;
        }
    });
}
