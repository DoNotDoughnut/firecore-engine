use core::ops::Deref;
use pokedex::pokemon::Pokemon;

use pokedex::{engine::Context, PokedexClientData};

use battle::data::BattleType;

use crate::{
    context::BattleGuiData,
    transition::TransitionState,
    ui::view::{ActivePokemonRenderer, GuiRemotePlayer},
};

use super::{BattleOpener, Openers, TrainerBattleOpener, WildBattleOpener};

pub struct BattleOpenerManager {
    current: Openers,

    wild: WildBattleOpener,
    trainer: TrainerBattleOpener,
}

impl BattleOpenerManager {
    pub fn new(ctx: &mut Context, gui: &BattleGuiData) -> Self {
        Self {
            current: Openers::default(),

            wild: WildBattleOpener::new(ctx, gui),
            trainer: TrainerBattleOpener::new(gui),
        }
    }

    pub fn begin<ID, P: Deref<Target = Pokemon>>(
        &mut self,
        ctx: &PokedexClientData,
        state: &mut TransitionState,
        battle_type: BattleType,
        opponent: &GuiRemotePlayer<ID, P>,
    ) {
        *state = TransitionState::Run;
        self.current = match battle_type {
            BattleType::Wild => Openers::Wild,
            BattleType::Trainer => Openers::Trainer,
            BattleType::GymLeader => Openers::Trainer,
        };
        let current = self.get_mut::<ID, P>();
        current.reset();
        current.spawn(ctx, opponent);
    }

    // pub fn end(&mut self, state: &mut TransitionState) {
    //     *state = TransitionState::Begin;
    // }

    pub fn update<ID, P: Deref<Target = Pokemon>>(
        &mut self,
        state: &mut TransitionState,
        delta: f32,
    ) {
        let current = self.get_mut::<ID, P>();
        current.update(delta);
        if current.finished() {
            *state = TransitionState::End;
        }
    }

    pub fn draw_below_panel<ID, P: Deref<Target = Pokemon>>(
        &self,
        ctx: &mut Context,
        player: &[ActivePokemonRenderer],
        opponent: &[ActivePokemonRenderer],
    ) {
        self.get::<ID, P>().draw_below_panel(ctx, player, opponent);
    }

    pub fn draw<ID, P: Deref<Target = Pokemon>>(&self, ctx: &mut Context) {
        self.get::<ID, P>().draw(ctx);
    }

    pub fn offset<ID, P: Deref<Target = Pokemon>>(&self) -> f32 {
        self.get::<ID, P>().offset()
    }

    fn get<ID, P: Deref<Target = Pokemon>>(&self) -> &dyn BattleOpener<ID, P> {
        match self.current {
            Openers::Wild => &self.wild,
            Openers::Trainer => &self.trainer,
        }
    }

    fn get_mut<ID, P: Deref<Target = Pokemon>>(&mut self) -> &mut dyn BattleOpener<ID, P> {
        match self.current {
            Openers::Wild => &mut self.wild,
            Openers::Trainer => &mut self.trainer,
        }
    }
}
