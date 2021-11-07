use core::ops::Deref;
use pokedex::{item::Item, moves::Move, pokemon::Pokemon};

use pokedex::{
    context::PokedexClientData,
    engine::{graphics::Color, math::vec2, Context},
    pokemon::owned::OwnedPokemon,
    texture::PokemonTexture,
    Identifiable, TrainerId,
};

use battle::{party::PlayerParty, pokemon::remote::UnknownPokemon};

use crate::{
    context::BattleGuiContext,
    ui::{
        pokemon::{flicker::Flicker, PokemonRenderer, PokemonStatusGui},
        BattleGuiPosition, BattleGuiPositionIndex,
    },
};

pub type InitLocalPlayer<ID, P, M, I> = PlayerParty<ID, usize, OwnedPokemon<P, M, I>>;
pub type InitRemotePlayer<ID, P> = PlayerParty<ID, usize, Option<UnknownPokemon<P>>>;

pub type GuiLocalPlayer<ID, P, M, I> = ActivePlayer<ID, OwnedPokemon<P, M, I>>;
pub type GuiRemotePlayer<ID, P> = ActivePlayer<ID, Option<UnknownPokemon<P>>>;

pub struct ActivePlayer<ID, P> {
    pub player: PlayerParty<ID, usize, P>,
    pub renderer: Vec<ActivePokemonRenderer>,
    pub trainer: Option<TrainerId>,
}

impl<ID, P> ActivePlayer<ID, P> {
    pub fn new(player: PlayerParty<ID, usize, P>) -> Self {
        Self {
            player,
            renderer: Vec::new(),
            trainer: None,
        }
    }
}

#[derive(Default, Clone)]
pub struct ActivePokemonRenderer {
    pub pokemon: PokemonRenderer,
    /// to - do: make non-optional
    pub status: PokemonStatusGui,
}

impl ActivePokemonRenderer {
    pub fn draw(&self, ctx: &mut Context) {
        self.pokemon.draw(ctx, vec2(0.0, 0.0), Color::WHITE);
        self.status.draw(
            ctx,
            0.0,
            if self.pokemon.flicker.accumulator % Flicker::HALF > Flicker::HALF / 8.0
                && self.pokemon.flicker.remaining > (Flicker::TIMES >> 1)
            {
                0.0
            } else {
                1.0
            },
        );
        // self.renderer.moves.draw(ctx);
    }
}

impl<ID, P: Deref<Target = Pokemon>, M: Deref<Target = Move>, I: Deref<Target = Item>>
    ActivePlayer<ID, OwnedPokemon<P, M, I>>
{
    pub fn init(&mut self, ctx: &BattleGuiContext, data: &PokedexClientData) {
        let size = self.player.active.len() as u8;

        for (i, index) in self.player.active.iter().enumerate() {
            let position = BattleGuiPositionIndex::new(BattleGuiPosition::Bottom, i as u8, size);
            let pokemon = (*index).map(|index| &self.player.pokemon[index]);
            let r = ActivePokemonRenderer {
                pokemon: PokemonRenderer::with(
                    ctx,
                    data,
                    position,
                    pokemon.map(|pokemon| *pokemon.pokemon.id()),
                    PokemonTexture::Back,
                ),
                status: PokemonStatusGui::with_known(ctx, data, position, pokemon),
            };
            self.renderer.push(r);
        }
    }
}

impl<ID, P: Deref<Target = Pokemon>> ActivePlayer<ID, Option<UnknownPokemon<P>>> {
    pub fn init(&mut self, ctx: &BattleGuiContext, data: &PokedexClientData) {
        let size = self.player.active.len() as u8;

        for (i, index) in self.player.active.iter().enumerate() {
            let position = BattleGuiPositionIndex::new(BattleGuiPosition::Top, i as u8, size);
            let pokemon = (*index)
                .map(|index| self.player.pokemon[index].as_ref())
                .flatten();
            let r = ActivePokemonRenderer {
                pokemon: PokemonRenderer::with(
                    ctx,
                    data,
                    position,
                    pokemon.map(|pokemon| *pokemon.pokemon.id()),
                    PokemonTexture::Front,
                ),
                status: PokemonStatusGui::with_unknown(ctx, data, position, pokemon),
            };
            self.renderer.push(r);
        }
        self.trainer = Some("rival".parse().unwrap());
    }
}
