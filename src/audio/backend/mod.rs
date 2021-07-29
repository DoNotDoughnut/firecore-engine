pub mod music;
pub mod sound;

use crate::{
    audio::{
        serialized::{SerializedMusicData, SerializedSoundData},
        sound::Sound,
        music::{MusicId, MusicName},
    },
    context::audio::GameAudioMap,
};

use tetra::audio::Sound as Audio;

pub fn add_music(music: &GameAudioMap<MusicId>, ids: &GameAudioMap<MusicName, MusicId>, music_data: SerializedMusicData) {
    add(music, music_data.music.track, &music_data.bytes);
    ids.insert(music_data.music.name, music_data.music.track);
}

pub fn add_sound(sounds: &GameAudioMap<Sound>, sound_data: SerializedSoundData) {
    add(sounds, sound_data.sound, &sound_data.bytes)
}

fn add<K: Eq + std::hash::Hash>(map: &GameAudioMap<K>, k: K, bytes: &[u8]) {
    let sound = Audio::from_file_data(bytes);
    map.insert(k, sound);
}
