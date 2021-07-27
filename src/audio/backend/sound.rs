use crate::EngineContext;
use crate::audio::sound::Sound;

use crate::audio::error::PlayAudioError;

pub fn play_sound(ctx: &EngineContext, sound: &Sound) -> Result<(), PlayAudioError> {
    match ctx.audio.sound.get(sound) {
        Some(handle) => {
            match handle.play(ctx) {
                Ok(instance) => {
                    instance.set_volume(0.3);
                    Ok(())
                }
                Err(err) => {
                    Err(PlayAudioError::TetraError(err))
                }
            }
        }
        None => {
            Err(PlayAudioError::Missing)
        }
    }
    
}