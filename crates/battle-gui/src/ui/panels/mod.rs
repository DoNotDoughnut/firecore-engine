use ::battle::party::PlayerParty;
use pokedex::{
    engine::{
        input::{pressed, Control},
        util::{Entity, Reset},
        EngineContext,
    },
    item::ItemId,
    moves::MoveTarget,
    pokemon::owned::OwnedPokemon,
};

use crate::view::GuiPokemonView;

use self::{battle::BattleOptions, fight::FightPanel, target::TargetPanel};

pub mod move_info;
pub mod moves;
pub mod target;

pub mod battle;
pub mod fight;

pub mod level;

pub struct BattlePanel<'d> {
    alive: bool,

    pub active: BattlePanels,

    pub battle: BattleOptions,
    pub fight: FightPanel<'d>,
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

impl<'d> BattlePanel<'d> {
    pub fn new() -> Self {
        Self {
            alive: false,
            active: BattlePanels::default(),
            battle: BattleOptions::new(),
            fight: FightPanel::new(),
            targets: TargetPanel::new(),
        }
    }

    pub fn user(&mut self, instance: &OwnedPokemon<'d>) {
        self.battle.setup(instance);
        self.fight.user(instance);
        self.battle.cursor = 0;
        self.fight.moves.cursor = 0;
        self.targets.cursor = 0;
        self.spawn();
    }

    pub fn target<ID, P: GuiPokemonView<'d>, const AS: usize>(&mut self, targets: &PlayerParty<ID, usize, P, AS>) {
        self.targets.update_names(targets);
    }

    pub fn input(
        &mut self,
        ctx: &EngineContext,
        pokemon: &OwnedPokemon<'d>,
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

    pub fn draw(&self, ctx: &mut EngineContext) {
        if self.alive {
            match self.active {
                BattlePanels::Main => self.battle.draw(ctx),
                BattlePanels::Fight => self.fight.draw(ctx),
                BattlePanels::Target(..) => self.targets.draw(ctx),
            }
        }
    }
}

impl<'d> Entity for BattlePanel<'d> {
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
