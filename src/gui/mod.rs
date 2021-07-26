mod panel;
mod text;
mod bar;

pub use self::{
    panel::*, 
    text::*,
    bar::*,
};

// pub struct StaticList<D, const SIZE: usize> {
//     pub options: [D; SIZE],
//     pub cursor: usize,
// }

// pub struct MultiStaticList<D: Array> {
//     pub options: 
//     pub cursor: usize,
// }