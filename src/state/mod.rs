use crate::prelude::*;

pub mod computed;
pub mod cutscene_state;
pub mod menu_state;
pub mod world_loading_state;
pub mod world_state;

pub(crate) use computed::*;
pub use cutscene_state::*;
pub use menu_state::*;
pub use world_loading_state::*;
pub use world_state::*;

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
pub struct TransitionState {
    pub exiting: Option<MetaState>,
    pub entering: Option<MetaState>,
    pub unpause_at_end: bool,
}
impl Default for TransitionState {
    fn default() -> Self {
        Self {
            entering: None,
            exiting: None,
            unpause_at_end: true,
        }
    }
}
impl TransitionState {
    pub fn is_active(&self) -> bool {
        match (&self.entering, &self.exiting) {
            (Some(_), Some(_)) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect, States)]
pub struct LevelScrollStateInner {
    pub from_pos: IVec2,
    pub to_pos: IVec2,
    pub time_milli: u32,
}

#[derive(Clone, Default, Debug, Eq, Hash, PartialEq, Reflect, States)]
pub struct LevelScrollState {
    pub active: Option<LevelScrollStateInner>,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Reflect, States)]
pub enum ConvoMetaState {
    #[default]
    None,
    Some,
}

pub(super) struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        debug_resource!(app, State<MetaState>);

        app.insert_state(MetaState::Menu(MenuState::Bevy));
        // app.insert_state(MetaState::Menu(MenuState::Savefile));

        app.insert_state(TransitionState::default());
        app.insert_state(PauseState::Unpaused);
        app.insert_state(ConvoMetaState::None);
        app.insert_state(CutsceneState::None);
        app.insert_state(LevelScrollState::default());

        app.add_computed_state::<MetaStateKind>();
        app.add_computed_state::<PhysicsState>();
        app.add_computed_state::<TransitionActiveState>();
        app.add_computed_state::<MenuState>();
        app.add_computed_state::<MenuStateKind>();
        app.add_computed_state::<WorldLoadingState>();
        app.add_computed_state::<WorldState>();
        app.add_computed_state::<LevelState>();
        app.add_computed_state::<PlayerMetaState>();
        app.add_computed_state::<LevelScrollStateKind>();
    }
}
