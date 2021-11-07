use std::ops::{Deref, DerefMut};

#[cfg(feature = "audio")]
pub mod audio;

use crate::{graphics::{Texture, scaling::ScreenScaler, text::TextRenderer}, input::controls::GameControls};


/// To - do: make Context::new not require fonts and have that be initialized later.
pub struct Context {
    pub(crate) text: TextRenderer,
    pub(crate) controls: GameControls,
    #[cfg(feature = "audio")]
    pub(crate) audio: audio::GameAudio,

    pub(crate) panel: Texture,
    pub(crate) running: bool,
    pub(crate) scaler: Option<ScreenScaler>
}

impl Context {
    pub fn new() -> Result<Self, image::ImageError> {
        Ok(Self {
            text: TextRenderer::new()?,
            controls: Default::default(),
            panel: Texture::crate_new(include_bytes!("../assets/panel.png"))?,
            #[cfg(feature = "audio")]
            audio: Default::default(),
            running: true,
            scaler: None,
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

pub struct DefaultContext(pub Context);

impl Deref for DefaultContext {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DefaultContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}