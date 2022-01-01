use crate::{error::FileError, Context};

pub use macroquad::audio::PlaySoundParams;

#[derive(Debug, Clone)]
pub struct Sound(macroquad::audio::Sound);

impl Sound {
    pub fn new(bytes: &[u8]) -> Result<Self, FileError> {
        macroquad::audio::load_sound_from_bytes(&bytes)
            .map(Sound)
            .map_err(Into::into)
    }
}

#[derive(Debug)]
pub struct SoundHandle(macroquad::audio::Sound);

#[allow(unused_variables)]
pub fn play_sound(ctx: &mut Context, sound: &Sound, params: PlaySoundParams) -> SoundHandle {
    macroquad::audio::play_sound(sound.0, params);
    SoundHandle(sound.0)
}

#[allow(unused_variables)]
pub fn stop_sound(ctx: &mut Context, sound: SoundHandle) {
    macroquad::audio::stop_sound(sound.0);
}
