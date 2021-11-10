use core::ops::Deref;
use pokedex::{item::Item, pokemon::Pokemon};

use pokedex::{
    engine::{
        input::controls::{pressed, Control},
        util::{Entity, Reset},
        Context,
    },
    item::ItemId,
    moves::{owned::OwnedMove, Move, MoveTarget},
    pokemon::owned::OwnablePokemon,
};

use crate::view::PlayerView;

use self::{battle::BattleOptions, fight::FightPanel, target::TargetPanel};

pub mod move_info;
pub mod moves;
pub mod target;

pub mod battle;
pub mod fight;

pub mod level;

pub struct BattlePanel<M: Deref<Target = Move> + Clone> {
    alive: bool,

    pub active: BattlePanels,

    pub battle: BattleOptions,
    pub fight: FightPanel<M>,
    pub targets: TargetPanel,
}

pub enum BattlePanels {
    Main,
    Fight,
    Target(MoveTarget, Option<ItemId>),
}

impl Default for BattlePanels {
    fn default() -> Self {
        Self::Main
    }
}

impl<M: Deref<Target = Move> + Clone> BattlePanel<M> {
    pub fn new() -> Self {
        Self {
            alive: false,
            active: BattlePanels::default(),
            battle: BattleOptions::new(),
            fight: FightPanel::new(),
            targets: TargetPanel::new(),
        }
    }

    pub fn user<P: Deref<Target = Pokemon>, MSET: Deref<Target = [OwnedMove<M>]>, I, G, H>(
        &mut self,
        instance: &OwnablePokemon<P, MSET, I, G, H>,
    ) {
        self.battle.setup(instance);
        self.fight.user(instance);
        self.battle.cursor = 0;
        self.fight.moves.cursor = 0;
        self.targets.cursor = 0;
        self.spawn();
    }

    pub fn target<ID, P: Deref<Target = Pokemon>, I: Deref<Target = Item>>(
        &mut self,
        targets: &dyn PlayerView<ID, P, M, I>,
    ) {
        self.targets.update_names(targets);
    }

    pub fn input<P, MSET: Deref<Target = [OwnedMove<M>]>, I, G, H>(
        &mut self,
        ctx: &Context,
        pokemon: &OwnablePokemon<P, MSET, I, G, H>,
    ) -> Option<BattlePanels> {
        if self.alive {
            match self.active {
                BattlePanels::Main => {
                    self.battle.input(ctx);
                    pressed(ctx, Control::A).then(|| BattlePanels::Main)
                }
                BattlePanels::Fight => {
                    if pressed(ctx, Control::B) {
                        self.active = BattlePanels::Main;
                    }
                    self.fight.input(ctx, pokemon);
                    pressed(ctx, Control::A).then(|| BattlePanels::Fight)
                }
                BattlePanels::Target(..) => {
                    if pressed(ctx, Control::B) {
                        self.active = BattlePanels::Fight;
                    }
                    self.targets.input(ctx);
                    pressed(ctx, Control::A).then(|| std::mem::take(&mut self.active))
                }
            }
        } else {
            None
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        if self.alive {
            match self.active {
                BattlePanels::Main => self.battle.draw(ctx),
                BattlePanels::Fight => self.fight.draw(ctx),
                BattlePanels::Target(..) => self.targets.draw(ctx),
            }
        }
    }
}

impl<M: Deref<Target = Move> + Clone> Entity for BattlePanel<M> {
    fn spawn(&mut self) {
        self.alive = true;
        self.active = BattlePanels::default();
        self.fight.reset();
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn alive(&self) -> bool {
        self.alive
    }
}
