#[cfg(feature = "audio")]
mod tetra;

use std::error::Error;

#[cfg(feature = "audio")]
pub use self::tetra::*;

#[cfg(not(feature = "audio"))]
mod dummy;

#[cfg(not(feature = "audio"))]
pub use dummy::*;

// To - do: this 

use firecore_audio::{serialized::SerializedAudio, sound::Sound};

pub trait AudioBackend {

    type AddError: Error;
    type PlayError: Error;

    /// Initialize the audio backend or panic.
    fn init(&mut self);

    fn load(&mut self, data: SerializedAudio);

    fn add_music(&mut self) -> Result<(), Self::AddError>;

    fn add_sound(&mut self) -> Result<(), Self::AddError>;

    fn try_play_music(&mut self) -> Result<(), Self::PlayError>;

    fn play_music(&mut self);

    fn try_play_sound(&mut self, sound: &Sound) -> Result<(), Self::PlayError>;

    fn play_sound(&mut self, sound: &Sound);

}