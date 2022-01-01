pub use fiirengine::*;

pub mod utils;
pub mod graphics;
pub mod controls;
pub mod text;
pub mod audio;
pub mod gui;

pub use context::*;

mod context {
    use fiirengine::{graphics::Texture, EngineError, UserContext, Context};

    use crate::{controls::ControlsContext, graphics::renderer::TextRenderer};

    pub struct EngineContext {
        pub(crate) controls: ControlsContext,
        pub(crate) text: TextRenderer,
        pub(crate) panel: Texture,
        #[cfg(feature = "audio")]
        pub(crate) audio: crate::audio::backend::AudioContext,
    }
    
    impl UserContext for EngineContext {
        fn new(ctx: &mut Context) -> Result<Self, EngineError> {
            Ok(Self {
                text: TextRenderer::new(ctx)?,
                controls: ControlsContext::default(),
                panel: Texture::new(ctx, include_bytes!("../assets/panel.png"))?,
                #[cfg(feature = "audio")]
                audio: Default::default(),
            })
        }
    }

}