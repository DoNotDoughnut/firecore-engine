use crate::{graphics::ScalingMode, input::InputContext, EngineError};
pub struct Context {
    pub(crate) running: bool,
    pub(crate) debug: bool,
    pub(crate) input: InputContext,
    pub(crate) scaling: Scaling,
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

impl Context {
    pub(crate) fn new() -> Result<Self, EngineError> {
        Ok(Self {
            running: true,
            debug: cfg!(debug_assertions),
            input: InputContext::new()?,
            scaling: Default::default(),
        })
    }
}

pub trait UserContext: Sized {
    fn new(ctx: &mut Context) -> Result<Self, EngineError>;
}

pub struct ContextBuilder<T: Into<String>> {
    pub title: T,
    pub width: i32,
    pub height: i32,
}

#[derive(Default)]
pub(crate) struct Scaling(ScalingMode, Option<f32>);

impl UserContext for () {
    #[allow(unused_variables)]
    fn new(ctx: &mut Context) -> Result<Self, EngineError> {
        Ok(())
    }
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

impl From<(ScalingMode, Option<f32>)> for Scaling {
    fn from(s: (ScalingMode, Option<f32>)) -> Self {
        Self(s.0, s.1)
    }
}
