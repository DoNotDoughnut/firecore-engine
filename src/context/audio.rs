use dashmap::DashMap;
use firecore_audio::{
    music::{MusicId, MusicName},
    serialized::SerializedAudio,
    sound::Sound,
};
use std::sync::Arc;
use tetra::audio::{Sound as Audio, SoundInstance as AudioInstance};

pub type GameAudioMap<K, V = Audio> = Arc<DashMap<K, V>>;

#[derive(Default)]
pub struct GameAudio {
    pub music_id: GameAudioMap<MusicName, MusicId>,
    pub music: GameAudioMap<MusicId>, // To - do: looping to specific points
    pub current_music: Option<(MusicId, AudioInstance)>,
    pub sound: GameAudioMap<Sound>,
}

impl GameAudio {
    pub fn init(&self, audio_data: SerializedAudio) {
        for music_data in audio_data.music {
            let music = self.music.clone();
            let ids = self.music_id.clone();
            std::thread::spawn(move || crate::audio::add_music(&music, &ids, music_data));
        }
        for sound_data in audio_data.sounds {
            let sounds = self.sound.clone();
            std::thread::spawn(move || crate::audio::add_sound(&sounds, sound_data));
        }
    }
}
