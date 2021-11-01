use pokedex::{
    context::PokedexClientContext,
    engine::{graphics::ZERO, tetra::graphics::Color, EngineContext},
    pokemon::owned::OwnedPokemon,
    texture::PokemonTexture,
    Identifiable, TrainerId,
};

use battle::party::PlayerParty;

use crate::{
    context::BattleGuiContext,
    ui::{
        pokemon::{flicker::Flicker, PokemonRenderer, PokemonStatusGui},
        BattleGuiPosition, BattleGuiPositionIndex,
    },
    view::InitUnknownPokemon,
};

pub type InitLocalPlayer<'d, ID> = PlayerParty<ID, usize, OwnedPokemon<'d>>;
pub type InitRemotePlayer<'d, ID> = PlayerParty<ID, usize, Option<InitUnknownPokemon<'d>>>;

pub type GuiLocalPlayer<'d, ID> = ActivePlayer<ID, OwnedPokemon<'d>>;
pub type GuiRemotePlayer<'d, ID> = ActivePlayer<ID, Option<InitUnknownPokemon<'d>>>;

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
    pub fn draw(&self, ctx: &mut EngineContext) {
        self.pokemon.draw(ctx, ZERO, Color::WHITE);
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

impl<'d, ID> ActivePlayer<ID, OwnedPokemon<'d>> {
    pub fn init(&mut self, ctx: &BattleGuiContext, dex: &PokedexClientContext) {
        let size = self.player.active.len() as u8;

        for (i, index) in self.player.active.iter().enumerate() {
            let position = BattleGuiPositionIndex::new(BattleGuiPosition::Bottom, i as u8, size);
            let pokemon = (*index).map(|index| &self.player.pokemon[index]);
            let r = ActivePokemonRenderer {
                pokemon: PokemonRenderer::with(
                    ctx,
                    dex,
                    position,
                    pokemon.map(|pokemon| *pokemon.pokemon.id()),
                    PokemonTexture::Back,
                ),
                status: PokemonStatusGui::with_known(ctx, dex, position, pokemon),
            };
            self.renderer.push(r);
        }
    }
}

impl<'d, ID> ActivePlayer<ID, Option<InitUnknownPokemon<'d>>> {
    pub fn init(&mut self, ctx: &BattleGuiContext, dex: &PokedexClientContext) {
        let size = self.player.active.len() as u8;

        for (i, index) in self.player.active.iter().enumerate() {
            let position = BattleGuiPositionIndex::new(BattleGuiPosition::Top, i as u8, size);
            let pokemon = (*index)
                .map(|index| self.player.pokemon[index].as_ref())
                .flatten();
            let r = ActivePokemonRenderer {
                pokemon: PokemonRenderer::with(
                    ctx,
                    dex,
                    position,
                    pokemon.map(|pokemon| *pokemon.pokemon.id()),
                    PokemonTexture::Front,
                ),
                status: PokemonStatusGui::with_unknown(ctx, dex, position, pokemon),
            };
            self.renderer.push(r);
        }
        self.trainer = Some("rival".parse().unwrap());
    }
}
