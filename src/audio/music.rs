use crate::Context;

use super::{error::PlayAudioError, MusicId};

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn play_music(ctx: &mut Context, music: &MusicId) -> Result<(), PlayAudioError> {
    #[cfg(feature = "audio")] {
        super::backend::music::play_music(ctx, music)
    }
    #[cfg(not(feature = "audio"))] {
        Ok(())
    }
}

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn get_current_music(ctx: &Context) -> Option<MusicId> {
    #[cfg(feature = "audio")] {
        ctx.audio.current_music.as_ref().map(|(id, _)| *id)
    }
    #[cfg(not(feature = "audio"))] {
        None
    }
}