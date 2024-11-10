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

pub fn camera_clamp_logic(pos: &mut Pos, level_rects: &LevelRects) {
    let Some(bounds) = level_rects.current else {
        return;
    };
    pos.x = pos.x.clamp(
        bounds.min.x + SCREEN_WIDTH_f32 / 2.0,
        bounds.max.x - SCREEN_WIDTH_f32 / 2.0,
    );
    pos.y = pos.y.clamp(
        bounds.min.y + SCREEN_HEIGHT_f32 / 2.0,
        bounds.max.y - SCREEN_HEIGHT_f32 / 2.0,
    );
}

// No matter the mode, always keep the camera inside the current level if there's a level
fn clamp_camera_in_level(
    mut dynamic_camera: Query<&mut Pos, With<DynamicCamera>>,
    level_rects: Res<LevelRects>,
) {
    let Ok(mut pos) = dynamic_camera.get_single_mut() else {
        warn!("yikes clamp_camera_in_level");
        return;
    };
    camera_clamp_logic(&mut pos, &level_rects);
}

fn follow_dynamic_camera(
    dynamic_camera: Query<&Pos, With<DynamicCamera>>,
    mut followers: Query<&mut Transform, (With<FollowDynamicCamera>, Without<DynamicCamera>)>,
    camera_shake: Res<CameraShake>,
) {
    let Ok(leader) = dynamic_camera.get_single() else {
        warn!("yikes followdynamic");
        return;
    };
    for mut tran in &mut followers {
        tran.translation.x = leader.x + camera_shake.get_offset().x;
        tran.translation.y = leader.y + camera_shake.get_offset().y;
    }
}

pub(super) fn register_camera_movement(app: &mut App) {
    app.add_systems(
        Update,
        (move_camera, clamp_camera_in_level, follow_dynamic_camera)
            .chain()
            .in_set(CameraSet)
            .after(PhysicsSet),
    );
}
