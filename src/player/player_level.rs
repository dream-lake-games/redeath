use crate::prelude::*;

fn update_player_level(
    player_q: Query<&PhysicalLid, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
) {
    let plid = player_q.single();
    if let Some(iid) = &plid.last_known_iid {
        *level_selection = LevelSelection::iid(iid.clone());
    }
}

pub(super) fn register_player_level(app: &mut App) {
    app.add_systems(
        Update,
        update_player_level
            .run_if(in_state(PlayerMetaState::Playing))
            .after(MyLdtkLevelMaint),
    );
}
