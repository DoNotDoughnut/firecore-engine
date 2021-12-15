#[cfg(feature = "audio")]
pub(crate) mod backend;

pub mod error;

pub type MusicId = tinystr::TinyStr16;
pub type SoundId = tinystr::TinyStr8;
pub type SoundVariant = Option<u16>;

use crate::Context;

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn play_music(ctx: &mut Context, id: &MusicId) {
    #[cfg(feature = "audio")]
    if let Err(err) = backend::music::play_music(ctx, id) {
        crate::log::warn!("Could not play music id {} with error {}", id, err);
    }
}

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn get_current_music(ctx: &Context) -> Option<&MusicId> {
    #[cfg(feature = "audio")]
    {
        ctx.audio.current_music.as_ref().map(|(id, _)| id)
    }
    #[cfg(not(feature = "audio"))]
    {
        None
    }
}

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn play_sound(ctx: &Context, sound: &SoundId, variant: Option<u16>) {
    #[cfg(feature = "audio")]
    if let Err(err) = backend::sound::play_sound(ctx, sound, variant) {
        crate::log::warn!(
            "Could not play sound {}, variant {:?} with error {}",
            sound,
            variant,
            err
        );
    }
}

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn add_music(ctx: &mut Context, id: MusicId, data: Vec<u8>) {
    #[cfg(feature = "audio")]
    if let Err(err) = backend::music::add_music(&mut ctx.audio.music, id, data) {
        crate::log::error!("Cannot add audio with error {}", err)
    }
}

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn add_sound(ctx: &mut Context, id: SoundId, variant: Option<u16>, data: Vec<u8>) {
    #[cfg(feature = "audio")]
    if let Err(err) = backend::sound::add_sound(&mut ctx.audio.sounds, id, variant, data) {
        crate::log::error!("Cannot add sound with error {}", err);
    }
}
