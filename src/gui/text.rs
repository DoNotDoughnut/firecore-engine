use crate::{
    font::FontId,
    graphics::{draw_button, draw_text_left},
    input::{pressed, Control},
    util::{Completable, Entity, Reset},
    EngineContext,
};

use tetra::math::Vec2;
use text::{Message, MessagePage, MessagePages, TextColor};

#[derive(Default, Clone)]
pub struct MessageBox {
    alive: bool,
    origin: Vec2<f32>,

    pub font: FontId,
    pub message: Message,

    button: Button,

    page: usize,
    line: usize,
    accumulator: f32,

    waiting: bool,
    finished: bool,
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
            button: Default::default(),
            page: 0,
            line: 0,
            accumulator: 0.0,
            waiting: false,
            finished: false,
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

    pub fn is_empty(&self) -> bool {
        self.pages() == 0
    }

    pub fn page(&self) -> usize {
        self.page
    }

    pub fn pages(&self) -> usize {
        self.message.pages.len()
    }

    pub fn waiting(&self) -> bool {
        self.waiting
    }

    fn reset_page(&mut self) {
        self.line = 0;
        self.accumulator = 0.0;
    }

    pub fn update(&mut self, ctx: &EngineContext, delta: f32) {
        if self.alive {
            match self.message.pages.get(self.page) {
                Some(page) => match self.waiting {
                    false => {
                        if (self.accumulator as usize)
                            < page
                                .lines
                                .get(self.line)
                                .map(String::len)
                                .unwrap_or_default()
                        {
                            self.accumulator += delta * 30.0;
                        } else {
                            self.accumulator = 0.0;
                            if self.line < page.lines.len() - 1 {
                                self.line += 1;
                            } else {
                                self.waiting = true;
                            }
                        }
                    }
                    true => match page.wait {
                        Some(wait) => {
                            self.accumulator += delta;
                            if self.accumulator.abs() >= wait.abs() {
                                self.finish_waiting();
                            }
                        }
                        None => match pressed(ctx, Control::A) {
                            true => self.finish_waiting(),
                            false => {
                                self.button.position += match self.button.direction {
                                    true => delta,
                                    false => -delta,
                                } * 7.5;

                                if self.button.position.abs() > 3.0 {
                                    self.button.direction = !self.button.direction;
                                }
                            }
                        },
                    },
                },
                None => self.finished = true,
            }
        }
    }

    fn finish_waiting(&mut self) {
        self.waiting = false;
        match self.page < self.pages() {
            false => self.finished = true,
            true => {
                self.page += 1;
                self.reset_page();
            }
        }
    }

    pub fn draw(&self, ctx: &mut EngineContext) {
        if self.alive {
            if let Some(page) = self.message.pages.get(self.page) {
                if let Some(line) = page.lines.get(self.line) {
                    let len = self.accumulator as usize;
                    let (string, finished) = if line.len() > len && !self.waiting {
                        (&line[..len], false)
                    } else {
                        (line.as_str(), self.line + 1 >= page.lines.len())
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
        self.finished = false;
        self.button = Default::default();
    }
}

impl Completable for MessageBox {
    fn finished(&self) -> bool {
        self.finished || self.is_empty()
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
