use hashbrown::HashMap;
use macroquad::audio::Sound as Audio;

use crate::audio::{MusicId, SoundId, SoundVariant};
use crate::Context;

pub type GameAudioMap<K, V = Audio> = HashMap<K, V>;
pub type SerializedAudio = (HashMap<MusicId, Vec<u8>>, HashMap<(SoundId, SoundVariant), Vec<u8>>);

#[derive(Default)]
pub struct GameAudio {
    pub(crate) music: GameAudioMap<MusicId>,
    pub(crate) current_music: Option<(MusicId, Audio)>,
    pub(crate) sounds: GameAudioMap<(SoundId, SoundVariant)>,
}

impl GameAudio {
    pub async fn init(ctx: &mut Context, data: SerializedAudio) {
        for (id, data) in data.0 {
            crate::audio::add_music(ctx, id, data).await;
        }
        for ((id, variant), data) in data.1 {
            crate::audio::add_sound(ctx, id, variant, data).await;
        }
    }
}
