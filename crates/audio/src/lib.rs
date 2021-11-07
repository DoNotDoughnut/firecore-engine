use serde::{Deserialize, Serialize};

pub type MusicId = tinystr::TinyStr16;
pub type SoundId = tinystr::TinyStr8;
pub type SoundVariant = Option<u16>;

// #[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
// pub struct MusicData {
//     pub loop_start: Option<f64>,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicData<D> {
    pub id: MusicId,
    pub data: D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundData<D> {
    pub id: SoundId,
    #[serde(default)]
    pub variant: Option<u16>,
    pub data: D,
}