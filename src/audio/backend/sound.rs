use crate::{
    audio::{
        backend::{add, GameAudioMap},
        error::PlayAudioError,
        SoundId, SoundVariant,
    },
    Context,
};

pub fn add_sound(
    sounds: &mut GameAudioMap<(SoundId, SoundVariant)>,
    id: SoundId,
    variant: SoundVariant,
    data: Vec<u8>,
) -> Result<(), macroquad::prelude::FileError> {
    add(sounds, (id, variant), &data)
}

pub fn play_sound(
    ctx: &Context,
    sound: &SoundId,
    variant: Option<u16>,
) -> Result<(), PlayAudioError> {
    match ctx.audio.sounds.get(&(*sound, variant)) {
        Some(handle) => {
            macroquad::audio::play_sound_once(*handle);
            Ok(())
            // match  handle.play(ctx) {
            //     Ok(instance) => {
            //         instance.set_volume(0.3);
            //         Ok(())
            //     }
            //     Err(err) => {
            //         Err(PlayAudioError::TetraError(err))
            //     }
            // }
        }
        None => Err(PlayAudioError::Missing),
    }
}
