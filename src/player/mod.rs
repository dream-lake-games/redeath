use crate::prelude::*;

mod player_invariants;
mod scratch;
mod spawn;

#[derive(Component)]
pub struct Player;

pub(super) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        scratch::register_scratch(app);
        spawn::register_spawn(app);
    }
}
