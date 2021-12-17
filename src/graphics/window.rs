use crate::Context;

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
    // /// The entire screen will be displayed as large as possible while maintaining the original
    // /// aspect ratio. Letterboxing may occur.
    // ShowAll,

    // /// Works the same as ShowAll, but will only scale by integer values.
    // ShowAllPixelPerfect,

    // /// The screen will fill the entire window, maintaining the original aspect ratio but
    // /// potentially being cropped.
    // Crop,

    // /// Works the same as Crop, but will only scale by integer values.
    // CropPixelPerfect,
}

impl Default for ScalingMode {
    fn default() -> Self {
        Self::Fixed
    }
}

#[allow(unused_variables)]
pub fn set_scaling_mode(ctx: &mut Context, mode: ScalingMode) {
    use macroquad::prelude::*;
    match mode {
        ScalingMode::Fixed => set_default_camera(),
        ScalingMode::Stretch => set_camera(&Camera2D::from_display_rect(Rect::new(
            0.0,
            0.0,
            width(ctx),
            height(ctx),
        ))),
    }
    ctx.scaling = mode;
}

#[allow(unused_variables)]
pub fn width(ctx: &Context) -> f32 {
    macroquad::prelude::screen_width()
}

#[allow(unused_variables)]
pub fn height(ctx: &Context) -> f32 {
    macroquad::prelude::screen_height()
}
