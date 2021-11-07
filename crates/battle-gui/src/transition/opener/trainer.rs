use core::ops::Deref;
use pokedex::pokemon::Pokemon;

use pokedex::{
    context::PokedexClientData,
    engine::{
        graphics::Texture,
        util::{Completable, Reset},
        Context,
    },
};

use crate::{
    context::BattleGuiContext,
    ui::view::{ActivePokemonRenderer, GuiRemotePlayer},
};

use super::{BattleOpener, DefaultBattleOpener};

pub struct TrainerBattleOpener {
    opener: DefaultBattleOpener,
    trainer: Option<Texture>,
}

impl TrainerBattleOpener {
    pub fn new(ctx: &BattleGuiContext) -> Self {
        Self {
            opener: DefaultBattleOpener::new(ctx),
            trainer: None,
        }
    }
}

impl<ID: Default, P: Deref<Target = Pokemon>> BattleOpener<ID, P> for TrainerBattleOpener {
    fn spawn(&mut self, ctx: &PokedexClientData, opponent: &GuiRemotePlayer<ID, P>) {
        if let Some(id) = &opponent.trainer {
            self.trainer = Some(ctx.trainer_textures.get(id).clone());
        }
    }

    fn update(&mut self, delta: f32) {
        self.opener.update(delta);
    }

    fn draw_below_panel(
        &self,
        ctx: &mut Context,
        player: &[ActivePokemonRenderer],
        opponent: &[ActivePokemonRenderer],
    ) {
        if let Some(texture) = self.trainer.as_ref() {
            texture.draw(
                ctx,
                144.0 - self.opener.offset,
                74.0 - texture.height(),
                Default::default(),
            );
        }
        self.opener.draw_below_panel(ctx, player, opponent);
    }

    fn draw(&self, ctx: &mut Context) {
        self.opener.draw(ctx);
    }

    fn offset(&self) -> f32 {
        self.opener.offset
    }
}

impl Reset for TrainerBattleOpener {
    fn reset(&mut self) {
        self.opener.reset();
        self.trainer = None;
    }
}

impl Completable for TrainerBattleOpener {
    fn finished(&self) -> bool {
        self.opener.finished()
    }
}
