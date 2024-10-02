use crate::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum MenuState {
    Bevy,
    DreamLake,
    Title,
}
impl_core_computed_state!(Menu, MenuState);
impl_kind_computed_state!(MenuState, Bevy, DreamLake, Title);
