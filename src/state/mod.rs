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
pub enum PauseState {
    Paused,
    Unpaused,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect, States)]
pub enum PlayerExistsState {
    Exists,
    DoesNotExist,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Reflect, States)]
pub struct TransitionState {
    pub exiting: Option<MetaState>,
    pub entering: Option<MetaState>,
}
impl TransitionState {
    pub fn is_active(&self) -> bool {
        match (&self.entering, &self.exiting) {
            (Some(_), Some(_)) => true,
            _ => false,
        }
    }
}

pub(super) struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        debug_resource!(app, State<MetaState>);
        debug_resource!(app, State<MetaStateKind>);

        app.insert_state(MetaState::Menu(MenuState::Bevy));
        // app.insert_state(MetaState::Menu(MenuState::Savefile));
        app.insert_state(TransitionState::default());
        app.insert_state(PauseState::Unpaused);

        app.add_computed_state::<MetaStateKind>();
        app.add_computed_state::<PhysicsState>();
        app.add_computed_state::<TransitionActiveState>();
        app.add_computed_state::<MenuState>();
        app.add_computed_state::<MenuStateKind>();
        app.add_computed_state::<WorldLoadingState>();
        app.add_computed_state::<WorldState>();
        app.add_computed_state::<LevelState>();
        app.add_computed_state::<PlayerMetaState>();
    }
}
