use super::playerlude::*;
use crate::prelude::*;

#[derive(Resource, Clone, Debug, Reflect)]
struct PlayerMovementConsts {
    /// Maximum horizontal speed (when not dashing)
    max_hor_speed: f32,
    /// Horizontal acceleration
    hor_acc: f32,
    /// Reduces acceleration when in the air
    air_hor_friction: f32,
    /// Maximum vertical speed (when not dashing)
    max_ver_speed: f32,
    /// How quickly to slow down back to max speed when over max speed (for instance, post-dash)
    over_max_slowdown_acc: f32,
    /// Jump speed
    jump_vel: f32,
    dash_speed: f32,
    dash_time: f32,
    coyote_time: f32,
}
impl Default for PlayerMovementConsts {
    fn default() -> Self {
        Self {
            max_hor_speed: 90.0,
            hor_acc: 480.0,
            air_hor_friction: 0.66,
            max_ver_speed: 200.0,
            over_max_slowdown_acc: 960.0,
            jump_vel: 156.0,
            dash_speed: 200.0,
            dash_time: 0.1,
            coyote_time: 0.2,
        }
    }
}

fn update_touching(
    player: Query<(Entity, &StaticRxCtrl), With<Player>>,
    colls: Res<StaticColls>,
    mut commands: Commands,
) {
    let (eid, srx_ctrl) = player.single();
    let colls = colls.get_refs(&srx_ctrl.coll_keys).by_rx_hbox();
    let marker_tx_kinds = |marker: u32| {
        colls
            .get(&marker)
            .map(|v| v.iter().map(|i| i.tx_kind.clone()).collect::<HashSet<_>>())
            .unwrap_or(default())
    };
    let touching = TouchingDir::default()
        .with_right(marker_tx_kinds(PLAYER_RIGHT_HBOX).contains(&StaticTxKind::Solid))
        .with_up(marker_tx_kinds(PLAYER_ABOVE_HBOX).contains(&StaticTxKind::Solid))
        .with_left(marker_tx_kinds(PLAYER_LEFT_HBOX).contains(&StaticTxKind::Solid))
        .with_down({
            let kinds = marker_tx_kinds(PLAYER_BELOW_HBOX);
            kinds.contains(&StaticTxKind::Solid) || kinds.contains(&StaticTxKind::PassUp)
        });
    commands.entity(eid).insert(touching);
}

fn update_can_jump(
    mut player: Query<
        (
            Entity,
            &TouchingDir,
            Option<&mut CanRegularJump>,
            Option<&mut CanWallJumpFromLeft>,
            Option<&mut CanWallJumpFromRight>,
        ),
        With<Player>,
    >,
    time: Res<Time>,
    mut commands: Commands,
    consts: Res<PlayerMovementConsts>,
) {
    let (eid, touching, mut can_regular, mut can_wall_left, mut can_wall_right) =
        player.single_mut();
    // Update regular jump
    if touching.down() {
        commands.entity(eid).insert(CanRegularJump {
            coyote_time: consts.coyote_time,
        });
    } else {
        if let Some(regular_mut) = can_regular.as_mut() {
            regular_mut.coyote_time -= time.delta_seconds();
            if regular_mut.coyote_time < 0.0 {
                commands.entity(eid).remove::<CanRegularJump>();
            }
        }
    }
    // Update left wall jump
    if touching.left() {
        commands.entity(eid).insert(CanWallJumpFromLeft {
            coyote_time: consts.coyote_time,
        });
    } else {
        if let Some(wall_left_mut) = can_wall_left.as_mut() {
            wall_left_mut.coyote_time -= time.delta_seconds();
            if wall_left_mut.coyote_time < 0.0 {
                commands.entity(eid).remove::<CanWallJumpFromLeft>();
            }
        }
    }
    // Update right wall jump
    if touching.right() {
        commands.entity(eid).insert(CanWallJumpFromRight {
            coyote_time: consts.coyote_time,
        });
    } else {
        if let Some(wall_right_mut) = can_wall_right.as_mut() {
            wall_right_mut.coyote_time -= time.delta_seconds();
            if wall_right_mut.coyote_time < 0.0 {
                commands.entity(eid).remove::<CanWallJumpFromRight>();
            }
        }
    }
}

fn update_current_dash(
    mut player: Query<(Entity, &mut Dashing), With<Player>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let Ok((eid, mut dashing)) = player.get_single_mut() else {
        // Means player is not dashing
        return;
    };
    dashing.time_left -= time.delta_seconds();
    if dashing.time_left < 0.0 {
        commands.entity(eid).insert(Gravity::default());
        commands.entity(eid).remove::<Dashing>();
    }
}

/// NOTE: Other ways of restoring dash are not handled here.
///       This is only for giving your dash back when you are on the ground and not dashing
fn update_can_dash_from_ground(
    player: Query<(Entity, &TouchingDir), (With<Player>, Without<Dashing>)>,
    mut commands: Commands,
) {
    let Ok((eid, touching)) = player.get_single() else {
        // Means player is dashing
        return;
    };
    if touching.down() {
        commands.entity(eid).insert(CanDash);
    }
}

fn maybe_start_dash(
    mut player: Query<(Entity, &mut Dyno), (With<Player>, With<CanDash>)>,
    butt: Res<ButtInput>,
    dir: Res<DirInput>,
    consts: Res<PlayerMovementConsts>,
    mut commands: Commands,
) {
    let Ok((eid, mut dyno)) = player.get_single_mut() else {
        // Means the player can't dash
        return;
    };
    if butt.just_pressed(ButtKind::B) && dir.length_squared() > 0.1 {
        let card_dir = CardDir::from_vec2(dir.as_vec2());
        dyno.vel = card_dir.as_non_normal_vec2().normalize_or_zero() * consts.dash_speed;
        commands.entity(eid).insert(Dashing {
            time_left: consts.dash_time,
        });
        commands.entity(eid).remove::<CanDash>();
        commands.entity(eid).remove::<Gravity>();
    }
}

fn maybe_start_regular_jump(
    mut player: Query<(Entity, &mut Dyno), (With<Player>, With<CanRegularJump>, Without<Dashing>)>,
    butt: Res<ButtInput>,
    consts: Res<PlayerMovementConsts>,
    mut commands: Commands,
) {
    let Ok((eid, mut dyno)) = player.get_single_mut() else {
        // Means the player can't jump rn
        return;
    };
    if butt.just_pressed(ButtKind::A) {
        dyno.vel.y = consts.jump_vel;
        commands.entity(eid).remove::<CanRegularJump>();
        commands.entity(eid).remove::<CanWallJumpFromLeft>();
        commands.entity(eid).remove::<CanWallJumpFromRight>();
    }
}

fn maybe_start_wall_jump(
    mut player: Query<
        (Entity, &mut Dyno, Option<&CanWallJumpFromLeft>),
        (
            With<Player>,
            Or<(With<CanWallJumpFromLeft>, With<CanWallJumpFromRight>)>,
            Without<Dashing>,
            Without<CanRegularJump>,
        ),
    >,
    butt: Res<ButtInput>,
    consts: Res<PlayerMovementConsts>,
    mut commands: Commands,
) {
    let Ok((eid, mut dyno, from_left)) = player.get_single_mut() else {
        // Means the player can't wall jump rn
        return;
    };
    if butt.just_pressed(ButtKind::A) {
        let from_left = from_left.is_some();
        let x_mul = if from_left { 1.0 } else { -1.0 };
        dyno.vel.x = consts.jump_vel * x_mul * 0.5;
        dyno.vel.y = consts.jump_vel;
        commands.entity(eid).remove::<CanWallJumpFromLeft>();
        commands.entity(eid).remove::<CanWallJumpFromRight>();
    }
}

fn move_horizontally(
    mut player: Query<(&mut Dyno, &TouchingDir), (With<Player>, Without<Dashing>)>,
    dir: Res<DirInput>,
    consts: Res<PlayerMovementConsts>,
    bullet_time: Res<BulletTime>,
) {
    let Ok((mut dyno, touching)) = player.get_single_mut() else {
        // Means the player can't move horizontally rn
        return;
    };
    let friction = if touching.down() {
        1.0
    } else {
        consts.air_hor_friction
    };
    let acc = consts.hor_acc * bullet_time.delta_seconds() * friction;
    if dir.x.abs() < 0.01 {
        // Go towards 0.0
        if acc >= dyno.vel.x.abs() {
            // We would overshoot 0, hard 0.0
            dyno.vel.x = 0.0;
        } else {
            dyno.vel.x -= dyno.vel.x.signum() * acc;
        }
    } else {
        // Accelerate
        dyno.vel.x += dir.x.signum() * acc;
    }
}

fn limit_speed() {}

pub(super) fn register_player_movement(app: &mut App) {
    app.insert_resource(PlayerMovementConsts::default());

    // Update touching. Should happen first and whenever there's a spawned player.
    app.add_systems(
        Update,
        update_touching
            .before(AnimSet)
            .in_set(PlayerSet)
            .in_set(PlayerMovementSet)
            .after(InputSet)
            .after(PhysicsSet)
            .run_if(
                in_state(PlayerMetaState::Puppet)
                    .or_else(in_state(PlayerMetaState::Playing))
                    .or_else(in_state(PlayerMetaState::Dying)),
            ),
    );
    // Then do the actual movement stuff
    app.add_systems(
        Update,
        (
            update_can_jump,
            update_current_dash,
            update_can_dash_from_ground,
            maybe_start_dash,
            maybe_start_regular_jump,
            maybe_start_wall_jump,
            move_horizontally,
            limit_speed,
        )
            .chain()
            .before(AnimSet)
            .in_set(PlayerSet)
            .in_set(PlayerMovementSet)
            .after(InputSet)
            .after(PhysicsSet)
            .after(update_touching)
            .run_if(in_state(PlayerMetaState::Playing)),
    );
}
