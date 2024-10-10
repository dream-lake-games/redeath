use crate::prelude::*;

fn player_meta_state_checks(
    player_q: Query<Entity, With<Player>>,
    player_meta_state: Res<State<PlayerMetaState>>,
) {
    let player_count = player_q.iter().count();
    match player_meta_state.get() {
        PlayerMetaState::NoneOk | PlayerMetaState::Spawning => {
            debug_assert!(player_count == 0);
        }
        PlayerMetaState::Puppet | PlayerMetaState::Playing => {
            debug_assert!(player_count == 1);
        }
        PlayerMetaState::Dying => {
            // Haven't thought about this yet
        }
    }
}

pub(super) fn register_player_invariants(app: &mut App) {
    app.add_systems(First, player_meta_state_checks);
}
