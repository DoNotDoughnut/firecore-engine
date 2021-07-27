use crate::EngineContext;

pub use super::audio::music::*;

use super::error::PlayAudioError;

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn play_music(ctx: &mut EngineContext, music: MusicId) -> Result<(), PlayAudioError> {
    #[cfg(feature = "audio")] {
        super::backend::music::play_music(ctx, music)
    }
    #[cfg(not(feature = "audio"))] {
        Ok(())
    }
}

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn get_current_music(ctx: &EngineContext) -> Option<MusicId> {
    #[cfg(feature = "audio")] {
        ctx.audio.current_music.as_ref().map(|(id, _)| *id)
    }
    #[cfg(not(feature = "audio"))] {
        None
    }
}

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn get_music_id(ctx: &EngineContext, name: &str) -> Option<MusicId> {
    #[cfg(feature = "audio")] {
        ctx.audio.music_id.get(name).as_deref().copied()
    }
    #[cfg(not(feature = "audio"))] {
        None
    }
}

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn play_music_named(ctx: &mut EngineContext, name: &str) -> Result<(), PlayAudioError> {
    #[cfg(feature = "audio")] {
        match get_music_id(ctx, &name.to_string()) {
            Some(music) => play_music(ctx, music),
            None => Err(PlayAudioError::Missing),
        }
    }
    #[cfg(not(feature = "audio"))] {
        Ok(())
    }
}