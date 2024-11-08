use crate::prelude::*;

pub mod convo_box;
pub mod convo_speaker;
pub mod convo_text;

pub use convo_box::*;
pub use convo_speaker::*;
pub use convo_text::*;

pub trait ConvoKind {
    fn get_convo_boxes(&self) -> Vec<ConvoBox>;
}

pub(super) struct ConvoPlugin;
impl Plugin for ConvoPlugin {
    fn build(&self, app: &mut App) {}
}
