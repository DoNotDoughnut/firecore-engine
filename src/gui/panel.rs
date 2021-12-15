use crate::{
    graphics::{draw_cursor, draw_text_left, Color, DrawParams},
    math::{vec2, Rectangle},
    Context,
};

pub struct Panel;

impl Panel {
    pub const BACKGROUND: Color = Color::rgb(248.0 / 255.0, 248.0 / 255.0, 248.0 / 255.0);

    pub fn draw(ctx: &mut Context, x: f32, y: f32, w: f32, h: f32) {
        Self::draw_color(ctx, x, y, w, h, Color::WHITE)
    }

    pub fn draw_color(ctx: &mut Context, x: f32, y: f32, w: f32, h: f32, color: Color) {
        const TEXTURE_SIZE: f32 = 7.0;

        let panel = &ctx.panel;

        panel.crate_draw(x, y, DrawParams::color(color));
        let x1 = x + w - TEXTURE_SIZE;
        panel.crate_draw(
            x1,
            y,
            DrawParams {
                color,
                flip_x: true,
                ..Default::default()
            },
        );

        let y1 = y + h - TEXTURE_SIZE;
        panel.crate_draw(
            x,
            y1,
            DrawParams {
                color,
                flip_y: true,
                ..Default::default()
            },
        );

        panel.crate_draw(
            x1,
            y1,
            DrawParams {
                color,
                flip_x: true,
                flip_y: true,
                ..Default::default()
            },
        );

        let w = w - 14.0;
        let h = h - 14.0;

        crate::graphics::draw_rectangle(ctx, x + TEXTURE_SIZE, y + TEXTURE_SIZE, w, h, color);

        let panel = &ctx.panel;

        panel.crate_draw(
            x + TEXTURE_SIZE,
            y,
            DrawParams {
                source: Some(Rectangle::new(6.0, 0.0, 1.0, TEXTURE_SIZE)),
                dest_size: Some(vec2(w, panel.height())),
                color,
                ..DrawParams::default()
            },
        );
        panel.crate_draw(
            x + TEXTURE_SIZE,
            y1,
            DrawParams {
                source: Some(Rectangle::new(6.0, 0.0, 1.0, TEXTURE_SIZE)),
                dest_size: Some(vec2(w, panel.height())),
                flip_y: true,
                color,
                ..Default::default()
            },
        );

        panel.crate_draw(
            x,
            y + TEXTURE_SIZE,
            DrawParams {
                source: Some(Rectangle::new(0.0, 6.0, TEXTURE_SIZE, 1.0)),
                dest_size: Some(vec2(panel.width(), h)),
                color,
                ..Default::default()
            },
        );

        panel.crate_draw(
            x1,
            y + TEXTURE_SIZE,
            DrawParams {
                source: Some(Rectangle::new(0.0, 6.0, TEXTURE_SIZE, 1.0)),
                dest_size: Some(vec2(panel.width(), h)),
                flip_x: true,
                color,
                ..Default::default()
            },
        );
    }

    pub fn draw_text(
        ctx: &mut Context,
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
        Self::draw(ctx, x, y, w, h);
        let tx = x + 15.0;
        let ty = y + 11.0;
        for (index, text) in text.iter().enumerate() {
            draw_text_left(
                ctx,
                &1,
                text,
                tx,
                ty + (index << 4) as f32,
                DrawParams::color(Color::BLACK),
            );
        }
        if add_cancel {
            draw_text_left(
                ctx,
                &1,
                "Cancel",
                tx,
                ty + (text.len() << 4) as f32,
                DrawParams::color(Color::BLACK),
            );
        }
        draw_cursor(
            ctx,
            x + 8.0,
            y + 13.0 + (cursor << 4) as f32,
            Default::default(),
        );
    }

    // pub fn draw_text_with_columns(&self, x: f32, y: f32, w: f32, h: f32) {

    // }
}
