use crate::{
    audio::{error::PlayAudioError, music::MusicId},
    Context,
};

pub fn play_music(ctx: &mut Context, music: MusicId) -> Result<(), PlayAudioError> {
    if let Some((_, instance)) = ctx.game.audio.current_music.take() {
        instance.stop();
    }
    match ctx.game.audio.music.get_mut(&music) {
        Some(audio) => match audio.play(ctx) {
            Ok(instance) => {
                instance.set_repeating(true);
                instance.set_volume(0.3);
                ctx.game.audio.current_music = Some((music, instance));
                Ok(())
            }
            Err(err) => Err(PlayAudioError::TetraError(err)),
        },
        None => Err(PlayAudioError::Missing),
    }
}
