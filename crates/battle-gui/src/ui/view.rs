use core::ops::Deref;
use pokedex::{item::Item, moves::Move, pokemon::Pokemon, NpcGroupId};

use pokedex::{
    engine::{graphics::Color, math::vec2, Context},
    pokemon::owned::OwnedPokemon,
    texture::PokemonTexture,
    Identifiable, PokedexClientData,
};

use battle::{party::PlayerParty, pokemon::remote::UnknownPokemon, prelude::BattleData};

use crate::{
    context::BattleGuiData,
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
    pub npc_group: Option<NpcGroupId>,
    pub data: BattleData,
}

#[derive(Clone)]
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
    pub fn local(
        player: &PlayerParty<ID, usize, OwnedPokemon<P, M, I>>,
        ctx: &BattleGuiData,
        data: &PokedexClientData,
    ) -> Vec<ActivePokemonRenderer> {
        let size = player.active.len() as u8;

        player
            .active
            .iter()
            .enumerate()
            .map(|(i, index)| {
                let position =
                    BattleGuiPositionIndex::new(BattleGuiPosition::Bottom, i as u8, size);
                let pokemon = (*index).map(|index| &player.pokemon[index]);
                ActivePokemonRenderer {
                    pokemon: PokemonRenderer::with(
                        ctx,
                        data,
                        position,
                        pokemon.map(|pokemon| *pokemon.pokemon.id()),
                        PokemonTexture::Back,
                    ),
                    status: PokemonStatusGui::with_known(ctx, data, position, pokemon),
                }
            })
            .collect()
    }
}

impl<ID, P: Deref<Target = Pokemon>> ActivePlayer<ID, Option<UnknownPokemon<P>>> {
    pub fn remote(
        player: &PlayerParty<ID, usize, Option<UnknownPokemon<P>>>,
        ctx: &BattleGuiData,
        data: &PokedexClientData,
    ) -> Vec<ActivePokemonRenderer> {
        let size = player.active.len() as u8;

        player
            .active
            .iter()
            .enumerate()
            .map(|(i, index)| {
                let position = BattleGuiPositionIndex::new(BattleGuiPosition::Top, i as u8, size);
                let pokemon = (*index)
                    .map(|index| player.pokemon[index].as_ref())
                    .flatten();
                ActivePokemonRenderer {
                    pokemon: PokemonRenderer::with(
                        ctx,
                        data,
                        position,
                        pokemon.map(|pokemon| *pokemon.pokemon.id()),
                        PokemonTexture::Front,
                    ),
                    status: PokemonStatusGui::with_unknown(ctx, data, position, pokemon),
                }
            })
            .collect()
    }
}
