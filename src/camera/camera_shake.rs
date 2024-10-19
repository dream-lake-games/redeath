use std::ops::RangeInclusive;

use crate::prelude::*;

const SHAKE_EVERY: f32 = 0.05;

#[derive(Clone, Debug, Reflect)]
struct CameraShakeSpec {
    time_left: f32,
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

#[derive(Resource, Debug, Reflect)]
pub struct CameraShake {
    specs: Vec<CameraShakeSpec>,
    offset: Vec2,
    time_since_last_update: f32,
}
impl CameraShake {
    pub fn shake(&mut self, time: f32, x_range: RangeInclusive<i32>, y_range: RangeInclusive<i32>) {
        self.specs.push(CameraShakeSpec {
            time_left: time,
            x_range,
            y_range,
        });
    }

    pub fn get_offset(&self) -> Vec2 {
        self.offset
    }
}

fn update_camera_shake(mut camera_shake: ResMut<CameraShake>, time: Res<Time>) {
    // Obey SHAKE_EVERY
    camera_shake.time_since_last_update += time.delta_seconds();
    if camera_shake.time_since_last_update < SHAKE_EVERY {
        return;
    }
    camera_shake.time_since_last_update = 0.0;

    // Calculate offset
    let mut offset = Vec2::ZERO;
    let mut rng = thread_rng();
    for spec in &mut camera_shake.specs {
        spec.time_left -= SHAKE_EVERY;
        offset.x += rng.gen_range(spec.x_range.clone()) as f32;
        offset.y += rng.gen_range(spec.y_range.clone()) as f32;
    }
    camera_shake.offset = offset;

    // Cleanup specs
    camera_shake.specs.retain(|spec| spec.time_left > 0.0);
}

pub(super) fn register_camera_shake(app: &mut App) {
    app.insert_resource(CameraShake {
        specs: vec![],
        offset: Vec2::ZERO,
        time_since_last_update: 0.0,
    });

    app.add_systems(Update, update_camera_shake);
}
