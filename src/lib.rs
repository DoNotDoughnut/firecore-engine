pub extern crate tetra;

pub mod util;
pub mod graphics;
pub mod audio;
pub mod input;
pub mod gui;

pub use firecore_font::message as text;

pub fn play_music(ctx: &tetra::Context, id: audio::music::MusicId) {
    if let Err(err) = audio::music::play_music_id(ctx, id) {
        log::warn!("Could not play music id {:x} with error {}", id, err);
    }
}

pub fn play_music_named(ctx: &tetra::Context, music: &str) {
    if let Err(err) = audio::music::play_music_named(ctx, music) {
        log::warn!(
            "Could not play music named \"{}\" with error {}",
            music, err
        );
    }
}

pub fn play_sound(ctx: &tetra::Context, sound: &audio::sound::Sound) {
    if let Err(err) = audio::sound::play_sound(ctx, &sound) {
        log::warn!("Could not play sound {} with error {}", sound, err);
    }
}