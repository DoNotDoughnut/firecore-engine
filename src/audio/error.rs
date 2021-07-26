use std::error::Error;
use core::fmt::Display;

#[derive(Debug)]
pub enum PlayAudioError {
    #[cfg(feature = "audio")]
    Missing,
    #[cfg(feature = "audio")]
    TetraError(tetra::TetraError),
}

impl Error for PlayAudioError {}

impl Display for PlayAudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "audio")] {
            match self {
                Self::Missing => write!(f, "Could not find music with specified id!"),
                Self::TetraError(err) => err.fmt(f),
            }
        }
        #[cfg(not(feature = "audio"))] {
            write!(f, "Audio is disabled by feature.")
        }
    }
}