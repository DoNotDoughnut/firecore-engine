use crate::Context;

pub use super::audio::music::*;

use super::error::PlayAudioError;

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn play_music(ctx: &mut Context, music: MusicId) -> Result<(), PlayAudioError> {
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
        ctx.game.audio.current_music.as_ref().map(|(id, _)| *id)
    }
    #[cfg(not(feature = "audio"))] {
        None
    }
}

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn get_music_id(ctx: &Context, name: &str) -> Option<MusicId> {
    #[cfg(feature = "audio")] {
        ctx.game.audio.music_id.get(name).as_deref().copied()
    }
    #[cfg(not(feature = "audio"))] {
        None
    }
}

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn play_music_named(ctx: &mut Context, name: &str) -> Result<(), PlayAudioError> {
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