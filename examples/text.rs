use engine::{
    context::GameContext,
    gui::MessageBox,
    tetra::{ContextBuilder, Result, State, graphics::Color},
    text::{Message, MessagePage, TextColor},
    util::Entity,
    Context,
};
use firecore_engine as engine;

fn main() -> Result {
    engine::build(
        &mut ContextBuilder::new("MessageBox", engine::WIDTH as _, engine::HEIGHT as _),
        firecore_dependencies::ser::deserialize(include_bytes!(
            "../../../pokemon-game/build/data/fonts.bin"
        ))
        .unwrap(),
    )?
    .run(Game::new)
}

struct Game {
    messagebox: MessageBox,
}

impl Game {
    pub fn new(_: &mut Context) -> Result<Self> {
        Ok(Self {
            messagebox: MessageBox::new(Default::default(), 0),
        })
    }
}

impl State<GameContext> for Game {
    fn begin(&mut self, ctx: &mut Context) -> Result {
        let page = MessagePage {
            lines: vec!["Test Page Test Page".to_owned(), "Page Test Page Test".to_owned()],
            wait: None,
        };
        self.messagebox.message = Message {
            pages: vec![page.clone(), page],
            color: TextColor::Black,
        };
        self.messagebox.spawn();
        Ok(())
    }

    fn end(&mut self, ctx: &mut Context) -> Result {
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> Result {
        if !self.messagebox.alive() {
            tetra::window::quit(ctx)
        } else {
            self.messagebox
                .update(ctx, tetra::time::get_delta_time(ctx).as_secs_f32());
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result {
        tetra::graphics::clear(ctx, Color::WHITE);
        self.messagebox.draw(ctx);
        Ok(())
    }

    fn event(&mut self, ctx: &mut Context, event: tetra::Event) -> Result {
        Ok(())
    }
}
