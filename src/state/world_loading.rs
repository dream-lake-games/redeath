use crate::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct WorldLoadingState {
    pub kind: WorldKind,
}
impl_core_computed_state!(WorldLoading, WorldLoadingState);
