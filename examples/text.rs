extern crate firecore_tetra as tetra;

use engine::{
    gui::MessageBox,
    tetra::{graphics::Color, ContextBuilder, Result, State},
    text::{Message, MessagePage, TextColor},
    util::{Completable, Entity},
    EngineContext,
};
use firecore_engine as engine;

fn main() -> Result {
    let mut ctx = engine::build(
        &mut ContextBuilder::new("MessageBox", engine::WIDTH as _, engine::HEIGHT as _),
        bincode::deserialize(include_bytes!("../../../pokemon-game/build/data/fonts.bin")).unwrap(),
    )?;
    tetra::run(&mut ctx, Game::new)
}

struct Game {
    messagebox: MessageBox,
}

impl Game {
    pub fn new(_: &mut EngineContext) -> Result<Self> {
        Ok(Self {
            messagebox: MessageBox::new(Default::default(), 0),
        })
    }
}

impl State<EngineContext> for Game {
    fn begin(&mut self, _: &mut EngineContext) -> Result {
        let page = MessagePage {
            lines: vec![
                "Test Page Test Page".to_owned(),
                "Page Test Page Test".to_owned(),
            ],
            wait: None,
        };
        let page2 = MessagePage {
            lines: page.lines.clone(),
            wait: Some(1.0),
        };
        self.messagebox.message = Message {
            pages: vec![page, page2],
            color: TextColor::Black,
        };
        self.messagebox.spawn();
        Ok(())
    }

    fn update(&mut self, ctx: &mut EngineContext) -> Result {
        if !self.messagebox.alive() {
            tetra::window::quit(ctx)
        } else {
            let delta = tetra::time::get_delta_time(ctx).as_secs_f32();
            self.messagebox.update(ctx, delta);
            if self.messagebox.finished() {
                self.messagebox.despawn();
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut EngineContext) -> Result {
        tetra::graphics::clear(ctx, Color::WHITE);
        self.messagebox.draw(ctx);
        Ok(())
    }
}
