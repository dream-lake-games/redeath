use crate::prelude::*;

#[derive(Component)]
struct PlayerHighIndicator;

#[derive(Bundle)]
struct PlayerHighIndicatorBundle {
    name: Name,
    marker: PlayerHighIndicator,
    pos: Pos,
    spatial: SpatialBundle,
    anim: AnimMan<PlayerHighIndicatorAnim>,
}
impl Default for PlayerHighIndicatorBundle {
    fn default() -> Self {
        Self {
            name: Name::new("player_high_indicator"),
            marker: PlayerHighIndicator,
            pos: Pos::default(),
            spatial: Pos::default().to_spatial(ZIX_PLAYER + 0.0152),
            anim: default(),
        }
    }
}

fn ensure_one_exists(
    mut commands: Commands,
    q: Query<&PlayerHighIndicator>,
    root: Res<WorldDetailRoot>,
) {
    if q.is_empty() {
        commands
            .spawn(PlayerHighIndicatorBundle::default())
            .set_parent(root.eid());
    }
}

fn position_indicator(
    mut indicator_q: Query<
        (&mut Pos, &mut Visibility),
        (With<PlayerHighIndicator>, Without<Player>),
    >,
    player_pos_q: Query<&Pos, With<Player>>,
    scroll_state_kind: Res<State<LevelScrollStateKind>>,
    level_rects: Option<Res<LevelRects>>,
) {
    let Ok((mut indicator_pos, mut indicator_viz)) = indicator_q.get_single_mut() else {
        warn!("hmm there should be an indicator by now");
        return;
    };
    let mut nope = || {
        *indicator_viz = Visibility::Hidden;
    };
    if scroll_state_kind.get() == &LevelScrollStateKind::Some {
        nope();
        return;
    }
    let Ok(player_pos) = player_pos_q.get_single() else {
        nope();
        return;
    };
    let Some(level_rects) = level_rects else {
        nope();
        return;
    };
    let Some(current_level_rect) = level_rects.current else {
        nope();
        return;
    };
    if player_pos.y < current_level_rect.max.y {
        nope();
        return;
    }
    indicator_pos.x = player_pos.x;
    indicator_pos.y = current_level_rect.max.y - 6.0;
    *indicator_viz = Visibility::Inherited;
}

pub(super) fn register_player_high_indicator(app: &mut App) {
    app.add_systems(
        Update,
        (ensure_one_exists, position_indicator)
            .chain()
            .after(PhysicsSet)
            .after(MyLdtkLevelMaint),
    );
}
