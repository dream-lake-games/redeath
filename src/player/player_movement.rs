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
    /// What's the max speed when wall sliding?
    max_wall_slide_ver_speed: f32,
    /// How quickly to slow down back to max speed when over max speed (for instance, post-dash)
    over_max_slowdown_acc: f32,
    /// Jump speed
    jump_speed: f32,
    /// How long after a wall jump to mess with input?
    post_jump_time: f32,
    dash_speed: f32,
    dash_time: f32,
    coyote_time: f32,
}
impl Default for PlayerMovementConsts {
    fn default() -> Self {
        Self {
            max_hor_speed: 100.0,
            hor_acc: 600.0,
            air_hor_friction: 0.75,
            max_ver_speed: 190.0,
            max_wall_slide_ver_speed: 40.0,
            over_max_slowdown_acc: 960.0,
            jump_speed: 190.0,
            post_jump_time: 0.136,
            dash_speed: 160.0,
            dash_time: 0.25,
            coyote_time: 0.1,
        }
    }
}

fn update_touching(
    player: Query<(Entity, &StaticRxCtrl), With<Player>>,
    colls_res: Res<StaticColls>,
    mut commands: Commands,
) {
    let (eid, srx_ctrl) = player.single();
    let colls = colls_res.get_refs(&srx_ctrl.coll_keys).by_rx_hbox();
    let marker_tx_kinds = |marker: u32| {
        colls
            .get(&marker)
            .map(|v| v.iter().map(|i| i.tx_kind.clone()).collect::<HashSet<_>>())
            .unwrap_or(default())
    };
    let touching = TouchingDir::default()
        .with_right(
            marker_tx_kinds(PLAYER_RIGHT_HBOX)
                .iter()
                .any(|kind| matches!(kind, StaticTxKind::Solid | StaticTxKind::SolidFragile)),
        )
        .with_up(
            marker_tx_kinds(PLAYER_ABOVE_HBOX)
                .iter()
                .any(|kind| matches!(kind, StaticTxKind::Solid | StaticTxKind::SolidFragile)),
        )
        .with_left(
            marker_tx_kinds(PLAYER_LEFT_HBOX)
                .iter()
                .any(|kind| matches!(kind, StaticTxKind::Solid | StaticTxKind::SolidFragile)),
        )
        .with_down({
            marker_tx_kinds(PLAYER_BELOW_HBOX).iter().any(|kind| {
                matches!(
                    kind,
                    StaticTxKind::Solid | StaticTxKind::SolidFragile | StaticTxKind::PassUp
                )
            })
        });
    commands.entity(eid).insert(touching);
}

fn update_forceful_touching(
    mut commands: Commands,
    player: Query<(Entity, &Dyno, &StaticRxCtrl, &TouchingDir), With<Player>>,
    colls_res: Res<StaticColls>,
    dynos: Query<&Dyno>,
    dir_input: Res<DirInput>,
) {
    let (eid, player_dyno, srx_ctrl, touching) = player.single();
    let colls = colls_res.get_refs(&srx_ctrl.coll_keys).by_rx_hbox();

    // Because we haven't moved horizontally yet, we don't know if we are pushing into a wall
    // Adjust here for horizontal checks
    let player_vel = player_dyno.vel + Vec2::new(dir_input.x * 0.1, 0.0);

    let marker_vels = |marker: u32| {
        let colls = colls.get(&marker).unwrap();
        colls.iter().map(|coll| {
            let dyno = dynos.get(coll.tx_ctrl).cloned().unwrap_or_default();
            dyno.vel
        })
    };

    let mut forceful = ForcefulTouchingDir::default();
    if touching.right() {
        let mut relevant = marker_vels(PLAYER_RIGHT_HBOX);
        forceful.set_right(relevant.any(|vel| vel.x < player_vel.x));
    }
    if touching.up() {
        let mut relevant = marker_vels(PLAYER_ABOVE_HBOX);
        forceful.set_up(relevant.any(|vel| vel.y < player_vel.y));
    }
    if touching.left() {
        let mut relevant = marker_vels(PLAYER_LEFT_HBOX);
        forceful.set_left(relevant.any(|vel| vel.x > player_vel.x));
    }
    if touching.down() {
        let mut relevant = marker_vels(PLAYER_BELOW_HBOX);
        forceful.set_down(relevant.any(|vel| vel.y > player_vel.y));
    }
    commands.entity(eid).insert(forceful);
}

fn update_can_jump(
    mut player: Query<
        (
            Entity,
            &ForcefulTouchingDir,
            Option<&mut CanRegularJump>,
            Option<&mut CanWallJumpFromLeft>,
            Option<&mut CanWallJumpFromRight>,
            Option<&mut PostJump>,
        ),
        With<Player>,
    >,
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
    mut commands: Commands,
    consts: Res<PlayerMovementConsts>,
) {
    let (eid, forceful, mut can_regular, mut can_wall_left, mut can_wall_right, mut post_jump) =
        player.single_mut();
    // Update regular jump
    if forceful.down() {
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
    if forceful.left() {
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
    if forceful.right() {
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
    // BUT if we are post jump we can't jump
    if let Some(post_jump) = post_jump.as_mut() {
        commands.entity(eid).remove::<CanRegularJump>();
        commands.entity(eid).remove::<CanWallJumpFromLeft>();
        commands.entity(eid).remove::<CanWallJumpFromRight>();
        post_jump.time_left -= bullet_time.delta_seconds();
        if post_jump.time_left < 0.0 {
            commands.entity(eid).remove::<PostJump>();
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
    player: Query<(Entity, &ForcefulTouchingDir), (With<Player>, Without<Dashing>)>,
    mut commands: Commands,
) {
    let Ok((eid, forceful_touching)) = player.get_single() else {
        // Means player is dashing
        return;
    };
    if forceful_touching.down() {
        commands.entity(eid).insert(CanDash);
    }
}

fn update_can_dash_from_replenish(
    player: Query<(Entity, &TriggerRxCtrl), (With<Player>, Without<CanDash>)>,
    trigger_colls: Res<TriggerColls>,
    mut commands: Commands,
) {
    let Ok((eid, trx_ctrl)) = player.get_single() else {
        // Means the player can already dash
        return;
    };
    let replenish_coll = trigger_colls
        .get_refs(&trx_ctrl.coll_keys)
        .into_iter()
        .filter(|coll| coll.tx_kind == TriggerTxKind::Replenish)
        .next();
    if let Some(coll) = replenish_coll {
        commands.entity(eid).insert(CanDash);
        commands.entity(coll.tx_ctrl).remove::<TriggerTxCtrl>();
    }
}

fn maybe_start_dash(
    mut player: Query<(Entity, &mut Dyno, &AnimMan<PlayerAnim>), (With<Player>, With<CanDash>)>,
    butt: Res<ButtInput>,
    dir: Res<DirInput>,
    consts: Res<PlayerMovementConsts>,
    mut commands: Commands,
) {
    let Ok((eid, mut dyno, anim)) = player.get_single_mut() else {
        // Means the player can't dash
        return;
    };
    if butt.just_pressed(ButtKind::B) {
        let card_dir = if dir.as_vec2().length_squared() > 0.1 {
            CardDir::from_vec2(dir.as_vec2())
        } else {
            if anim.get_flip_x() {
                CardDir::W
            } else {
                CardDir::E
            }
        };
        dyno.vel = card_dir.as_non_normal_vec2().normalize_or_zero() * consts.dash_speed;
        commands.entity(eid).insert(Dashing {
            time_left: consts.dash_time,
        });
        commands.entity(eid).remove::<CanDash>();
        commands.entity(eid).remove::<Gravity>();
        commands.entity(eid).remove::<CanRegularJump>();
        commands.entity(eid).remove::<CanWallJumpFromLeft>();
        commands.entity(eid).remove::<CanWallJumpFromRight>();
        let event = DashEvent { dir: card_dir };
        commands.trigger(event);
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
        dyno.vel.y = consts.jump_speed;
        let kind = JumpKind::Regular;
        commands.entity(eid).insert(PostJump {
            kind,
            time_left: consts.post_jump_time,
        });
        commands.entity(eid).remove::<CanRegularJump>();
        commands.entity(eid).remove::<CanWallJumpFromLeft>();
        commands.entity(eid).remove::<CanWallJumpFromRight>();
        let event = JumpEvent { kind };
        commands.trigger(event);
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
    if !butt.just_pressed(ButtKind::A) {
        return;
    }
    let from_left = from_left.is_some();
    if butt.just_pressed(ButtKind::A) {
        let x_mul = if from_left { 1.0 } else { -1.0 };
        dyno.vel.x = consts.max_hor_speed * x_mul;
        dyno.vel.y = consts.jump_speed;
        let kind = if from_left {
            JumpKind::FromLeftWall
        } else {
            JumpKind::FromRightWall
        };
        commands.entity(eid).insert(PostJump {
            kind,
            time_left: consts.post_jump_time,
        });
        commands.entity(eid).remove::<CanRegularJump>();
        commands.entity(eid).remove::<CanWallJumpFromLeft>();
        commands.entity(eid).remove::<CanWallJumpFromRight>();
        let event = JumpEvent { kind };
        commands.trigger(event);
    }
}

fn move_horizontally(
    mut player: Query<
        (&mut Dyno, &ForcefulTouchingDir, Option<&PostJump>),
        (With<Player>, Without<Dashing>),
    >,
    dir: Res<DirInput>,
    consts: Res<PlayerMovementConsts>,
    bullet_time: Res<BulletTime>,
) {
    let Ok((mut dyno, forceful_touching, post_jump)) = player.get_single_mut() else {
        // Means the player can't move horizontally rn
        return;
    };
    let friction = if forceful_touching.down() {
        1.0
    } else {
        consts.air_hor_friction
    };

    if !forceful_touching.down()
        && matches!(
            post_jump.map(|s| s.kind),
            Some(JumpKind::FromLeftWall) | Some(JumpKind::FromRightWall)
        )
    {
        // Don't let the player decelerate back into wall while post is happening
        let post_jump = post_jump.unwrap();
        if matches!(post_jump.kind, JumpKind::FromLeftWall) && dyno.vel.x < 0.0 {
            return;
        }
        if matches!(post_jump.kind, JumpKind::FromRightWall) && dyno.vel.x > 0.0 {
            return;
        }
    } else {
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
}

fn limit_speed(
    mut player: Query<(&mut Dyno, &ForcefulTouchingDir), (With<Player>, Without<Dashing>)>,
    consts: Res<PlayerMovementConsts>,
    bullet_time: Res<BulletTime>,
) {
    let Ok((mut dyno, forceful_touching)) = player.get_single_mut() else {
        // Means the player can't move horizontally rn
        return;
    };
    let acc = consts.over_max_slowdown_acc * bullet_time.delta_seconds();
    // Hor
    if dyno.vel.x.abs() > consts.max_hor_speed {
        dyno.vel.x -= dyno.vel.x.signum() * acc;
        if dyno.vel.x.abs() < consts.max_hor_speed {
            dyno.vel.x = dyno.vel.x.signum() * consts.max_hor_speed;
        }
    }
    // Ver
    let wall_sliding = dyno.vel.y < 0.0 && (forceful_touching.right() || forceful_touching.left());
    let actual_max_ver_speed = if wall_sliding {
        consts.max_wall_slide_ver_speed
    } else {
        consts.max_ver_speed
    };
    if dyno.vel.y.abs() > actual_max_ver_speed {
        dyno.vel.y -= dyno.vel.y.signum() * acc;
        if dyno.vel.y.abs() < actual_max_ver_speed {
            dyno.vel.y = dyno.vel.y.signum() * actual_max_ver_speed;
        }
    }
}

fn update_breaking(
    player: Query<(&StaticRxCtrl, &AnimMan<PlayerAnim>, Option<&Dashing>)>,
    mut srx_comps: Query<&mut StaticRxComp>,
) {
    let (srx_ctrl, anim, dashing) = player.single();
    for comp_eid in &srx_ctrl.comps {
        let mut comp = srx_comps.get_mut(*comp_eid).unwrap();
        if comp.kind == StaticRxKind::Observe {
            continue;
        }
        comp.kind = if matches!(anim.get_state(), PlayerAnim::Dash) || dashing.is_some() {
            StaticRxKind::DefaultBreaker
        } else {
            StaticRxKind::Default
        }
    }
}

fn keep_inside_edge_level(
    mut player_q: Query<(&mut Pos, &PhysicalLid), With<Player>>,
    level_rects: Res<LevelRects>,
) {
    let (mut player_pos, plid) = player_q.single_mut();
    let Some(lid) = &plid.last_known_iid else {
        return;
    };
    let Some(rect) = level_rects.get(lid) else {
        warn!("hmm looks weird keep inside edge");
        return;
    };
    let right_in_some_level = level_rects
        .values()
        .any(|rect| rect.contains(player_pos.translated(Vec2::new(4.0, 0.0)).as_vec2()));
    let left_in_some_level = level_rects
        .values()
        .any(|rect| rect.contains(player_pos.translated(Vec2::new(-4.0, 0.0)).as_vec2()));
    if !right_in_some_level {
        player_pos.x = player_pos.x.min(rect.max.x - 4.0);
    }
    if !left_in_some_level {
        player_pos.x = player_pos.x.max(rect.min.x + 4.0);
    }
}

pub(super) fn register_player_movement(app: &mut App) {
    app.insert_resource(PlayerMovementConsts::default());

    // Update touching. Should happen first and whenever there's a spawned player.
    app.add_systems(
        Update,
        (update_touching, update_forceful_touching)
            .chain()
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
    // Then do the actual movement stuff, only in playing states
    app.add_systems(
        Update,
        (
            update_can_jump,
            update_can_dash_from_ground,
            update_can_dash_from_replenish,
            maybe_start_dash,
            maybe_start_regular_jump,
            maybe_start_wall_jump,
            move_horizontally,
            limit_speed,
            update_breaking,
            keep_inside_edge_level,
        )
            .chain()
            .before(AnimSet)
            .in_set(PlayerSet)
            .in_set(PlayerMovementSet)
            .after(InputSet)
            .after(PhysicsSet)
            .after(update_forceful_touching)
            .run_if(in_state(PlayerMetaState::Playing)),
    );
    // Lol except this stuff, which should happen for puppets so they end
    app.add_systems(
        Update,
        (update_current_dash,)
            .chain()
            .before(AnimSet)
            .in_set(PlayerSet)
            .in_set(PlayerMovementSet)
            .after(InputSet)
            .after(PhysicsSet)
            .after(keep_inside_edge_level)
            .run_if(in_state(PlayerMetaState::Playing).or_else(in_state(PlayerMetaState::Puppet))),
    );
}
