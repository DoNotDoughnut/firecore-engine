use macroquad::audio::PlaySoundParams;

use crate::{
    audio::{error::PlayAudioError, MusicId},
    Context,
};

use super::{add, GameAudioMap};

pub fn add_music(
    music: &mut GameAudioMap<MusicId>,
    id: MusicId,
    data: Vec<u8>,
) -> Result<(), macroquad::prelude::FileError> {
    add(music, id, &data)
}

pub fn play_music(ctx: &mut Context, music: &MusicId) -> Result<(), PlayAudioError> {
    stop_music(ctx);
    match ctx.audio.music.get_mut(music) {
        Some(audio) => {
            let audio = *audio;
            macroquad::audio::play_sound(
                audio,
                PlaySoundParams {
                    looped: true,
                    volume: 0.5,
                },
            );
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
        }
        None => Err(PlayAudioError::Missing),
    }
}

pub fn stop_music(ctx: &mut Context) {
    if let Some((_, instance)) = ctx.audio.current_music.take() {
        macroquad::audio::stop_sound(instance);
    }
}
