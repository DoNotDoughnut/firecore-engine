use pokedex::{
    context::PokedexClientContext,
    engine::{
        graphics::position,
        tetra::{
            graphics::{Color, Rectangle, Texture},
            math::Vec2,
            Context,
        },
        util::Reset,
    },
    pokemon::PokemonId,
    texture::PokemonTexture,
};

use crate::{
    context::BattleGuiContext,
    ui::{BattleGuiPosition, BattleGuiPositionIndex},
};

use self::{
    faint::Faint,
    flicker::Flicker,
    spawner::{Spawner, SpawnerState},
};

// mod moves;
mod status;

// pub use moves::*;
pub use status::*;
pub mod bounce;

pub mod faint;
pub mod flicker;
pub mod spawner;

#[derive(Clone)]
pub struct PokemonRenderer {
    // pub moves: MoveRenderer,
    pub pokemon: Option<Texture>,
    side: PokemonTexture,

    pub pos: Vec2<f32>,

    pub spawner: Spawner,
    pub faint: Faint,
    pub flicker: Flicker,
}

impl PokemonRenderer {
    pub fn new(
        ctx: &BattleGuiContext,
        index: BattleGuiPositionIndex,
        side: PokemonTexture,
    ) -> Self {
        Self {
            // moves: MoveRenderer::new(index.position),
            pokemon: None,
            side,
            pos: Self::position(index),
            spawner: Spawner::new(ctx, None),
            faint: Faint::default(),
            flicker: Flicker::default(),
        }
    }

    pub fn with(
        ctx: &BattleGuiContext,
        dex: &PokedexClientContext,
        index: BattleGuiPositionIndex,
        pokemon: Option<PokemonId>,
        side: PokemonTexture,
    ) -> Self {
        Self {
            pokemon: pokemon.map(|pokemon| dex.pokemon_textures.get(&pokemon, side).clone()),
            spawner: Spawner::new(ctx, pokemon),
            ..Self::new(ctx, index, side)
        }
    }

    fn position(index: BattleGuiPositionIndex) -> Vec2<f32> {
        let offset = (index.size - 1) as f32 * 32.0 - index.index as f32 * 64.0;
        match index.position {
            BattleGuiPosition::Top => Vec2::new(144.0 - offset, 74.0),
            BattleGuiPosition::Bottom => Vec2::new(40.0 - offset, 113.0),
        }
    }

    pub fn new_pokemon(&mut self, dex: &PokedexClientContext, pokemon: Option<PokemonId>) {
        self.spawner.id = pokemon;
        self.pokemon = pokemon.map(|pokemon| dex.pokemon_textures.get(&pokemon, self.side).clone());
        self.reset();
    }

    pub fn spawn(&mut self) {
        self.spawner.spawning = SpawnerState::Start;
        self.spawner.x = 0.0;
    }

    pub fn faint(&mut self) {
        if let Some(texture) = self.pokemon.as_ref() {
            self.faint.fainting = true;
            self.faint.remaining = texture.height() as f32;
        }
    }

    pub fn flicker(&mut self) {
        self.flicker.remaining = Flicker::TIMES;
        self.flicker.accumulator = 0.0;
    }

    pub fn draw(&self, ctx: &mut Context, offset: Vec2<f32>, color: Color) {
        if let Some(texture) = &self.pokemon {
            let pos = self.pos + offset;
            if self.spawner.spawning() {
                self.spawner.draw(ctx, pos, texture);
            } else if self.flicker.accumulator < Flicker::HALF {
                if self.faint.fainting {
                    if self.faint.remaining > 0.0 {
                        texture.draw_region(
                            ctx,
                            Rectangle::new(0.0, 0.0, texture.width() as f32, self.faint.remaining),
                            position(pos.x, pos.y - self.faint.remaining).color(color),
                        );
                    }
                } else {
                    texture.draw(
                        ctx,
                        position(
                            pos.x, //+ self.moves.pokemon_x(),
                            pos.y - texture.height() as f32,
                        )
                        .color(color),
                    );
                }
            }
        }
    }
}

impl Default for PokemonRenderer {
    fn default() -> Self {
        Self {
            pokemon: Default::default(),
            side: PokemonTexture::Front,
            pos: Default::default(),
            spawner: Spawner::default(),
            faint: Default::default(),
            flicker: Default::default(),
        }
    }
}

impl Reset for PokemonRenderer {
    fn reset(&mut self) {
        self.faint = Faint::default();
        self.flicker = Flicker::default();
        self.spawner.spawning = SpawnerState::None;
    }
}
