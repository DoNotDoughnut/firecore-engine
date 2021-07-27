use crate::{
    audio::{error::PlayAudioError, music::MusicId},
    EngineContext,
};

pub fn play_music(ctx: &mut EngineContext, music: MusicId) -> Result<(), PlayAudioError> {
    if let Some((_, instance)) = ctx.audio.current_music.take() {
        instance.stop();
    }
    match ctx.audio.music.get_mut(&music) {
        Some(audio) => match audio.play(ctx) {
            Ok(instance) => {
                instance.set_repeating(true);
                instance.set_volume(0.3);
                ctx.audio.current_music = Some((music, instance));
                Ok(())
            }
            Err(err) => Err(PlayAudioError::TetraError(err)),
        },
        None => Err(PlayAudioError::Missing),
    }
}
