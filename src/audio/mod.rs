extern crate firecore_audio as audio;

pub mod music;
pub mod sound;

pub mod error;
#[cfg(feature = "audio")]
mod backend;

#[cfg(feature = "audio")]
pub use backend::{add_music, add_sound};

pub use audio::serialized;

use crate::EngineContext;

pub fn play_music(ctx: &mut EngineContext, id: music::MusicId) {
    if let Err(err) = music::play_music(ctx, id) {
        log::warn!("Could not play music id {:x} with error {}", id, err);
    }
}

pub fn play_music_named(ctx: &mut EngineContext, music: &str) {
    if let Err(err) = music::play_music_named(ctx, music) {
        log::warn!(
            "Could not play music named \"{}\" with error {}",
            music, err
        );
    }
}

pub fn play_sound(ctx: &EngineContext, sound: &sound::Sound) {
    if let Err(err) = sound::play_sound(ctx, &sound) {
        log::warn!("Could not play sound {} with error {}", sound, err);
    }
}