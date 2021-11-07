use core::{cell::Cell, ops::Deref};

use engine::{
    graphics::{draw_cursor, draw_text_left, DrawParams, Texture},
    gui::Panel,
    input::controls::{pressed, Control},
    text::TextColor,
    util::HEIGHT,
    Context,
};

use pokedex::item::{Item, ItemStack};

use crate::context::PokedexClientData;

// const WORLD_OPTIONS: &[&'static str] = &[
//     "Use",
//     "Give",
//     "Toss",
// ];

type TextOption = &'static [&'static str];

const BATTLE_OPTIONS: TextOption = &["Use"];

pub struct BagGui {
    alive: Cell<bool>,
    background: Texture,

    // offset: Cell<usize>,
    cursor: Cell<usize>,

    selecting: Cell<bool>,
    select_cursor: Cell<usize>,
    // select_text: Cell<Option<TextOption>>,
    // items: [Cell<Option<TinyStr4>>; 12],
    selected: Cell<Option<usize>>,
}

impl BagGui {
    pub fn new(ctx: &PokedexClientData) -> Self {
        Self {
            alive: Default::default(),
            background: ctx.bag_background.clone(),
            // offset: Default::default(),
            cursor: Default::default(),
            selecting: Default::default(),
            select_cursor: Default::default(),
            // items: Default::default(),
            selected: Default::default(),
        }
    }

    pub fn input<I>(&self, ctx: &Context, items: &[ItemStack<I>]) {
        match self.selecting.get() {
            true => {
                // match self.select_text {
                // Some(text) => {
                let cursor = self.cursor.get();
                if pressed(ctx, Control::B) {
                    self.selecting.set(false);
                }
                if pressed(ctx, Control::Up) && cursor > 0 {
                    self.select_cursor.set(self.select_cursor.get() - 1);
                }
                if pressed(ctx, Control::Down) && cursor < BATTLE_OPTIONS.len() {
                    self.select_cursor.set(self.select_cursor.get() + 1);
                }
                if pressed(ctx, Control::A) {
                    match cursor {
                        0 => {
                            self.selected.set(Some(cursor));
                        }
                        1 => (), // cancel
                        _ => unreachable!("Selected an option that is not use/cancel"),
                    }
                    self.selecting.set(false);
                }

                // }
                //     None => self.selecting = false,
                // }
            }
            false => {
                if pressed(ctx, Control::B) {
                    self.despawn();
                }
                let cursor = self.cursor.get();
                if pressed(ctx, Control::A) {
                    if cursor < items.len() {
                        self.spawn_select();
                    } else {
                        self.despawn();
                    }
                }
                if pressed(ctx, Control::Up) && cursor > 0 {
                    self.cursor.set(cursor - 1);
                }
                if pressed(ctx, Control::Down) {
                    if cursor < items.len() {
                        self.cursor.set(cursor + 1);
                    }
                }
            }
        }
    }

    pub fn draw<I: Deref<Target = Item>>(
        &self,
        ctx: &mut Context,
        dex: &PokedexClientData,
        items: &[ItemStack<I>],
    ) {
        self.background.draw(ctx, 0.0, 0.0, Default::default());
        let cursor = self.cursor.get();
        for (index, stack) in items.iter().enumerate() {
            let y = 11.0 + (index << 4) as f32;
            draw_text_left(
                ctx,
                &1,
                &stack.item.name,
                98.0,
                y,
                DrawParams::color(TextColor::Black.into()),
            );
            draw_text_left(
                ctx,
                &1,
                "x",
                200.0,
                y,
                DrawParams::color(TextColor::Black.into()),
            );
            // if let Some(ref count) = self.items.get(index - self.offset.get()).map(|cell| cell.get()).flatten() {
            //     draw_text_left(ctx, &1, &count, TextColor::Black, 208.0, y);
            // }
        }
        draw_text_left(
            ctx,
            &1,
            "Cancel",
            98.0,
            11.0 + (items.len() << 4) as f32,
            DrawParams::color(TextColor::Black.into()),
        );
        if let Some(stack) = items.get(cursor) {
            if let Some(texture) = dex.item_textures.try_get(&stack.item.id) {
                texture.draw(ctx, 8.0, 125.0, Default::default());
            }
            for (index, line) in stack.item.description.lines().enumerate() {
                draw_text_left(
                    ctx,
                    &1,
                    line,
                    41.0,
                    117.0 + (index * 14) as f32,
                    DrawParams {
                        color: TextColor::White.into(),
                        ..Default::default()
                    },
                );
            }
        }
        draw_cursor(ctx, 91.0, 13.0 + (cursor << 4) as f32, Default::default());
        if self.selecting.get() {
            // if let Some(text) = self.select_text {
            Panel::draw_text(
                ctx,
                146.0,
                HEIGHT,
                94.0,
                &BATTLE_OPTIONS,
                self.select_cursor.get(),
                true,
                true,
            )
            // }
        }
    }

    fn spawn_select(&self) {
        self.selecting.set(true);
        self.select_cursor.set(0);
    }

    // fn set_cell<'d>(&self, index: usize, stack: Option<&ItemRefStack<'d>>) {
    //     if let Some(cell) = self.items.get(index) {
    //         cell.set(stack.map(|stack| to_ascii4(stack.count).ok()).flatten())
    //     }
    // }

    pub fn take_selected_despawn<I: Deref<Target = Item> + Clone>(&self, items: &mut [ItemStack<I>]) -> Option<I> {
        let selected = self.selected.get();
        selected
            .map(|selected| {
                self.selected.set(None);
                let item = items[selected].try_use().then(|| items[selected].item.clone());
                self.despawn();
                item
            })
            .flatten()
    }

    pub fn spawn(&self) {
        self.alive.set(true);
        // self.select_text.set(Some(BATTLE_OPTIONS));
    }

    pub fn despawn(&self) {
        self.alive.set(false);
        // self.items.clear()
    }

    pub fn alive(&self) -> bool {
        self.alive.get()
    }
}
