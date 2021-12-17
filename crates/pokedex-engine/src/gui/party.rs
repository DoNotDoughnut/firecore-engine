use core::{cell::Cell, ops::Deref};

use pokedex::pokemon::{
    owned::OwnablePokemon,
    party::{Party, PARTY_SIZE},
    Health, Pokemon,
};

use engine::{
    graphics::{draw_rectangle, draw_straight_line, draw_text_left, draw_text_right},
    graphics::{Color, DrawParams, Texture},
    input::controls::{pressed, Control},
    math::{Rectangle, Vec2},
    text::MessagePage,
    Context,
};

use crate::{data::PokedexClientData, gui::cellref};

use self::select::PartySelectMenu;
use self::summary::SummaryGui;

use super::{health::HealthBar, LEVEL_PREFIX};

pub mod select;
pub mod summary;

mod cell;
use cell::*;

pub struct PartyGui {
    alive: Cell<bool>,

    select: PartySelectMenu,
    summary: SummaryGui,

    background: Texture,
    ball: Texture,
    health: HealthBar,

    pokemon: [PartyCell; PARTY_SIZE],

    selected: Cell<Option<usize>>,

    accumulator: Cell<f32>,

    cursor: Cell<usize>,
    right_cursor: Cell<Option<usize>>,

    exitable: Cell<bool>,
}

impl PartyGui {
    const LIGHT: Color = Color::rgb(128.0 / 255.0, 192.0 / 255.0, 216.0 / 255.0);
    const DARK: Color = Color::rgb(56.0 / 255.0, 144.0 / 255.0, 216.0 / 255.0);

    const HOVER_LIGHT: Color = Color::rgb(168.0 / 255.0, 232.0 / 255.0, 248.0 / 255.0);
    const HOVER_DARK: Color = Color::rgb(120.0 / 255.0, 208.0 / 255.0, 232.0 / 255.0);

    const HOVER_BORDER: Color = Color::rgb(248.0 / 255.0, 112.0 / 255.0, 48.0 / 255.0);

    const SELECT_LIGHT: Color = Color::rgb(176.0 / 255.0, 248.0 / 255.0, 160.0 / 255.0);
    const SELECT_DARK: Color = Color::rgb(120.0 / 255.0, 216.0 / 255.0, 128.0 / 255.0);

    const SELECT_BORDER: Color = Color::rgb(248.0 / 255.0, 248.0 / 255.0, 112.0 / 255.0);

    const SELECT_CORNER: Color = Color::rgb(120.0 / 255.0, 152.0 / 255.0, 96.0 / 255.0);

    pub fn new(ctx: &PokedexClientData) -> Self {
        Self {
            alive: Default::default(),
            select: PartySelectMenu::new(ctx),
            summary: SummaryGui::new(ctx),
            background: ctx.party.background.clone(),
            ball: ctx.party.ball.clone(),
            health: HealthBar::new(ctx),
            accumulator: Default::default(),
            pokemon: Default::default(),
            cursor: Default::default(),
            right_cursor: Default::default(),
            selected: Default::default(),
            exitable: Cell::new(true),
        }
    }

    pub fn on_spawn(&self, world: Option<bool>) {
        self.alive.set(true);
        self.reset();
        self.select.is_world.set(world);
    }

    pub fn spawn<P: Deref<Target = Pokemon>, M, I, G>(
        &self,
        ctx: &PokedexClientData,
        party: &Party<OwnablePokemon<P, M, I, G, Health>>,
        is_world: Option<bool>,
        exitable: bool,
    ) {
        self.on_spawn(is_world);
        self.exitable.set(exitable);
        for (index, pokemon) in party.iter().enumerate() {
            self.pokemon[index].init(ctx, pokemon);
        }
    }

    pub fn input<P: Deref<Target = Pokemon>, M, I, G>(
        &self,
        ctx: &Context,
        dex: &PokedexClientData,
        party: &mut [OwnablePokemon<P, M, I, G, Health>],
    ) {
        if self.summary.alive() {
            self.summary.input(ctx);
        } else if self.select.alive.get() {
            if let Some(action) = self.select.input(ctx) {
                let cursor = self.cursor.get();
                match action {
                    select::PartySelectAction::Select => {
                        self.selected.set(Some(cursor));
                        self.select.alive.set(false);
                    }
                    select::PartySelectAction::Summary => {
                        self.summary
                            .spawn(dex, &party[cursor], &self.pokemon[cursor]);
                        self.select.alive.set(false);
                    }
                }
            }
        } else if pressed(ctx, Control::A) {
            let is_world = self.select.is_world.get();
            if let Some(selected) = self.take_selected() {
                if let Some(is_world) = is_world {
                    if is_world {
                        let old = self.cursor.get();
                        party.swap(old, selected);
                    }
                }
            } else if is_world.is_some() {
                self.select.toggle();
            } else {
                self.selected.set(Some(self.cursor.get()));
            }
        } else {
            let cursor = self.cursor.get();
            if pressed(ctx, Control::Up) && cursor > 1 {
                self.cursor.set(cursor - 1);
            }
            if pressed(ctx, Control::Down) {
                if cursor < party.len() - 1 {
                    self.cursor.set(cursor + 1);
                }
            }
            if pressed(ctx, Control::Left) && cursor != 0 {
                self.right_cursor.set(Some(cursor));
                self.cursor.set(0);
            }
            if pressed(ctx, Control::Right) && cursor == 0 {
                self.cursor.set(self.right_cursor.get().unwrap_or(1));
            }
            if (pressed(ctx, Control::B) || pressed(ctx, Control::Start)) && self.exitable.get() {
                self.despawn();
            }
        }
    }

    pub fn update(&self, delta: f32) {
        if self.alive.get() {
            let acc = self.accumulator.get() + delta;
            self.accumulator.set(if acc > PartyCell::ICON_TICK * 2.0 {
                0.0
            } else {
                acc
            });
            if let Some(is_world) = self.select.is_world.get() {
                if is_world && self.summary.alive() {
                    self.summary.update(delta);
                }
            }
        }
    }

    pub fn draw<P: Deref<Target = Pokemon>, M, I, G>(
        &self,
        ctx: &mut Context,
        party: &[OwnablePokemon<P, M, I, G, Health>],
    ) {
        // deps::log::debug!("to - do: /party brings up party gui");
        if self.summary.alive() {
            match self.selected.get() {
                Some(selected) => self.summary.draw(ctx, &party[selected]),
                None => self.summary.despawn(),
            }
        } else {
            self.background.draw(ctx, 0.0, 0.0, Default::default());
            party.iter().enumerate().for_each(|(index, pokemon)| {
                let cell = &self.pokemon[index];
                if index == 0 {
                    self.draw_primary(ctx, pokemon, cell);
                } else {
                    self.draw_cell(ctx, index, pokemon, cell, self.cursor.get() == index);
                }
            });
            if self.select.is_world.get().is_some() {
                self.select.draw(ctx);
            }
        }
    }

    fn draw_primary<P: Deref<Target = Pokemon>, M, I, G>(
        &self,
        ctx: &mut Context,
        pokemon: &OwnablePokemon<P, M, I, G, Health>,
        cell: &PartyCell,
    ) {
        let selected = self.cursor.get() == 0;
        let mut skip = false;
        if self.select.is_world.get().is_some() {
            if let Some(selected_index) = self.selected.get() {
                let selected_index = selected_index == 0;
                if selected_index || selected {
                    draw_straight_line(ctx, 10.5, 28.0, 45.0, false, 2.0, Self::SELECT_LIGHT);
                    draw_straight_line(ctx, 10.0, 28.5, 74.0, true, 2.0, Self::SELECT_LIGHT);

                    draw_straight_line(ctx, 83.5, 28.0, 45.0, false, 1.0, Self::SELECT_CORNER);
                    draw_straight_line(ctx, 10.0, 72.5, 74.0, true, 1.0, Self::SELECT_CORNER);
                    self.draw_primary_color(
                        ctx,
                        Self::SELECT_LIGHT,
                        Self::SELECT_DARK,
                        Some(if selected {
                            Self::HOVER_BORDER
                        } else {
                            Self::SELECT_BORDER
                        }),
                    );
                    skip = true;
                }
            }
        }
        if !skip {
            if selected {
                self.draw_primary_color(
                    ctx,
                    Self::HOVER_LIGHT,
                    Self::HOVER_DARK,
                    Some(Self::HOVER_BORDER),
                );
            } else {
                self.draw_primary_color(ctx, Self::LIGHT, Self::DARK, None);
            }
        }
        self.draw_ball(ctx, 3.0, 20.0, selected);
        if let Some(icon) = cellref(&cell.icon) {
            self.draw_pokemon(ctx, icon, 0.0, 20.0, selected);
        }
        draw_text_left(
            ctx,
            &0,
            pokemon.name(),
            33.0,
            36.0,
            DrawParams::color(MessagePage::WHITE),
        );
        draw_text_left(
            ctx,
            &0,
            LEVEL_PREFIX,
            41.0,
            45.0,
            DrawParams::color(MessagePage::WHITE),
        );
        draw_text_left(
            ctx,
            &0,
            cell.level.get(),
            51.0,
            45.0,
            DrawParams::color(MessagePage::WHITE),
        );
        self.draw_health(ctx, cell, 17.0, 57.0);
    }

    fn draw_primary_color(
        &self,
        ctx: &mut Context,
        light: Color,
        dark: Color,
        border: Option<Color>,
    ) {
        draw_rectangle(ctx, 11.0, 29.0, 72.0, 27.0, dark);
        draw_straight_line(ctx, 11.0, 56.5, 72.0, true, 1.0, light);
        draw_straight_line(ctx, 11.0, 57.5, 72.0, true, 1.0, dark);
        draw_rectangle(ctx, 11.0, 58.0, 72.0, 14.0, light);
        if let Some(border) = border {
            draw_straight_line(ctx, 9.0, 27.0, 76.0, true, 2.0, border);
            draw_straight_line(ctx, 9.0, 27.0, 47.0, false, 2.0, border);
            draw_straight_line(ctx, 9.0, 74.0, 75.0, true, 2.0, border);
            draw_straight_line(ctx, 85.0, 27.0, 47.0, false, 2.0, border);
        }
    }

    fn draw_cell<P: Deref<Target = Pokemon>, M, I, G>(
        &self,
        ctx: &mut Context,
        index: usize,
        pokemon: &OwnablePokemon<P, M, I, G, Health>,
        cell: &PartyCell,
        selected: bool,
    ) {
        let offset = -14.0 + (24.0 * index as f32);
        let mut skip = false;
        if self.select.is_world.get().is_some() {
            if let Some(selected_index) = self.selected.get() {
                let selected_index = selected_index == index;
                if selected_index || selected {
                    self.draw_cell_color(
                        ctx,
                        offset,
                        Self::SELECT_LIGHT,
                        Self::SELECT_DARK,
                        Some(if selected {
                            Self::HOVER_BORDER
                        } else {
                            Self::SELECT_BORDER
                        }),
                    );
                    skip = true;
                }
            }
        }
        if !skip {
            if selected {
                self.draw_cell_color(
                    ctx,
                    offset,
                    Self::HOVER_LIGHT,
                    Self::HOVER_DARK,
                    Some(Self::HOVER_BORDER),
                );
            } else {
                self.draw_cell_color(ctx, offset, Self::LIGHT, Self::DARK, None);
            }
        }
        self.draw_ball(ctx, 88.0, offset - 1.0, selected);
        if let Some(icon) = cellref(&cell.icon) {
            self.draw_pokemon(ctx, icon, 87.0, offset - 8.0, selected);
        }
        draw_text_left(
            ctx,
            &0,
            pokemon.name(),
            119.0,
            offset,
            DrawParams::color(MessagePage::WHITE),
        );
        draw_text_left(
            ctx,
            &0,
            LEVEL_PREFIX,
            129.0,
            offset + 9.0,
            DrawParams::color(MessagePage::WHITE),
        );
        draw_text_left(
            ctx,
            &0,
            cell.level.get(),
            139.0,
            offset + 9.0,
            DrawParams::color(MessagePage::WHITE),
        );
        self.draw_health(ctx, cell, 170.0, offset + 6.0);
    }

    fn draw_cell_color(
        &self,
        ctx: &mut Context,
        y: f32,
        light: Color,
        dark: Color,
        border: Option<Color>,
    ) {
        // 89 + 11
        draw_rectangle(ctx, 98.0, y + 2.0, 138.0, 12.0, dark);
        let y1 = y + 14.5;
        draw_straight_line(ctx, 98.0, y1, 138.0, true, 1.0, light);
        let y1 = y1 + 1.0;
        draw_straight_line(ctx, 98.0, y1, 138.0, true, 1.0, dark);
        draw_rectangle(ctx, 98.0, y + 16.0, 138.0, 4.0, light);
        if let Some(border) = border {
            let y1 = y + 1.0;
            const XLEN: f32 = 140.0;
            const YLEN: f32 = 20.0;
            draw_straight_line(ctx, 97.0, y1, XLEN, true, 2.0, border);
            draw_straight_line(ctx, 97.0, y1 + YLEN, XLEN, true, 2.0, border);
            draw_straight_line(ctx, 237.0, y1, YLEN, false, 2.0, border);
        }
    }

    fn draw_ball(&self, ctx: &mut Context, x: f32, y: f32, selected: bool) {
        self.ball.draw(
            ctx,
            x,
            y,
            DrawParams::source(Rectangle::new(
                0.0,
                if selected { 24.0 } else { 0.0 },
                20.0,
                24.0,
            )),
        );
    }

    fn draw_pokemon(&self, ctx: &mut Context, icon: &Texture, x: f32, y: f32, selected: bool) {
        let second = self.accumulator.get() > PartyCell::ICON_TICK;
        icon.draw(
            ctx,
            x - 3.0,
            if second && selected { y - 5.0 } else { y },
            DrawParams {
                source: Some(Rectangle::new(
                    0.0,
                    if second { 32.0 } else { 0.0 },
                    32.0,
                    32.0,
                )),
                ..Default::default()
            },
        );
    }

    fn draw_health(&self, ctx: &mut Context, cell: &PartyCell, x: f32, y: f32) {
        self.health
            .draw_width(ctx, Vec2::new(x, y), cell.health.percent.get());
        draw_text_right(
            ctx,
            &0,
            cell.health.current.get(),
            x + 25.0,
            y + 5.0,
            DrawParams::color(MessagePage::WHITE),
        );
        draw_text_left(
            ctx,
            &0,
            "/",
            x + 35.0,
            y + 5.0,
            DrawParams::color(MessagePage::WHITE),
        );
        draw_text_left(
            ctx,
            &0,
            cell.health.maximum.get(),
            x + 40.0,
            y + 5.0,
            DrawParams::color(MessagePage::WHITE),
        );
    }

    pub fn take_selected(&self) -> Option<usize> {
        let selected = self.selected.get();
        if selected.is_some() {
            self.selected.set(None);
            selected
        } else {
            None
        }
    }

    pub fn despawn(&self) {
        self.alive.set(false);
        self.select.alive.set(false);
    }

    pub fn alive(&self) -> bool {
        self.alive.get()
    }

    pub fn reset(&self) {
        self.cursor.set(0);
        self.right_cursor.set(None);
        self.accumulator.set(0.0);
        self.selected.set(None);
        self.pokemon.iter().for_each(PartyCell::clear);
    }
}
