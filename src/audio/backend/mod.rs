pub mod music;
pub mod sound;

use crate::context::audio::GameAudioMap;

use firecore_audio::{MusicId, SoundId, SoundVariant};

pub async fn add_music(
    music: &mut GameAudioMap<MusicId>,
    id: MusicId,
    data: Vec<u8>,
) -> Result<(), macroquad::prelude::FileError> {
    add(music, id, &data).await
}

pub async fn add_sound(
    sounds: &mut GameAudioMap<(SoundId, SoundVariant)>,
    id: SoundId,
    variant: SoundVariant,
    data: Vec<u8>,
) -> Result<(), macroquad::prelude::FileError> {
    add(sounds, (id, variant), &data).await
}

async fn add<K: Eq + std::hash::Hash>(
    map: &mut GameAudioMap<K>,
    k: K,
    data: &[u8],
) -> Result<(), macroquad::prelude::FileError> {
    let audio = macroquad::audio::load_sound_from_bytes(data).await?;
    map.insert(k, audio);
    Ok(())
}

// pub struct Audio(pub(crate) macroquad::audio::Sound);

// impl Audio {
//     pub async fn crate_new(data: &[u8]) -> Result<Self, macroquad::file::FileError> {
//         macroquad::audio::load_sound_from_bytes(data)
//             .await
//             .map(Self)
//     }

//     pub async fn new() {}
// }
