use firecore_battle_gui::pokedex::engine::{
    self,
    graphics::{self, scaling::ScreenScaler, Color},
    gui::{MessageBox, Panel},
    state::State,
    text::{Message, MessagePage, TextColor},
    util::{Completable, Entity},
    ContextBuilder, DefaultContext,
};

fn main() {
    engine::run(
        ContextBuilder::new(
            "MessageBox",
            2 * engine::util::WIDTH as i32,
            (2.0 * engine::util::HEIGHT) as _,
        ),
        move |context| async { DefaultContext(context) },
        |_| Game::new(),
    )
}

struct Game {
    messagebox: MessageBox,
}

impl Game {
    pub fn new() -> Self {
        Self {
            messagebox: MessageBox::new(Default::default(), 0),
        }
    }
}

impl State<DefaultContext> for Game {
    fn start(&mut self, ctx: &mut DefaultContext) {
        let scaler = ScreenScaler::with_size(
            ctx,
            engine::util::WIDTH as _,
            engine::util::HEIGHT as _,
            graphics::scaling::ScalingMode::ShowAllPixelPerfect,
        );

        engine::graphics::scaling::set_scaler(ctx, scaler);

        //-> Result {
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
        // Ok(())
    }

    fn update(&mut self, ctx: &mut DefaultContext, delta: f32) {
        //-> Result {
        if !self.messagebox.alive() {
            engine::quit(ctx)
        } else {
            self.messagebox.update(ctx, delta);
            if self.messagebox.finished() {
                self.messagebox.despawn();
            }
        }
        // Ok(())
    }

    fn draw(&mut self, ctx: &mut DefaultContext) {
        //-> Result<(), ()> {
        graphics::clear(ctx, Color::rgb(0.1, 0.2, 0.56));
        Panel::draw(
            ctx,
            10.0,
            10.0,
            engine::util::WIDTH - 20.0,
            engine::util::HEIGHT - 20.0,
        );
        self.messagebox.draw(ctx);
        // Ok(())
    }
}
