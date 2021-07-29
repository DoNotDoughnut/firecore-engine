#[cfg(feature = "audio")]
use {
    dashmap::DashMap,
    firecore_audio::{
        music::{MusicId, MusicName},
        sound::Sound,
    },
    std::sync::Arc,
    tetra::audio::{Sound as Audio, SoundInstance as AudioInstance},
};

use std::ops::{Deref, DerefMut};

use tetra::{Context, graphics::Texture};

use crate::{
    font::SerializedFonts,
    graphics::{byte_texture, TextRenderer},
    input::{
        controller::{default_button_map, ButtonMap},
        keyboard::{default_key_map, KeyMap},
    },
};

pub struct EngineContext {
    pub tetra: Context,
    pub text_renderer: TextRenderer,
    pub controls: GameControls,
    #[cfg(feature = "audio")]
    pub audio: GameAudio,

    pub(crate) white: Texture,
    pub(crate) panel: Texture,
}

impl Deref for EngineContext {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.tetra
    }
}

impl DerefMut for EngineContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tetra
    }
}

pub struct GameControls {
    pub keyboard: KeyMap,
    pub controller: ButtonMap,
}

#[cfg(feature = "audio")]
pub type GameAudioMap<K, V = Audio> = Arc<DashMap<K, V>>;

#[cfg(feature = "audio")]
#[derive(Default)]
pub struct GameAudio {
    pub music_id: GameAudioMap<MusicName, MusicId>,
    pub music: GameAudioMap<MusicId>, // To - do: looping to specific points
    pub current_music: Option<(MusicId, AudioInstance)>,
    pub sound: GameAudioMap<Sound>,
}

impl EngineContext {
    pub fn new(mut ctx: Context, fonts: SerializedFonts) -> tetra::Result<Self> {
        Ok(Self {
            text_renderer: TextRenderer::new(&mut ctx, fonts)?,
            controls: GameControls {
                keyboard: default_key_map(),
                controller: default_button_map(),
            },
            white: byte_texture(
                &mut ctx,
                &[
                    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49,
                    0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02,
                    0x00, 0x00, 0x00, 0x90, 0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x01, 0x73, 0x52,
                    0x47, 0x42, 0x00, 0xAE, 0xCE, 0x1C, 0xE9, 0x00, 0x00, 0x00, 0x04, 0x67, 0x41,
                    0x4D, 0x41, 0x00, 0x00, 0xB1, 0x8F, 0x0B, 0xFC, 0x61, 0x05, 0x00, 0x00, 0x00,
                    0x09, 0x70, 0x48, 0x59, 0x73, 0x00, 0x00, 0x0E, 0xC3, 0x00, 0x00, 0x0E, 0xC3,
                    0x01, 0xC7, 0x6F, 0xA8, 0x64, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54,
                    0x18, 0x57, 0x63, 0xF8, 0xFF, 0xFF, 0x3F, 0x00, 0x05, 0xFE, 0x02, 0xFE, 0xA7,
                    0x35, 0x81, 0x84, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42,
                    0x60, 0x82,
                ],
            ),
            panel: byte_texture(&mut ctx, include_bytes!("../assets/panel.png")),
            #[cfg(feature = "audio")]
            audio: Default::default(),
            tetra: ctx,
        })
    }

    #[cfg(feature = "audio")]
    pub fn audio(&self, audio_data: crate::audio::serialized::SerializedAudio) {
        for music_data in audio_data.music {
            let music = self.audio.music.clone();
            let ids = self.audio.music_id.clone();
            std::thread::spawn(move || crate::audio::add_music(&music, &ids, music_data));
        }
        for sound_data in audio_data.sounds {
            let sounds = self.audio.sound.clone();
            std::thread::spawn(move || crate::audio::add_sound(&sounds, sound_data));
        }
    }
}
