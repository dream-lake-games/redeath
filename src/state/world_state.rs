use crate::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct WorldState {
    pub kind: WorldKind,
    pub level_state: LevelState,
    pub player_meta_state: PlayerMetaState,
}
impl_core_computed_state!(World, WorldState);

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct LevelState {}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum PlayerMetaState {
    NoneOk,
    Spawning,
    Puppet,
    Playing,
    Dying,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum WorldKind {
    Canyon,
}
#[derive(Clone, Debug, Reflect)]
pub struct WorldKindData {
    pub ldtk_path: String,
}
impl WorldKind {
    pub fn to_data(&self) -> WorldKindData {
        match self {
            Self::Canyon => WorldKindData {
                ldtk_path: "worlds/lake.ldtk".into(),
            },
        }
    }
}
