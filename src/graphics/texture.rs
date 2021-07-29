use hashbrown::HashMap;
use std::{hash::Hash, fmt::Display};
use tetra::graphics::Texture;

pub struct TextureManager<ID: Eq + Hash + Display>(HashMap<ID, Texture>);

impl<ID: Eq + Hash + Display> TextureManager<ID> {

    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    pub fn new(map: HashMap<ID, Texture>) -> Self  {
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