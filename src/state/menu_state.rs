use crate::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum MenuState {
    Bevy,
    DreamLake,
    Title,
    Savefile,
    OverworldLoading,
    Overworld(OverworldState),
}
impl_core_computed_state!(Menu, MenuState);
impl_kind_computed_state!(
    MenuState,
    Bevy,
    DreamLake,
    Title,
    Savefile,
    OverworldLoading,
    Overworld
);

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct OverworldState {
    pub kind: WorldKind,
    pub player_meta_state: PlayerMetaState,
}
impl OverworldState {
    pub fn from_world_kind(kind: WorldKind) -> Self {
        Self {
            kind,
            player_meta_state: PlayerMetaState::NoneOk,
        }
    }
}
