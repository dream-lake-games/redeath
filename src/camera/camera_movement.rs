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

pub fn camera_clamp_logic(pos: &Pos, rect: &Rect) -> Pos {
    Pos::new(
        f32::min(
            f32::max(pos.x, rect.min.x + SCREEN_WIDTH_f32 / 2.0),
            rect.max.x - SCREEN_WIDTH_f32 / 2.0,
        ),
        f32::min(
            f32::max(pos.y, rect.min.y + SCREEN_HEIGHT_f32 / 2.0),
            rect.max.y - SCREEN_HEIGHT_f32 / 2.0,
        ),
    )
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
    *pos = camera_clamp_logic(&pos, &level_rects.current.unwrap_or_default());
}

fn update_level_scroll(
    level_scroll: Res<State<LevelScrollState>>,
    mut next_level_scroll_state: ResMut<NextState<LevelScrollState>>,
    time: Res<Time>,
    mut dynamic_camera: Query<&mut Pos, With<DynamicCamera>>,
) {
    let Ok(mut cam_pos) = dynamic_camera.get_single_mut() else {
        warn!("yikes move_camera");
        return;
    };
    let Some(mut current_inner) = level_scroll.get().clone().active else {
        panic!("shouldn't be here");
    };
    // Move the camera
    current_inner.time_milli += time.delta().as_millis() as u32;
    let transition_time = 250_u32;
    let frac = current_inner.time_milli as f32 / transition_time as f32;
    let current_pos = Pos::new(
        Spleen::EaseInOutCubic.bound_interp(
            frac,
            current_inner.from_pos.x as f32,
            current_inner.to_pos.x as f32,
        ),
        Spleen::EaseInOutCubic.bound_interp(
            frac,
            current_inner.from_pos.y as f32,
            current_inner.to_pos.y as f32,
        ),
    );
    cam_pos.x = current_pos.x.round();
    cam_pos.y = current_pos.y.round();
    // Maybe end the transition
    if current_inner.time_milli > transition_time {
        next_level_scroll_state.set(LevelScrollState { active: None });
    } else {
        next_level_scroll_state.set(LevelScrollState {
            active: Some(current_inner),
        });
    }
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
        (move_camera, clamp_camera_in_level)
            .chain()
            .in_set(CameraSet)
            .after(PhysicsSet)
            .run_if(in_state(LevelScrollStateKind::None)),
    );
    app.add_systems(
        Update,
        (update_level_scroll,)
            .chain()
            .in_set(CameraSet)
            .after(PhysicsSet)
            .run_if(in_state(LevelScrollStateKind::Some)),
    );
    app.add_systems(
        PostUpdate,
        (follow_dynamic_camera,)
            .chain()
            .in_set(CameraSet)
            .after(PhysicsSet)
            .after(clamp_camera_in_level),
    );
}
