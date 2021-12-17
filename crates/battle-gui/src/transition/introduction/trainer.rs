use core::ops::Deref;
use pokedex::{item::Item, moves::Move, pokemon::Pokemon};

use pokedex::{
    engine::{
        graphics::Texture,
        gui::MessageBox,
        text::MessagePage,
        utils::{Completable, Reset},
        Context,
    },
    PokedexClientData,
};

use battle::data::BattleType;

use crate::{
    context::BattleGuiContext,
    ui::view::{ActivePokemonRenderer, GuiLocalPlayer, GuiRemotePlayer},
};

use super::{basic::BasicBattleIntroduction, BattleIntroduction};

pub struct TrainerBattleIntroduction {
    introduction: BasicBattleIntroduction,

    texture: Option<Texture>,
    offset: f32,
    leaving: bool,
}

impl TrainerBattleIntroduction {
    const FINAL_TRAINER_OFFSET: f32 = 126.0;

    pub fn new(ctx: &BattleGuiContext) -> Self {
        Self {
            introduction: BasicBattleIntroduction::new(ctx),
            texture: None,
            offset: 0.0,
            leaving: false,
        }
    }
}

impl<ID: Default, P: Deref<Target = Pokemon>, M: Deref<Target = Move>, I: Deref<Target = Item>>
    BattleIntroduction<ID, P, M, I> for TrainerBattleIntroduction
{
    fn spawn(
        &mut self,
        ctx: &PokedexClientData,
        _battle_type: BattleType,
        player: &GuiLocalPlayer<ID, P, M, I>,
        opponent: &GuiRemotePlayer<ID, P>,
        text: &mut MessageBox,
    ) {
        text.clear();

        if let Some(id) = &opponent.npc_group {
            self.texture = Some(ctx.npc_group_textures.get(id).clone());
        }

        if let Some(name) = &opponent.player.name {
            text.push(MessagePage {
                lines: vec![name.to_owned(), "would like to battle!".to_owned()],
                wait: None,
            });

            text.push(MessagePage {
                lines: vec![
                    format!("{} sent", name),
                    format!(
                        "out {}",
                        BasicBattleIntroduction::concatenate(&opponent.player)
                    ),
                ],
                wait: Some(0.5),
            });
        } else {
            text.push(MessagePage {
                lines: vec![String::from("No trainer data found!")],
                wait: None,
            });
        }

        self.introduction.common_setup(text, player);
    }

    fn update(
        &mut self,
        ctx: &Context,
        delta: f32,
        player: &mut GuiLocalPlayer<ID, P, M, I>,
        opponent: &mut GuiRemotePlayer<ID, P>,
        text: &mut MessageBox,
    ) {
        self.introduction.update(ctx, delta, player, opponent, text);
        if text.waiting() && text.page() == text.pages() - 2 {
            self.leaving = true;
        }
        if self.leaving && self.offset < Self::FINAL_TRAINER_OFFSET {
            self.offset += 300.0 * delta;
        }
    }

    fn draw(
        &self,
        ctx: &mut Context,
        player: &[ActivePokemonRenderer],
        opponent: &[ActivePokemonRenderer],
    ) {
        if self.offset < Self::FINAL_TRAINER_OFFSET {
            if let Some(texture) = &self.texture {
                texture.draw(
                    ctx,
                    144.0 + self.offset,
                    74.0 - texture.height(),
                    Default::default(),
                );
            }
        } else {
            self.introduction.draw_opponent(ctx, opponent);
        }
        self.introduction.draw_player(ctx, player);
    }
}

impl Completable for TrainerBattleIntroduction {
    fn finished(&self) -> bool {
        self.introduction.finished()
    }
}

impl Reset for TrainerBattleIntroduction {
    fn reset(&mut self) {
        self.introduction.reset();
        self.offset = 0.0;
        self.leaving = false;
    }
}
