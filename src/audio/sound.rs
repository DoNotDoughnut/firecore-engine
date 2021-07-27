use crate::EngineContext;
pub use super::audio::sound::*;
use super::error::PlayAudioError;

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn play_sound(ctx: &EngineContext, sound: &Sound) -> Result<(), PlayAudioError> {
    #[cfg(feature = "audio")] {
        super::backend::sound::play_sound(ctx, sound)
    }
    #[cfg(not(feature = "audio"))] {
        Ok(())
    }
}