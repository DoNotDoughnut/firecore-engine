use macroquad::audio::PlaySoundParams;

use crate::{
    audio::{error::PlayAudioError, MusicId},
    Context,
};

pub fn play_music(ctx: &mut Context, music: &MusicId) -> Result<(), PlayAudioError> {
    if let Some((_, instance)) = ctx.audio.current_music.take() {
        macroquad::audio::stop_sound(instance);
    }
    match ctx.audio.music.get_mut(music) {
        Some(audio) => {
            let audio = *audio;
            macroquad::audio::play_sound(audio, PlaySoundParams {
                looped: true,
                volume: 0.5,
            });
            ctx.audio.current_music = Some((*music, audio));
            Ok(())
            // match audio.play(ctx) {
            // Ok(instance) => {
            //     instance.set_repeating(true);
            //     instance.set_volume(0.3);
            //     ctx.audio.current_music = Some((music, instance));
            //     Ok(())
            // }
            // Err(err) => Err(PlayAudioError::TetraError(err)),
        },
        None => Err(PlayAudioError::Missing),
    }
}
