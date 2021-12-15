pub use image::ImageError;

#[derive(Debug)]
pub enum EngineError {
    Image(image::ImageError),
}

impl From<image::ImageError> for EngineError {
    fn from(e: image::ImageError) -> Self {
        Self::Image(e)
    }
}
