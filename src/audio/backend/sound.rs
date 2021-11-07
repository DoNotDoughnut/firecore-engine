use crate::Context;

use crate::audio::{error::PlayAudioError, SoundId};

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
