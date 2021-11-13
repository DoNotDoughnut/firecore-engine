use macroquad::{camera::{Camera2D, set_camera, set_default_camera}, prelude::{screen_height, screen_width}};

use crate::{Context, math::{Rectangle, vec2}};

pub fn set_scaler(ctx: &mut Context, scaler: ScreenScaler) {
    ctx.scaler = Some(scaler);
}

pub fn get_scaler(ctx: &mut Context) -> Option<&mut ScreenScaler> {
    ctx.scaler.as_mut()
}

pub struct ScreenScaler {
    width: i32,
    height: i32,
    mode: ScalingMode,
}

impl ScreenScaler {

    pub fn with_size(
        _: &mut Context,
        width: i32,
        height: i32,
        mode: ScalingMode,
    ) -> Self {
        Self {
            width,
            height,
            mode,
        }
    }

    pub(crate) fn update(&mut self) {
        match &self.mode {
            ScalingMode::Fixed => set_default_camera(),
            ScalingMode::Stretch => set_camera(&Camera2D::from_display_rect(Rectangle::new(0.0, 0.0, self.width as _, self.height as _))),
            ScalingMode::ShowAll => {

                let inner_ratio = self.width as f32 / self.height as f32;
                let screen_ratio = screen_width() as f32 / screen_height() as f32;

                // let s
            },
            ScalingMode::ShowAllPixelPerfect => {

                self.mode = ScalingMode::Stretch;
                self.update();

                let (inner_w, inner_h) = (self.width, self.height);

                let (outer_w, outer_h) = (screen_width(), screen_height());

                let inner_ratio = inner_w as f32 / inner_h as f32;

                let outer_ratio = outer_w / outer_h;

                let mut scale_factor = if inner_ratio > outer_ratio {
                    outer_w as i32 / inner_w
                } else {
                    outer_h as i32 / inner_h 
                };

                if scale_factor == 0 {
                    scale_factor = 1;
                }

                let screen_width = inner_w * scale_factor;
                let screen_height = inner_h * scale_factor;
                let screen_x = (outer_w as i32 - screen_width) / 2;
                let screen_y = (outer_h as i32 - screen_height) / 2;

                let target = vec2((inner_w / 2) as f32, (inner_h / 2) as f32);

                set_camera(&Camera2D {
                    target,
                    zoom: vec2(1. / screen_width as f32 * 2., -1. / screen_height as f32 * 2.),
                    offset: vec2(screen_x as f32, screen_y as f32),
                    rotation: 0.,

                    render_target: None,
                    viewport: None,
                })

            },
            ScalingMode::Crop => todo!(),
            ScalingMode::CropPixelPerfect => todo!(),
        }
    }

}

/// Algorithms that can be used to scale the game's screen.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ScalingMode {
    /// The game will always be displayed at its native resolution, with no scaling applied.
    /// If the window is bigger than the native resolution, letterboxing will be applied.
    /// If the window is smaller than the native resolution, it will be cropped.
    Fixed,

    /// The screen will be stretched to fill the window, without trying to preserve the original
    /// aspect ratio. Distortion/stretching/squashing may occur.
    Stretch,

    /// The entire screen will be displayed as large as possible while maintaining the original
    /// aspect ratio. Letterboxing may occur.
    ShowAll,

    /// Works the same as ShowAll, but will only scale by integer values.
    ShowAllPixelPerfect,

    /// The screen will fill the entire window, maintaining the original aspect ratio but
    /// potentially being cropped.
    Crop,

    /// Works the same as Crop, but will only scale by integer values.
    CropPixelPerfect,
}