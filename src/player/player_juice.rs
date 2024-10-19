use super::playerlude::*;
use crate::prelude::*;

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
    let player_pos = player.single();
    let event = trigger.event();
    let (anim, offset, flipx) = match event.kind {
        JumpKind::Regular => (vec![JumpAnim::Regular1].pick(), Vec2::new(0.0, -3.0), false),
        JumpKind::FromLeftWall => (vec![JumpAnim::Wall1].pick(), Vec2::new(0.0, 0.0), false),
        JumpKind::FromRightWall => (vec![JumpAnim::Wall1].pick(), Vec2::new(0.0, 0.0), true),
    };
    let smoke_pos = player_pos.translated(offset);
    let mut man = AnimMan::new(anim);
    if flipx {
        man = man.with_flip_x();
    }
    commands.spawn((man, smoke_pos.to_spatial(ZIX_PLAYER + 1.0)));
}

pub(super) fn register_player_juice(app: &mut App) {
    app.observe(juice_after_dash);
    app.observe(juice_after_jump);
}
