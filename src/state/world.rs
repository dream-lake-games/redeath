use crate::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct WorldState {
    pub kind: WorldKind,
    pub level_state: LevelState,
}
impl_core_computed_state!(World, WorldState);

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct LevelState {
    pub iid: String,
    pub lower_left: IVec2,
    pub upper_right: IVec2,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum WorldKind {
    Lake,
}
#[derive(Clone, Debug, Reflect)]
pub struct WorldKindData {
    pub ldtk_path: String,
}
impl WorldKind {
    pub fn to_data(&self) -> WorldKindData {
        match self {
            Self::Lake => WorldKindData {
                ldtk_path: "worlds/lake.ldtk".into(),
            },
        }
    }
}
