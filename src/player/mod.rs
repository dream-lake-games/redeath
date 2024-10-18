use crate::prelude::*;

mod player_invariants;
mod scratch;
mod spawn;

/// The set that contains all player related systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerSet;

#[derive(Component)]
pub struct Player;

pub(super) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        player_invariants::register_player_invariants(app);
        scratch::register_scratch(app);
        spawn::register_spawn(app);
    }
}
