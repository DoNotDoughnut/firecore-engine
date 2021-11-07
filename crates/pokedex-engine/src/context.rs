use engine::{error::ImageError, graphics::Texture, Context};

use crate::{
    serialize::SerializedPokedexEngine,
    texture::{ItemTextures, PokemonTextures, TrainerTextures},
};

pub struct PokedexClientData {
    pub health_bar: Texture,
    pub bag_background: Texture,
    pub party: PokedexPartyData,
    pub pokemon_textures: PokemonTextures,
    pub item_textures: ItemTextures,
    pub trainer_textures: TrainerTextures,
}

pub struct PokedexPartyData {
    pub background: Texture,
    pub ball: Texture,
    pub select: Texture,
    pub summary: PokedexSummaryData,
}

pub struct PokedexSummaryData {
    pub pages: [Texture; 3],
    pub background: Texture,
}

impl PokedexClientData {
    pub async fn new(
        ctx: &mut Context,
        engine: SerializedPokedexEngine,
    ) -> Result<Self, ImageError> {
        let mut pokemon_textures = PokemonTextures::with_capacity(engine.pokemon.len());

        for (id, pokemon) in engine.pokemon {
            if let Err(err) = pokemon_textures.insert(ctx, id, &pokemon) {
                log::warn!("Cannot add pokemon texture for {} with error {}", id, err)
            }

            #[cfg(feature = "audio")]
            if !pokemon.cry.is_empty() {
                engine::audio::add_sound(ctx, crate::CRY_ID, Some(id), pokemon.cry).await;
            }
        }

        let mut item_textures = ItemTextures::with_capacity(engine.items.len());

        for (id, texture) in engine.items.into_iter() {
            item_textures.insert(id, Texture::new(ctx, &texture)?);
        }

        let mut trainer_textures = TrainerTextures::with_capacity(engine.trainers.len());

        for (id, texture) in engine.trainers {
            trainer_textures.insert(id, Texture::new(ctx, &texture)?);
        }

        Ok(Self {
            health_bar: Texture::new(ctx, include_bytes!("../assets/health.png"))?,
            bag_background: Texture::new(ctx, include_bytes!("../assets/bag/items.png"))?,
            party: PokedexPartyData {
                background: Texture::new(ctx, include_bytes!("../assets/party/background.png"))?,
                ball: Texture::new(ctx, include_bytes!("../assets/party/ball.png"))?,
                select: Texture::new(ctx, include_bytes!("../assets/party/select.png"))?,
                summary: PokedexSummaryData {
                    background: Texture::new(
                        ctx,
                        include_bytes!("../assets/party/summary/pokemon.png"),
                    )?,
                    pages: [
                        Texture::new(ctx, include_bytes!("../assets/party/summary/info.png"))?,
                        Texture::new(ctx, include_bytes!("../assets/party/summary/skills.png"))?,
                        Texture::new(ctx, include_bytes!("../assets/party/summary/moves.png"))?,
                    ],
                },
            },
            pokemon_textures,
            item_textures,
            trainer_textures,
        })
    }
}
