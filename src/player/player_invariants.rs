use crate::prelude::*;

fn player_meta_state_checks(
    player_q: Query<Entity, With<Player>>,
    player_meta_state: Option<Res<State<PlayerMetaState>>>,
) {
    let Some(player_meta_state) = player_meta_state else {
        return;
    };
    let player_count = player_q.iter().count();
    match player_meta_state.get() {
        PlayerMetaState::NoneOk | PlayerMetaState::Spawning => {
            debug_assert!(player_count == 0);
        }
        PlayerMetaState::Puppet | PlayerMetaState::Playing | PlayerMetaState::Dying => {
            debug_assert!(player_count == 1);
        }
    }
}

pub(super) fn register_player_invariants(app: &mut App) {
    app.add_systems(
        PostUpdate,
        player_meta_state_checks.run_if(in_state(MetaStateKind::World)),
    );
}
