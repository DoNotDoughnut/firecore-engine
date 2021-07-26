pub extern crate firecore_text as text;
pub extern crate tetra;

pub mod audio;
pub mod context;
pub mod font;
pub mod graphics;
pub mod gui;
pub mod input;
pub mod util;

pub const WIDTH: f32 = 240.0;
pub const HEIGHT: f32 = 160.0;

pub type Context = tetra::Context<context::GameContext>;
pub use context::GameContext;

pub fn build(
    builder: &mut tetra::ContextBuilder,
    fonts: font::SerializedFonts,
) -> tetra::Result<Context> {
    builder
        .timestep(tetra::time::Timestep::Variable)
        .build(|ctx| context::GameContext::new(ctx, fonts))
}
