use super::playerlude::*;
use crate::prelude::*;

fn start_jump_animation(
    trigger: Trigger<JumpEvent>,
    mut player: Query<&mut AnimMan<PlayerAnim>, With<Player>>,
) {
    let event = trigger.event();
    let mut anim = player.single_mut();
    anim.set_state(PlayerAnim::Jump);
    match event {
        JumpEvent::Regular => {}
        JumpEvent::FromLeftWall => {
            anim.set_flip_x(false);
        }
        JumpEvent::FromRightWall => {
            anim.set_flip_x(true);
        }
    }
}

fn maybe_start_land_animation(
    mut player: Query<(&mut AnimMan<PlayerAnim>, &TouchingDir), With<Player>>,
) {
    let (mut anim, touching) = player.single_mut();
    if matches!(anim.get_state(), PlayerAnim::MidAir) && touching.down() {
        anim.set_state(PlayerAnim::Land);
    }
}

fn normal_movement_animation(
    mut player: Query<(&mut AnimMan<PlayerAnim>, &TouchingDir, &Dyno), With<Player>>,
    dir: Res<DirInput>,
) {
    let (mut anim, touching, dyno) = player.single_mut();

    if matches!(anim.get_state(), PlayerAnim::Jump | PlayerAnim::Land) {
        // Don't interrupt these animations for normal movement
        if dyno.vel.x.abs() > 1.0 {
            anim.set_flip_x(dyno.vel.x < 0.0);
        }
        return;
    }

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
        anim.set_state(PlayerAnim::MidAir);
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
