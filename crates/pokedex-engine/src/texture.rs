use engine::{
    graphics::{Texture, TextureManager}, Context, error::ImageError,
};
use hashbrown::HashMap;

use pokedex::{item::ItemId, pokemon::PokemonId};

use crate::{TrainerId, serialize::SerializedPokemon};

pub type TrainerTextures = TextureManager<TrainerId>;
pub type ItemTextures = TextureManager<ItemId>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PokemonTexture {
    Front,
    Back,
    Icon,
}

pub struct PokemonTextures {
    pub front: HashMap<PokemonId, Texture>,
    pub back: HashMap<PokemonId, Texture>,
    pub icon: HashMap<PokemonId, Texture>,
}

impl PokemonTextures {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            front: HashMap::with_capacity(capacity),
            back: HashMap::with_capacity(capacity),
            icon: HashMap::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, ctx: &mut Context, id: PokemonId, pokemon: &SerializedPokemon) -> Result<(), ImageError> {
        self.front.insert(
            id,
            Texture::new(ctx, &pokemon.front)?,
        );
        self.back.insert(
            id,
            Texture::new(ctx, &pokemon.back)?,
        );
        self.icon.insert(
            id,
            Texture::new(ctx, &pokemon.icon)?,
        );
        Ok(())
    }

    pub fn get(&self, id: &PokemonId, side: PokemonTexture) -> &Texture {
        match side {
            PokemonTexture::Front => self.front.get(id),
            PokemonTexture::Back => self.back.get(id),
            PokemonTexture::Icon => self.icon.get(id),
        }
        .unwrap_or_else(|| panic!("Could not get texture for pokemon with ID {}", id))
    }
}
