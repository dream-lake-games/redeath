//! I opened a help thread for this in the bevy discord, but ordering
//! Startup and OnEnter(<initialstate>) is like choosing between a rock and a hard place:
//! 1. My intution is startup is first
//! 2. BUT if startup is first, it's possible to be _in_ a state which was never entered
//! Bevy chooses to favor 2, so that OnEnter(initialstate) fires before startup.
//! Because I still want to be able to have systems run before anything else, the sane
//! solution is just to make my own state which I know is first.
//! I _think_ I should probably just never use `Startup` and always favor `OnEnter(MetaStateKind::Setup)`.
//! Will be nice if I want to subdivide setup too.

use crate::prelude::*;

/// Rn this just makes it so it'll spend one frame in setup then move on
fn update(mut next_meta_state: ResMut<NextState<MetaState>>) {
    // next_meta_state.set(MenuState::Bevy.to_meta_state());
    next_meta_state.set(MenuState::Savefile.to_meta_state());
}

pub(super) fn register_setup(app: &mut App) {
    app.add_systems(Update, update.run_if(in_state(MetaStateKind::Setup)));
}
