use crate::{
    graphics::{text::renderer::TextRenderer, ScalingMode, Texture},
    input::InputContext,
    EngineError,
};

pub struct Context {
    pub(crate) running: bool,
    pub(crate) debug: bool,

    pub(crate) input: InputContext,

    #[cfg(feature = "audio")]
    pub(crate) audio: crate::audio::backend::AudioContext,

    #[deprecated]
    pub(crate) text: TextRenderer,

    pub(crate) scaling: ScalingMode,

    pub(crate) panel: Texture,
}

impl Context {
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    // pub fn execute_future<O>(future: impl std::future::Future<Output = O>) {

    // }

    // pub fn get_future_result<T: Any>() -> Option<impl Deref<Target = T>>  {

    // }
}

#[allow(unused_variables)]
pub trait State {
    fn start(&mut self, ctx: &mut Context) {}

    fn update(&mut self, ctx: &mut Context, delta: f32) {}

    fn draw(&mut self, ctx: &mut Context) {}

    fn end(&mut self, ctx: &mut Context) {}
}

impl Context {
    pub(crate) fn new() -> Result<Self, EngineError> {
        Ok(Self {
            text: TextRenderer::new()?,
            input: InputContext::new()?,
            panel: Texture::crate_new(include_bytes!("../assets/panel.png"))?,
            #[cfg(feature = "audio")]
            audio: Default::default(),
            running: true,
            debug: cfg!(debug_assertions),
            scaling: ScalingMode::Fixed,
        })
    }
}

pub struct ContextBuilder<T: Into<String>> {
    pub title: T,
    pub width: i32,
    pub height: i32,
}

impl<T: Into<String>> ContextBuilder<T> {
    pub fn new(title: T, width: i32, height: i32) -> Self {
        Self {
            title,
            width,
            height,
        }
    }
}

impl<T: Into<String>> From<ContextBuilder<T>> for macroquad::prelude::Conf {
    fn from(e: ContextBuilder<T>) -> Self {
        Self {
            window_title: e.title.into(),
            window_width: e.width,
            window_height: e.height,
            ..Default::default()
        }
    }
}

// pub struct DefaultContext(pub Context);

// impl Deref for DefaultContext {
//     type Target = Context;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl DerefMut for DefaultContext {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
