use firecore_audio::SoundId;

use crate::Context;
use super::error::PlayAudioError;

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn play_sound(ctx: &Context, sound: &SoundId, variant: Option<u16>) -> Result<(), PlayAudioError> {
    #[cfg(feature = "audio")] {
        super::backend::sound::play_sound(ctx, sound, variant)
    }
    #[cfg(not(feature = "audio"))] {
        Ok(())
    }
}