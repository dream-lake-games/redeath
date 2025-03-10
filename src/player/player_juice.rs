use super::playerlude::*;
use crate::prelude::*;

#[derive(Resource, Clone, Debug, Reflect)]
struct PlayerJuiceConsts {
    /// Minimum speed of impact before spawning a sound
    impact_sound_floor: f32,
    /// Above this speed, sound of impact will be the same
    impact_sound_ceiling: f32,
    impact_sound_mult: f32,
    // Time to do the global palette shift after dashing
    dash_global_shift_time: f32,
}
impl Default for PlayerJuiceConsts {
    fn default() -> Self {
        Self {
            impact_sound_floor: 2.0,
            impact_sound_ceiling: 164.0,
            impact_sound_mult: 0.11,
            dash_global_shift_time: 0.2,
        }
    }
}

fn juice_dash_suspense(
    trigger: Trigger<DashSuspenseEvent>,
    mut commands: Commands,
    player: Query<(&Pos, &AnimMan<PlayerAnim>), With<Player>>,
    world_detail_root: Res<WorldDetailRoot>,
    mut global_shift: ResMut<GlobalPaletteShift>,
    consts: Res<PlayerJuiceConsts>,
    mut bullet_time: ResMut<BulletTime>,
) {
    let event = trigger.event();

    // Dash die
    let (player_pos, player_anim) = player.single();
    commands
        .spawn(EphemeralAnim::new(
            DashDieAnim::DashDie,
            player_anim.get_flip_x(),
            *player_pos,
            ZIX_PLAYER + 1.0,
        ))
        .set_parent(world_detail_root.eid());

    // Global shift
    global_shift.add(consts.dash_global_shift_time / 2.0, 1);
    global_shift.add(consts.dash_global_shift_time, 1);

    // Storm
    commands.spawn(SoundEffect::PlayerThunder);
    commands.spawn(Lightning);

    // Bullet time
    bullet_time.set_temp(BulletTimeSpeed::Stopped, event.suspense_time);
}

fn juice_dash_post_suspense(trigger: Trigger<DashEvent>, mut camera_shake: ResMut<CameraShake>) {
    let event = trigger.event();

    // Camera shake
    let dir_not_norm = event.dir.as_non_normal_vec2();
    let x_range = if dir_not_norm.x.abs() > 0.1 {
        if dir_not_norm.x > 0.0 {
            1..=1
        } else {
            -1..=-1
        }
    } else {
        0..=0
    };
    let y_range = if dir_not_norm.y.abs() > 0.1 {
        if dir_not_norm.y > 0.0 {
            1..=1
        } else {
            -1..=-1
        }
    } else {
        0..=0
    };
    camera_shake.shake(0.1, x_range, y_range);
}

fn juice_during_dash(
    player: Query<(&Pos, &AnimMan<PlayerAnim>, &Dashing), With<Player>>,
    mut commands: Commands,
    root: Res<WorldDetailRoot>,
) {
    let Ok((pos, anim, dashing)) = player.get_single() else {
        return;
    };
    commands
        .spawn(EphemeralAnim::new(
            DashFadeAnim::DashFade,
            anim.get_flip_x(),
            *pos,
            ZIX_PLAYER - dashing.time_left,
        ))
        .set_parent(root.eid());
}

fn juice_after_jump(
    trigger: Trigger<JumpEvent>,
    player: Query<&Pos, With<Player>>,
    mut commands: Commands,
    sound_root: Res<SoundRoot>,
) {
    // Smoke
    let player_pos = player.single();
    let event = trigger.event();
    let (anim, offset, flipx) = match event.kind {
        JumpKind::Regular => (
            vec![JumpSmokeAnim::Regular1].pick(),
            Vec2::new(0.0, -3.0),
            false,
        ),
        JumpKind::FromLeftWall => (
            vec![JumpSmokeAnim::Wall1].pick(),
            Vec2::new(0.0, 0.0),
            false,
        ),
        JumpKind::FromRightWall => (vec![JumpSmokeAnim::Wall1].pick(), Vec2::new(0.0, 0.0), true),
    };
    let smoke_pos = player_pos.translated(offset);
    let man = AnimMan::new(anim).with_flip_x(flipx);
    commands.spawn((
        man,
        smoke_pos.to_transform(ZIX_PLAYER + 1.0),
        Visibility::Inherited,
    ));

    // Sound
    commands
        .spawn(SoundEffect::PlayerJump)
        .set_parent(sound_root.eid());
}

/// TODO: I think I want to kill the jump events and just use this? But maybe it's fine? Idk
fn juice_animation_state_response(
    trigger: Trigger<AnimStateChange<PlayerAnim>>,
    player: Query<(&Pos, &AnimMan<PlayerAnim>), With<Player>>,
    mut commands: Commands,
) {
    let (pos, anim) = player.single();
    match trigger.event().next {
        PlayerAnim::Land => {
            let man = AnimMan::new(JumpSmokeAnim::Land1).with_flip_x(anim.get_flip_x());
            commands.spawn((
                man,
                pos.translated(Vec2::new(0.0, -3.0))
                    .to_transform(ZIX_PLAYER + 1.0),
                Visibility::Inherited,
            ));
        }
        _ => {}
    }
}

fn juice_animation_ix_response(
    trigger: Trigger<AnimIxChange<PlayerAnim>>,
    player: Query<(&Pos, &AnimMan<PlayerAnim>), With<Player>>,
    mut commands: Commands,
    sound_root: Res<SoundRoot>,
) {
    let event = trigger.event();
    let (pos, anim) = player.single();
    match (event.state, event.ix) {
        (PlayerAnim::Run, 3) => {
            let offset = Vec2::new(if anim.get_flip_x() { -1.0 } else { 1.0 } * 5.0, -3.0);
            commands.spawn((
                AnimMan::new(RunSmokeAnim::Run1),
                pos.translated(offset).to_transform(ZIX_PLAYER + 1.01),
                Visibility::Inherited,
            ));
            commands
                .spawn(SoundEffect::PlayerRunStep)
                .set_parent(sound_root.eid());
        }
        _ => {}
    }
}

fn juice_wall_slide(
    player: Query<(&AnimMan<PlayerAnim>, &Pos), With<Player>>,
    mut commands: Commands,
    sound_root: Res<SoundRoot>,
) {
    let (anim, player_pos) = player.single();
    if matches!(
        anim.get_state(),
        PlayerAnim::WallSlide | PlayerAnim::WallSlideExhausted
    ) {
        let man = AnimMan::new(vec![WallSlideSmokeAnim::WallSlide1].pick())
            .with_flip_x(anim.get_flip_x());
        let mut offset = Vec2::new(0.0, -4.0);
        if anim.get_flip_x() {
            offset.x *= -1.0;
        }
        commands.spawn((
            man,
            player_pos
                .translated(offset)
                // Make smokes closer to bottom on top, and (hopefully) give unique so no flicker
                .to_transform(ZIX_PLAYER - ((player_pos.y + 10000.0) / 100000.0)),
            Visibility::Inherited,
        ));
        commands
            .spawn((SoundEffect::PlayerWallSlide, OneSound::Ignore))
            .set_parent(sound_root.eid());
    }
}

fn player_impact_sounds(
    mut commands: Commands,
    player: Query<&StaticRxCtrl, With<Player>>,
    colls: Res<StaticColls>,
    consts: Res<PlayerJuiceConsts>,
    sound_root: Res<SoundRoot>,
) {
    let srx_ctrl = player.single();
    let mut plank_handled = false;
    for coll in colls.get_refs(&srx_ctrl.coll_keys) {
        if !matches!(
            coll.rx_kind,
            StaticRxKind::Default | StaticRxKind::DefaultBreaker
        ) {
            continue;
        }
        let mag = coll.rx_perp.length();
        if mag < consts.impact_sound_floor {
            continue;
        }
        if coll.tx_kind == StaticTxKind::PassUp && (plank_handled || coll.rx_perp.y > 0.0) {
            continue;
        } else {
            plank_handled = true;
        }

        let frac = ((mag - consts.impact_sound_floor)
            / (consts.impact_sound_ceiling - consts.impact_sound_floor))
            .min(1.0);
        commands
            .spawn((
                SoundEffect::PlayerImpactRegular,
                SoundMult(frac * consts.impact_sound_mult),
                OneSound::Ignore,
            ))
            .set_parent(sound_root.eid());
    }
}

fn head_smoke(
    player: Query<(&Pos, &AnimMan<PlayerAnim>), With<Player>>,
    mut commands: Commands,
    world_detail_root: Res<WorldDetailRoot>,
) {
    let Ok((pos, anim)) = player.get_single() else {
        return;
    };

    // Figure out the smoke pos
    let x_offset = match anim.get_state() {
        PlayerAnim::WallSlide => 1,
        _ => 0,
    } * if anim.get_flip_x() { -1 } else { 1 };
    let y_offset = match anim.get_state() {
        PlayerAnim::Squat => -1,
        _ => 1,
    };
    let smoke_pos = pos.translated(Vec2::new(x_offset as f32, y_offset as f32));

    // Spawn the particles always
    commands
        .spawn(EphemeralAnim::new(
            HeadSmokePartAnim::random(),
            thread_rng().gen_bool(0.5),
            smoke_pos,
            ZIX_PLAYER - 0.05,
        ))
        .set_parent(world_detail_root.eid());

    // Don't spawn anything else in these cases
    if matches!(
        anim.get_state(),
        PlayerAnim::EdgeSitting
            | PlayerAnim::EdgeSitup
            | PlayerAnim::AirDownExhausted
            | PlayerAnim::AirUpExhausted
            | PlayerAnim::WallSlideExhausted
            | PlayerAnim::WallJumpExhausted
            | PlayerAnim::Dash
            | PlayerAnim::Death
    ) {
        return;
    }

    // Otherwise we need the lil base of the thingy
    commands
        .spawn(EphemeralAnim::new(
            HeadSmokeAnim::HeadFull,
            anim.get_flip_x(),
            smoke_pos,
            ZIX_PLAYER - 0.06,
        ))
        .set_parent(world_detail_root.eid());
}

pub(super) fn register_player_juice(app: &mut App) {
    app.insert_resource(PlayerJuiceConsts::default());
    // debug_resource!(app, PlayerJuiceConsts);

    app.add_observer(juice_dash_suspense);
    app.add_observer(juice_dash_post_suspense);
    app.add_observer(juice_after_jump);
    app.add_observer(juice_animation_state_response);
    app.add_observer(juice_animation_ix_response);

    app.add_systems(
        Update,
        (
            juice_during_dash,
            juice_wall_slide,
            player_impact_sounds,
            head_smoke,
        )
            .in_set(PlayerSet)
            .after(PhysicsSet)
            .run_if(in_state(PlayerMetaState::Playing))
            .run_if(in_state(PhysicsState::Active))
            .run_if(in_state(PauseState::Unpaused)),
    );
}
