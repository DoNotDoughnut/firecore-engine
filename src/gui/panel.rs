use crate::{
    graphics::{draw_cursor, draw_text_left, flip_x, flip_y, position},
    tetra::{
        graphics::{Color, Rectangle},
        math::Vec2,
    },
    EngineContext,
};

use text::TextColor;

pub struct Panel;

impl Panel {
    pub const BACKGROUND: Color = Color::rgb(248.0 / 255.0, 248.0 / 255.0, 248.0 / 255.0);

    pub fn draw(&self, ctx: &mut EngineContext, x: f32, y: f32, w: f32, h: f32) {
        self.draw_color(ctx, x, y, w, h, Color::WHITE)
    }

    pub fn draw_color(&self, ctx_: &mut EngineContext, x: f32, y: f32, w: f32, h: f32, color: Color) {

        let panel = &ctx_.panel;
        let ctx = &mut ctx_.tetra;

        panel.draw(ctx, position(x, y).color(color));
        let x1 = x + w;
        panel.draw(ctx, flip_x(position(x1, y).color(color)));

        let y1 = y + h;
        panel.draw(ctx, flip_y(position(x, y1).color(color)));

        panel.draw(
            ctx,
            position(x1, y1).scale(Vec2::new(-1.0, -1.0)).color(color),
        );

        let w = w - 14.0;
        let h = h - 14.0;

        crate::graphics::draw_rectangle(ctx_, x + 7.0, y + 7.0, w, h, color);

        let panel = &ctx_.panel;
        let ctx = &mut ctx_.tetra;

        panel.draw_region(
            ctx,
            Rectangle::new(6.0, 0.0, 1.0, 7.0),
            position(x + 7.0, y).scale(Vec2::new(w, 1.0)).color(color),
        );
        panel.draw_region(
            ctx,
            Rectangle::new(6.0, 0.0, 1.0, 7.0),
            position(x + 7.0, y1).scale(Vec2::new(w, -1.0)).color(color),
        );

        panel.draw_region(
            ctx,
            Rectangle::new(0.0, 6.0, 7.0, 1.0),
            position(x, y + 7.0).scale(Vec2::new(1.0, h)).color(color),
        );
        panel.draw_region(
            ctx,
            Rectangle::new(0.0, 6.0, 7.0, 1.0),
            position(x1, y + 7.0).scale(Vec2::new(-1.0, h)).color(color),
        );
    }

    pub fn draw_text(
        &self,
        ctx: &mut EngineContext,
        x: f32,
        y: f32,
        w: f32,
        text: &[&str],
        cursor: usize,
        from_bottom: bool,
        add_cancel: bool,
    ) {
        let h = 22.0 + ((text.len() + if add_cancel { 1 } else { 0 }) << 4) as f32;
        let y = if from_bottom { y - h } else { y };
        self.draw(ctx, x, y, w, h);
        let tx = x + 15.0;
        let ty = y + 11.0;
        for (index, text) in text.iter().enumerate() {
            draw_text_left(
                ctx,
                &1,
                text,
                TextColor::Black,
                tx,
                ty + (index << 4) as f32,
            );
        }
        if add_cancel {
            draw_text_left(
                ctx,
                &1,
                "Cancel",
                TextColor::Black,
                tx,
                ty + (text.len() << 4) as f32,
            );
        }
        draw_cursor(ctx, x + 8.0, y + 13.0 + (cursor << 4) as f32);
    }

    // pub fn draw_text_with_columns(&self, x: f32, y: f32, w: f32, h: f32) {

    // }
}
