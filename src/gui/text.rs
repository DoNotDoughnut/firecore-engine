use crate::{
    font::FontId,
    graphics::{draw_button, draw_text_left},
    input::{pressed, Control},
    util::{Completable, Entity, Reset},
    Context,
};

use tetra::math::Vec2;
use text::{Message, MessagePage, MessagePages, TextColor};

#[derive(Default, Clone)]
pub struct MessageBox {
    alive: bool,
    origin: Vec2<f32>,

    pub font: FontId,
    pub message: Message,

    page: usize,
    line: usize,

    accumulator: f32,

    timer: Timer,
    button: Button,

    finished: bool,
}

#[derive(Default, Clone, Copy)]
struct Timer {
    length: f32,
    accumulator: f32,
    alive: bool,
}

#[derive(Default, Clone, Copy)]
struct Button {
    position: f32,
    direction: bool,
}

impl MessageBox {
    pub fn new(origin: Vec2<f32>, font: FontId) -> Self {
        Self {
            alive: false,

            origin,

            font,
            message: Default::default(),

            page: 0,
            line: 0,

            accumulator: 0.0,

            // can_continue: false,
            finished: false,
            timer: Default::default(),

            button: Default::default(),
        }
    }

    pub fn set(&mut self, pages: MessagePages) {
        self.message.pages = pages;
    }

    pub fn push(&mut self, page: MessagePage) {
        self.message.pages.push(page);
    }

    pub fn remove(&mut self, index: usize) {
        self.message.pages.remove(index);
    }

    pub fn clear(&mut self) {
        self.message.pages.clear();
    }

    pub fn color(&mut self, color: TextColor) {
        self.message.color = color;
    }

    pub fn len(&self) -> usize {
        self.message.pages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn page(&self) -> usize {
        self.page
    }

    fn reset_page(&mut self) {
        self.line = 0;
        self.accumulator = 0.0;
    }

    pub fn update(&mut self, ctx: &Context, delta: f32) {
        if self.alive {
            if let Some(page) = self.message.pages.get(self.page) {
                if (self.accumulator as usize) < page.lines[self.line].len() {
                    self.accumulator += delta * 30.0;
                } else if self.line < page.lines.len() - 1 {
                    self.line += 1;
                    self.accumulator = 0.0;
                } else {
                    match page.wait {
                        Some(wait) => match self.timer.alive {
                            false => {
                                self.timer.accumulator = 0.0;
                                self.timer.length = wait;
                                self.timer.alive = true;
                            }
                            true => {
                                self.timer.accumulator += delta;
                                if self.timer.accumulator >= self.timer.length {
                                    self.timer.alive = false;
                                    match self.page + 1 >= self.len() {
                                        true => self.finished = true,
                                        false => {
                                            self.page += 1;
                                            self.reset_page();
                                        }
                                    }
                                }
                            }
                        },
                        None => {
                            self.button.position += match self.button.direction {
                                true => delta,
                                false => -delta,
                            } * 7.5;

                            if self.button.position.abs() > 3.0 {
                                self.button.direction = !self.button.direction;
                            }

                            if pressed(ctx, Control::A) {
                                match self.page + 1 >= self.len() {
                                    true => self.finished = true,
                                    false => {
                                        self.page += 1;
                                        self.reset_page();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        if self.alive {
            if let Some(page) = self.message.pages.get(self.page) {
                if let Some(line) = page.lines.get(self.line) {
                    let len = self.accumulator as usize;
                    let (string, finished) = if line.len() > len {
                        (&line[..len], false)
                    } else {
                        (line.as_str(), true)
                    };

                    let y = (self.line << 4) as f32;
                    draw_text_left(
                        ctx,
                        &self.font,
                        string,
                        self.message.color,
                        self.origin.x,
                        self.origin.y + y,
                    );

                    for index in 0..self.line {
                        draw_text_left(
                            ctx,
                            &self.font,
                            &page.lines[index],
                            self.message.color,
                            self.origin.x,
                            self.origin.y + (index << 4) as f32,
                        );
                    }

                    if finished && page.wait.is_none() {
                        draw_button(
                            ctx,
                            &self.font,
                            line,
                            self.origin.x,
                            self.origin.y + 2.0 + self.button.position + y,
                        );
                    }
                }
            }
        }
    }
}

impl Reset for MessageBox {
    fn reset(&mut self) {
        self.page = 0;
        self.reset_page();
        self.button = Default::default();
        self.finished = false;
    }
}

impl Completable for MessageBox {
    fn finished(&self) -> bool {
        (!(self.page < self.len()) && self.finished) || self.is_empty()
    }
}

impl Entity for MessageBox {
    fn spawn(&mut self) {
        self.alive = true;
        self.reset();
    }

    fn despawn(&mut self) {
        self.alive = false;
        self.reset();
        self.clear();
    }

    fn alive(&self) -> bool {
        self.alive
    }
}
