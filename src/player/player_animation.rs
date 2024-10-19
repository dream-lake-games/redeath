use super::playerlude::*;
use crate::prelude::*;

fn start_jump_animation(
    trigger: Trigger<JumpEvent>,
    mut player: Query<&mut AnimMan<PlayerAnim>, With<Player>>,
) {
    let event = trigger.event();
    let mut anim = player.single_mut();
    match event.kind {
        JumpKind::Regular => {
            anim.set_state(PlayerAnim::Jump);
        }
        JumpKind::FromLeftWall => {
            anim.set_flip_x(false);
            anim.set_state(PlayerAnim::WallJump);
        }
        JumpKind::FromRightWall => {
            anim.set_flip_x(true);
            anim.set_state(PlayerAnim::WallJump);
        }
    }
}

fn maybe_start_land_animation(
    mut player: Query<(&mut AnimMan<PlayerAnim>, &TouchingDir), With<Player>>,
) {
    let (mut anim, touching) = player.single_mut();
    if matches!(anim.get_state(), PlayerAnim::AirUp | PlayerAnim::AirDown) && touching.down() {
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
        ),
        With<Player>,
    >,
    dir: Res<DirInput>,
) {
    let (mut anim, touching, dyno, dashing) = player.single_mut();

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
        anim.set_state(PlayerAnim::WallSlide);
        return;
    }

    // Then don't interrupt stuff
    if matches!(anim.get_state(), PlayerAnim::Jump | PlayerAnim::Land) {
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
            anim.set_state(PlayerAnim::Run);
            anim.set_flip_x(dyno.vel.x < 0.0);
        }
    } else {
        // In the air
        if dyno.vel.x.abs() > 1.0 {
            anim.set_flip_x(dyno.vel.x < 0.0);
        }
        anim.set_state(if dyno.vel.y > 0.0 {
            PlayerAnim::AirUp
        } else {
            PlayerAnim::AirDown
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
            .after(PlayerMovementSet)
            .after(InputSet)
            .after(PhysicsSet)
            .run_if(
                in_state(PlayerMetaState::Puppet)
                    .or_else(in_state(PlayerMetaState::Playing))
                    .or_else(in_state(PlayerMetaState::Dying)),
            ),
    );
}
