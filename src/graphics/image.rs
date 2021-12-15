use image::GenericImageView;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Image(pub(crate) image::RgbaImage);

impl Image {
    pub fn new(data: &[u8]) -> Result<Self, image::ImageError> {
        Ok(Self(image::load_from_memory(data)?.to_rgba8()))
    }

    pub fn width(&self) -> u32 {
        self.0.width()
    }

    pub fn height(&self) -> u32 {
        self.0.height()
    }

    pub fn region(&self, x: u32, y: u32, w: u32, h: u32) -> SubImage {
        SubImage(self.0.view(x, y, w, h))
    }
}

pub struct SubImage<'i>(
    image::SubImage<&'i <image::RgbaImage as image::GenericImageView>::InnerImageView>,
);

impl SubImage<'_> {
    pub fn width(&self) -> u32 {
        self.0.width()
    }

    pub fn height(&self) -> u32 {
        self.0.height()
    }
}

impl From<SubImage<'_>> for Image {
    fn from(image: SubImage<'_>) -> Self {
        Image(image.0.to_image())
    }
}
