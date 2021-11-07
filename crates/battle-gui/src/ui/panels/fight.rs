use core::ops::Deref;

use pokedex::{
    engine::{util::Reset, Context},
    moves::{owned::OwnedMove, Move},
    pokemon::owned::OwnablePokemon,
};

use super::{move_info::MoveInfoPanel, moves::MovePanel};

pub struct FightPanel<M: Deref<Target = Move> + Clone> {
    pub moves: MovePanel<M>,
    info: MoveInfoPanel,
}

impl<M: Deref<Target = Move> + Clone> FightPanel<M> {
    pub fn new() -> Self {
        Self {
            moves: MovePanel::new(),
            info: MoveInfoPanel::new(),
        }
    }

    pub fn user<P, MSET: Deref<Target = [OwnedMove<M>]>, I, G, H>(&mut self, instance: &OwnablePokemon<P, MSET, I, G, H>) {
        self.moves.update_names(instance);
        self.update_move(instance);
    }

    pub fn update_move<P, MSET: Deref<Target = [OwnedMove<M>]>, I, G, H>(&mut self, pokemon: &OwnablePokemon<P, MSET, I, G, H>) {
        if let Some(pmove) = pokemon.moves.get(self.moves.cursor) {
            self.info.update_move(pmove);
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        self.moves.draw(ctx);
        self.info.draw(ctx);
    }

    pub fn input<P, MSET: Deref<Target = [OwnedMove<M>]>, I, G, H>(&mut self, ctx: &Context, pokemon: &OwnablePokemon<P, MSET, I, G, H>) {
        if self.moves.input(ctx) {
            self.update_move(pokemon);
        }
    }
}

impl<M: Deref<Target = Move> + Clone> Reset for FightPanel<M> {
    fn reset(&mut self) {
        if self.moves.cursor >= self.moves.names.len() {
            self.moves.cursor = 0;
        }
    }
}
