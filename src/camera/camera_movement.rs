use crate::prelude::*;

fn move_camera(
    mut dynamic_camera: Query<&mut Pos, With<DynamicCamera>>,
    ipos_q: Query<&IPos>,
    mut camera_mode: ResMut<DynamicCameraMode>,
) {
    let Ok(mut pos) = dynamic_camera.get_single_mut() else {
        warn!("yikes move_camera");
        return;
    };

    // First handle mode specific movement
    match *camera_mode {
        DynamicCameraMode::Follow(eid) => {
            match ipos_q.get(eid) {
                Ok(ipos) => {
                    pos.x = ipos.cur.x as f32;
                    pos.y = ipos.cur.y as f32;
                }
                Err(e) => {
                    warn!("Camera following non-existent entity. Going to hang, {e:?}");
                    *camera_mode = DynamicCameraMode::Hanging;
                }
            };
        }
        DynamicCameraMode::Hanging => (),
    }
}

fn follow_dynamic_camera(
    dynamic_camera: Query<&Pos, With<DynamicCamera>>,
    mut followers: Query<&mut Transform, (With<FollowDynamicCamera>, Without<DynamicCamera>)>,
    // camera_shake: Res<CameraShakeOffset>,
) {
    let Ok(leader) = dynamic_camera.get_single() else {
        warn!("yikes followdynamic");
        return;
    };
    for mut tran in &mut followers {
        tran.translation.x = leader.x;
        tran.translation.y = leader.y;
        // tran.translation.x = leader.x + camera_shake.offset.x as f32;
        // tran.translation.y = leader.y + camera_shake.offset.y as f32;
    }
}

pub(super) fn register_camera_movement(app: &mut App) {
    app.add_systems(
        PostUpdate,
        (move_camera, follow_dynamic_camera)
            .chain()
            .in_set(CameraSet)
            .after(PhysicsSet),
    );
}
