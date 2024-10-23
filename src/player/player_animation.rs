use super::playerlude::*;
use crate::prelude::*;

fn start_jump_animation(
    trigger: Trigger<JumpEvent>,
    mut player: Query<(&mut AnimMan<PlayerAnim>, Option<&CanDash>), With<Player>>,
) {
    let event = trigger.event();
    let (mut anim, can_dash) = player.single_mut();
    match (event.kind, can_dash) {
        (JumpKind::Regular, _) => {
            anim.set_state(PlayerAnim::Jump);
        }
        (JumpKind::FromLeftWall, Some(_)) => {
            anim.set_flip_x(false);
            anim.set_state(PlayerAnim::WallJump);
        }
        (JumpKind::FromLeftWall, None) => {
            anim.set_flip_x(false);
            anim.set_state(PlayerAnim::WallJumpExhausted);
        }
        (JumpKind::FromRightWall, Some(_)) => {
            anim.set_flip_x(true);
            anim.set_state(PlayerAnim::WallJump);
        }
        (JumpKind::FromRightWall, None) => {
            anim.set_flip_x(true);
            anim.set_state(PlayerAnim::WallJumpExhausted);
        }
    }
}

fn maybe_start_land_animation(
    mut player: Query<(&mut AnimMan<PlayerAnim>, &TouchingDir), With<Player>>,
) {
    let (mut anim, touching) = player.single_mut();
    if matches!(
        anim.get_state(),
        PlayerAnim::AirDown | PlayerAnim::AirDownExhausted
    ) && touching.down()
    {
        anim.set_state(PlayerAnim::Land);
    }
}

fn normal_movement_animation(
    mut player: Query<
        (
            &mut AnimMan<PlayerAnim>,
            &TouchingDir,
            &Dyno,
            Option<&Dashing>,
            Option<&CanDash>,
        ),
        With<Player>,
    >,
    dir: Res<DirInput>,
) {
    let (mut anim, touching, dyno, dashing, can_dash) = player.single_mut();

    // Dashing overrides everything
    if dashing.is_some() {
        anim.set_state(PlayerAnim::Dash);
        return;
    }

    // Then wall sliding (note anim logic is slightly different from vel logic — be warned)
    // Okay it's actually not that scary it's just that anim also comes into play when sliding and moving up,
    // whereas vel only changes when going down
    let wall_sliding = !touching.down()
        && (touching.right() && dyno.vel.x > 0.0 || touching.left() && dyno.vel.x < 0.0);
    if wall_sliding {
        anim.set_flip_x(touching.right());
        if can_dash.is_some() {
            anim.set_state(PlayerAnim::WallSlide);
        } else {
            anim.set_state(PlayerAnim::WallSlideExhausted);
        }
        return;
    }

    // Then don't interrupt stuff
    if matches!(
        anim.get_state(),
        PlayerAnim::Jump | PlayerAnim::WallJump | PlayerAnim::WallJumpExhausted | PlayerAnim::Land
    ) {
        // Don't interrupt these animations for normal movement
        if dyno.vel.x.abs() > 1.0 {
            anim.set_flip_x(dyno.vel.x < 0.0);
        }
        return;
    }

    // Finally do running and inair if we get here
    if touching.down() {
        // On the ground
        if dyno.vel.x.abs() < 1.0 {
            // Not moving
            if dir.y < 0.0 {
                anim.set_state(PlayerAnim::Squat);
            } else {
                anim.set_state(PlayerAnim::Stand);
            }
        } else {
            // Moving
            let pushing_wall =
                (dyno.vel.x > 0.0 && touching.right()) || (dyno.vel.x < 0.0 && touching.left());
            if pushing_wall {
                anim.set_state(PlayerAnim::WallPush);
            } else {
                anim.set_state(PlayerAnim::Run);
            }
            anim.set_flip_x(dyno.vel.x < 0.0);
        }
    } else {
        // In the air
        if dyno.vel.x.abs() > 1.0 {
            anim.set_flip_x(dyno.vel.x < 0.0);
        }
        anim.set_state(if dyno.vel.y > 0.0 {
            if can_dash.is_some() {
                PlayerAnim::AirUp
            } else {
                PlayerAnim::AirUpExhausted
            }
        } else {
            if can_dash.is_some() {
                PlayerAnim::AirDown
            } else {
                PlayerAnim::AirDownExhausted
            }
        });
    }
}

pub(super) fn register_player_animation(app: &mut App) {
    // Events
    app.observe(start_jump_animation);
    // Normal stuff
    app.add_systems(
        Update,
        (maybe_start_land_animation, normal_movement_animation)
            .chain()
            .before(AnimSet)
            .in_set(PlayerSet)
            .in_set(PlayerAnimationSet)
            .after(PlayerDeathSet)
            .after(InputSet)
            .after(PhysicsSet)
            .run_if(in_state(PlayerMetaState::Puppet).or_else(in_state(PlayerMetaState::Playing))),
    );
}
