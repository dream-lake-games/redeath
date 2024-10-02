use crate::prelude::*;

pub mod computed;
pub mod menu;
pub mod world;
pub mod world_loading;

pub(crate) use computed::*;
pub use menu::*;
pub use world::*;
pub use world_loading::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect, States)]
pub enum MetaState {
    Menu(MenuState),
    World(WorldState),
    WorldLoading(WorldLoadingState),
}
impl_kind_computed_state!(MetaState, Menu, World, WorldLoading);
pub trait CoreState: Sized {
    fn to_meta_state(self) -> MetaState;

    fn from_meta_state(meta: &MetaState) -> Option<Self>;
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect, States)]
pub struct PauseState;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect, States)]
pub struct PlayerExistsState;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect, States)]
pub struct TransState {
    exiting: Option<MetaState>,
    entering: MetaState,
}

pub(super) struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(MetaState::Menu(MenuState::Bevy));
        debug_resource!(app, State<MetaState>);
    }
}
