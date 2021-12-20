use core::ops::Deref;
use pokedex::{item::Item, moves::Move, pokemon::Pokemon, engine::utils::Reset};

use pokedex::{
    engine::{
        gui::MessageBox,
        utils::{Completable, Entity},
        Context,
    },
    PokedexClientData,
};

use battle::data::BattleType;

use crate::{
    context::BattleGuiData,
    ui::view::{ActivePokemonRenderer, GuiLocalPlayer, GuiRemotePlayer},
};

use super::TransitionState;

mod basic;
mod trainer;

pub use basic::*;
pub use trainer::*;

pub enum Introductions {
    Basic,
    Trainer,
}

impl Default for Introductions {
    fn default() -> Self {
        Self::Basic
    }
}

pub(crate) trait BattleIntroduction<
    ID,
    P: Deref<Target = Pokemon>,
    M: Deref<Target = Move>,
    I: Deref<Target = Item>,
>: Completable
{
    fn spawn(
        &mut self,
        ctx: &PokedexClientData,
        battle_type: BattleType,
        player: &GuiLocalPlayer<ID, P, M, I>,
        opponent: &GuiRemotePlayer<ID, P>,
        text: &mut MessageBox,
    );

    fn update(
        &mut self,
        ctx: &Context,
        delta: f32,
        player: &mut GuiLocalPlayer<ID, P, M, I>,
        opponent: &mut GuiRemotePlayer<ID, P>,
        text: &mut MessageBox,
    );

    fn draw(
        &self,
        ctx: &mut Context,
        player: &[ActivePokemonRenderer],
        opponent: &[ActivePokemonRenderer],
    );
}

pub struct BattleIntroductionManager {
    current: Introductions,

    basic: BasicBattleIntroduction,
    trainer: TrainerBattleIntroduction,
}

impl BattleIntroductionManager {
    pub fn new(ctx: &BattleGuiData) -> Self {
        Self {
            current: Introductions::default(),

            basic: BasicBattleIntroduction::new(ctx),
            trainer: TrainerBattleIntroduction::new(ctx),
        }
    }

    pub fn begin<
        ID,
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &mut self,
        ctx: &PokedexClientData,
        state: &mut TransitionState,
        battle_type: BattleType,
        player: &GuiLocalPlayer<ID, P, M, I>,
        opponent: &GuiRemotePlayer<ID, P>,
        text: &mut MessageBox,
    ) {
        *state = TransitionState::Run;
        match battle_type {
            BattleType::Wild => self.current = Introductions::Basic,
            _ => self.current = Introductions::Trainer,
        }
        let current = self.get_mut();
        current.reset();
        current.spawn(ctx, battle_type, player, opponent, text);
        text.spawn();
    }

    pub fn end(&mut self, text: &mut MessageBox) {
        text.pages.clear();
        text.reset();
    }

    pub fn update<
        ID,
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &mut self,
        state: &mut TransitionState,
        ctx: &Context,
        delta: f32,
        player: &mut GuiLocalPlayer<ID, P, M, I>,
        opponent: &mut GuiRemotePlayer<ID, P>,
        text: &mut MessageBox,
    ) {
        let current = self.get_mut::<ID, P, M, I>();
        current.update(ctx, delta, player, opponent, text);
        if current.finished() {
            *state = TransitionState::End;
        }
    }

    pub fn draw<
        ID,
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &self,
        ctx: &mut Context,
        player: &[ActivePokemonRenderer],
        opponent: &[ActivePokemonRenderer],
    ) {
        self.get::<ID, P, M, I>().draw(ctx, player, opponent);
    }

    fn get<
        ID,
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &self,
    ) -> &dyn BattleIntroduction<ID, P, M, I> {
        match self.current {
            Introductions::Basic => &self.basic,
            Introductions::Trainer => &self.trainer,
        }
    }

    fn get_mut<
        ID,
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &mut self,
    ) -> &mut dyn BattleIntroduction<ID, P, M, I> {
        match self.current {
            Introductions::Basic => &mut self.basic,
            Introductions::Trainer => &mut self.trainer,
        }
    }
}
