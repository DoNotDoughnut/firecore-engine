use hashbrown::HashMap;
use macroquad::prelude::{DrawTextureParams, FilterMode, Texture2D};
use std::{fmt::Display, hash::Hash, rc::Rc};

use crate::{
    context::Context,
    math::{Rectangle, Vec2},
};

use super::{Color, Image};

#[derive(Debug, Clone, PartialEq)]
pub struct Texture(Rc<Texture2D>);

impl Texture {
    pub(crate) fn crate_new(data: &[u8]) -> Result<Self, image::ImageError> {
        let image = image::load_from_memory(data)?.to_rgba8();
        Ok(Self::crate_from_image(&image))
    }

    pub(crate) fn crate_from_image(image: &image::RgbaImage) -> Self {
        let tex = Texture2D::from_rgba8(
            image.width() as _,
            image.height() as _,
            image.as_raw(),
        );
        tex.set_filter(FilterMode::Nearest);
        Self(Rc::new(tex))
    }

    pub fn new(_: &mut Context, data: &[u8]) -> Result<Self, image::ImageError> {
        Self::crate_new(data)
    }

    pub fn from_image(_: &mut Context, image: &Image) -> Self {
        Self::crate_from_image(&image.0)
    }

    pub fn draw(&self, _: &mut Context, x: f32, y: f32, params: DrawParams) {
        self.crate_draw(x, y, params)
    }

    pub(crate) fn crate_draw(&self, x: f32, y: f32, params: DrawParams) {
        let (color, params) = params.init();
        macroquad::prelude::draw_texture_ex(*self.0, x, y, color, params);
    }

    pub fn width(&self) -> f32 {
        self.0.width()
    }

    pub fn height(&self) -> f32 {
        self.0.height()
    }

    pub fn set_filter(&self, filter: FilterMode) {
        self.0.set_filter(filter)
    }

    // pub fn try_draw(self: Option<&Self>, ctx: &mut Context, x: f32, y: f32, params: DrawParams) {}
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.0.delete()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DrawParams {
    pub color: Color,

    /// Part of texture to draw. If None - draw the whole texture.
    /// Good use example: drawing an image from texture atlas.
    /// Is None by default
    pub source: Option<Rectangle>,

    pub dest_size: Option<Vec2>,

    /// Rotation in radians
    pub rotation: f32,

    /// Mirror on the X axis
    pub flip_x: bool,

    /// Mirror on the Y axis
    pub flip_y: bool,

    pub origin: Option<Vec2>,

}

impl DrawParams {

    pub fn color(color: Color) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }

    pub fn source(source: Rectangle) -> Self {
        Self {
            source: Some(source),
            ..Default::default()
        }
    }

    pub(crate) fn init(self) -> (macroquad::prelude::Color, DrawTextureParams) {
        (
            self.color.0,
            DrawTextureParams {
                dest_size: self.dest_size,
                source: self.source,
                rotation: self.rotation,
                flip_x: self.flip_x,
                flip_y: self.flip_y,
                pivot: self.origin,
            },
        )
    }
}

impl Default for DrawParams {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            dest_size: None,
            source: None,
            rotation: 0.,
            flip_x: false,
            flip_y: false,
            origin: None,
        }
    }
}

pub struct TextureManager<ID: Eq + Hash + Display>(HashMap<ID, Texture>);

impl<ID: Eq + Hash + Display> TextureManager<ID> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    pub fn new(map: HashMap<ID, Texture>) -> Self {
        Self(map)
    }

    pub fn insert(&mut self, id: ID, texture: Texture) {
        self.0.insert(id, texture);
    }

    pub fn try_get(&self, id: &ID) -> Option<&Texture> {
        self.0.get(id)
    }

    pub fn get(&self, id: &ID) -> &Texture {
        self.try_get(id).unwrap_or_else(|| {
            panic!(
                "Could not get texture from texture manager \"{}\" with id {}",
                crate::util::type_name::<Self>(),
                id
            )
        })
    }
}
