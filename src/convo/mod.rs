use crate::prelude::*;

pub mod convo_box;
pub mod convo_data;
pub mod convo_oneoff;
pub mod convo_speaker;
pub mod convo_text;

pub use convo_box::*;
pub use convo_data::*;
pub use convo_oneoff::*;
pub use convo_speaker::*;
pub use convo_text::*;

pub trait ConvoKind: Queryable {
    fn get_convo_boxes(&self) -> Vec<ConvoBox>;
}

pub(super) struct ConvoPlugin;
impl Plugin for ConvoPlugin {
    fn build(&self, app: &mut App) {
        convo_box::register_convo_box(app);
        convo_data::register_convo_data(app);
        convo_text::register_convo_text(app);
        convo_oneoff::register_convo_oneoff(app);
    }
}
