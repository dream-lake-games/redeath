use super::playerlude::*;
use crate::prelude::*;

#[derive(Resource, Clone, Debug, Reflect)]
struct PlayerJuiceConsts {
    /// Minimum speed of impact before spawning a sound
    impact_sound_floor: f32,
    /// Above this speed, sound of impact will be the same
    impact_sound_ceiling: f32,
    impact_sound_mult: f32,
}
impl Default for PlayerJuiceConsts {
    fn default() -> Self {
        Self {
            impact_sound_floor: 2.0,
            impact_sound_ceiling: 144.0,
            impact_sound_mult: 0.05,
        }
    }
}

fn juice_after_dash(trigger: Trigger<DashEvent>, mut camera_shake: ResMut<CameraShake>) {
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

fn juice_after_jump(
    trigger: Trigger<JumpEvent>,
    player: Query<&Pos, With<Player>>,
    mut commands: Commands,
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
    let mut man = AnimMan::new(anim);
    if flipx {
        man = man.with_flip_x();
    }
    commands.spawn((man, smoke_pos.to_spatial(ZIX_PLAYER + 1.0)));

    // Sound
    commands.spawn(SoundEffect::PlayerJump);
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
            let mut man = AnimMan::new(JumpSmokeAnim::Land1);
            if anim.get_flip_x() {
                man = man.with_flip_x();
            }
            commands.spawn((
                man,
                pos.translated(Vec2::new(0.0, -3.0))
                    .to_spatial(ZIX_PLAYER + 1.0),
            ));
        }
        _ => {}
    }
}

fn juice_animation_ix_response(
    trigger: Trigger<AnimIxChange<PlayerAnim>>,
    player: Query<(&Pos, &AnimMan<PlayerAnim>), With<Player>>,
    mut commands: Commands,
) {
    let event = trigger.event();
    let (pos, anim) = player.single();
    match (event.state, event.ix) {
        (PlayerAnim::Run, 3) => {
            let offset = Vec2::new(if anim.get_flip_x() { -1.0 } else { 1.0 } * 5.0, -3.0);
            commands.spawn((
                AnimMan::new(RunSmokeAnim::Run1),
                pos.translated(offset).to_spatial(ZIX_PLAYER + 1.01),
            ));
            commands.spawn(SoundEffect::PlayerRunStep);
        }
        _ => {}
    }
}

fn juice_wall_slide(
    player: Query<(&AnimMan<PlayerAnim>, &Pos), With<Player>>,
    mut commands: Commands,
) {
    let (anim, player_pos) = player.single();
    if matches!(
        anim.get_state(),
        PlayerAnim::WallSlide | PlayerAnim::WallSlideExhausted
    ) {
        let mut man = AnimMan::new(vec![WallSlideSmokeAnim::WallSlide1].pick());
        let mut offset = Vec2::new(0.0, -4.0);
        if anim.get_flip_x() {
            man = man.with_flip_x();
            offset.x *= -1.0;
        }
        commands.spawn((
            man,
            player_pos
                .translated(offset)
                // Make smokes closer to bottom on top, and (hopefully) give unique so no flicker
                .to_spatial(ZIX_PLAYER - ((player_pos.y + 10000.0) / 100000.0)),
        ));
        commands.spawn((SoundEffect::PlayerWallSlide, OneSound::Ignore));
    }
}

fn player_impact_sounds(
    mut commands: Commands,
    player: Query<&StaticRxCtrl, With<Player>>,
    colls: Res<StaticColls>,
    consts: Res<PlayerJuiceConsts>,
) {
    let srx_ctrl = player.single();
    for coll in colls.get_refs(&srx_ctrl.coll_keys) {
        let mag = coll.rx_perp.length();
        if mag < consts.impact_sound_floor {
            continue;
        }
        let frac = ((mag - consts.impact_sound_floor)
            / (consts.impact_sound_ceiling - consts.impact_sound_floor))
            .min(1.0);
        commands.spawn((
            SoundEffect::PlayerImpactRegular,
            SoundMult(frac * consts.impact_sound_mult),
            OneSound::Ignore,
        ));
    }
}

pub(super) fn register_player_juice(app: &mut App) {
    app.insert_resource(PlayerJuiceConsts::default());
    debug_resource!(app, PlayerJuiceConsts);

    app.observe(juice_after_dash);
    app.observe(juice_after_jump);
    app.observe(juice_animation_state_response);
    app.observe(juice_animation_ix_response);

    app.add_systems(
        Update,
        (juice_wall_slide, player_impact_sounds)
            .in_set(PlayerSet)
            .after(PhysicsSet)
            .run_if(in_state(PlayerMetaState::Playing).or_else(in_state(PlayerMetaState::Puppet))),
    );
}
