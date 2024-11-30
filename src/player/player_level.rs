use crate::prelude::*;

fn update_player_level(
    player_q: Query<(&Pos, &PhysicalLid), With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    mut next_level_scroll_state: ResMut<NextState<LevelScrollState>>,
    camera_pos: Query<&Pos, With<DynamicCamera>>,
    level_rects: Res<LevelRects>,
) {
    let (player_pos, plid) = player_q.single();
    if let Some(iid) = &plid.last_known_iid {
        let LevelSelection::Iid(existing_iid) = level_selection.clone() else {
            panic!("bad");
        };
        if iid != existing_iid.as_str() {
            let next_rect = level_rects.get(iid).cloned().unwrap_or_default();
            let to_pos = camera_clamp_logic(player_pos, &next_rect);
            next_level_scroll_state.set(LevelScrollState {
                active: Some(LevelScrollStateInner {
                    from_pos: camera_pos.single().as_ivec2(),
                    to_pos: to_pos.as_ivec2(),
                    time_milli: 0,
                }),
            });
            *level_selection = LevelSelection::Iid(LevelIid::new(iid.clone()));
        }
    }
}

pub(super) fn register_player_level(app: &mut App) {
    app.add_systems(
        Update,
        update_player_level
            .run_if(in_state(PlayerMetaState::Playing))
            .run_if(in_state(LevelScrollStateKind::None))
            .after(MyLdtkLevelMaint)
            .after(CameraSet),
    );
}
