pub extern crate firecore_engine as engine;
pub extern crate firecore_pokedex as pokedex;
// pub use battle::pokedex;
pub use pokedex::*;

// #[deprecated(note = "add battle moves to battle-gui crate")]
// pub mod battle_move;

pub(crate) mod data;
pub mod gui;
pub mod texture;

/// Holds the string "cry"
pub const CRY_ID: tinystr::TinyStr8 = unsafe { tinystr::TinyStr8::new_unchecked(7959107) };

pub use data::PokedexClientData;
pub use firecore_pokedex_engine_builder::{npc_group::NpcGroupId, SerializedPokedexEngine};
