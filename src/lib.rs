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

pub use context::EngineContext;

pub fn build(
    builder: &mut tetra::ContextBuilder,
    fonts: font::SerializedFonts,
) -> tetra::Result<EngineContext> {
    EngineContext::new(builder
        .timestep(tetra::time::Timestep::Variable)
        .build()?,
        fonts
    )
}
