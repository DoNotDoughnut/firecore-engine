pub use image::ImageError;

#[derive(Debug)]
pub enum EngineError {
    Image(image::ImageError),
    File(FileError),
    Gamepad(gilrs::Error),
}

#[derive(Debug)]
pub enum FileError {
    Engine(macroquad::prelude::FileError),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for EngineError {}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::Image(err) => std::fmt::Display::fmt(err, f),
            EngineError::File(err) => std::fmt::Display::fmt(err, f),
            EngineError::Gamepad(err) => std::fmt::Display::fmt(err, f),
        }
    }
}

impl std::error::Error for FileError {}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::Engine(err) => std::fmt::Display::fmt(err, f),
            FileError::String(err) => std::fmt::Display::fmt(err, f),
        }
    }
}

impl From<macroquad::prelude::FileError> for FileError {
    fn from(err: macroquad::prelude::FileError) -> Self {
        Self::Engine(err)
    }
}

impl From<image::ImageError> for EngineError {
    fn from(e: image::ImageError) -> Self {
        Self::Image(e)
    }
}
