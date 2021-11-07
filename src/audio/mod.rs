pub mod music;
pub mod sound;

#[cfg(feature = "audio")]
mod backend;
pub mod error;

pub type MusicId = tinystr::TinyStr16;
pub type SoundId = tinystr::TinyStr8;
pub type SoundVariant = Option<u16>;

use crate::Context;

pub fn play_music(ctx: &mut Context, id: &MusicId) {
    if let Err(err) = music::play_music(ctx, id) {
        log::warn!("Could not play music id {} with error {}", id, err);
    }
}

pub fn play_sound(ctx: &Context, sound: &SoundId, variant: Option<u16>) {
    if let Err(err) = sound::play_sound(ctx, sound, variant) {
        log::warn!(
            "Could not play sound {}, variant {:?} with error {}",
            sound,
            variant,
            err
        );
    }
}

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub async fn add_music(ctx: &mut Context, id: MusicId, data: Vec<u8>) {
    #[cfg(feature = "audio")]
    if let Err(err) = backend::add_music(&mut ctx.audio.music, id, data).await {
        log::error!("Cannot add audio with error {}", err)
    }
}

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub async fn add_sound(ctx: &mut Context, id: SoundId, variant: Option<u16>, data: Vec<u8>) {
    #[cfg(feature = "audio")]
    if let Err(err) = backend::add_sound(&mut ctx.audio.sounds, id, variant, data).await {
        log::error!("Cannot add sound with error {}", err);
    }
}
