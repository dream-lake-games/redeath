use crate::prelude::*;

#[derive(Component)]
struct CloudXVel(f32);

#[derive(Bundle)]
pub struct BgClouds {
    name: Name,
    psi: ParallaxScreenImage,
    could_x_vel: CloudXVel,
}
impl BgClouds {
    pub fn new(path: &str, size_x: u32, size_y: u32, zix: f32, mult: f32, x_vel: f32) -> Self {
        Self {
            name: Name::new("bg_close"),
            psi: ParallaxScreenImage::new_bg(path, size_x, size_y, zix).with_parallax_x(mult),
            could_x_vel: CloudXVel(x_vel),
        }
    }
}

fn update_bg_clouds(mut clouds: Query<(&mut Transform, &CloudXVel)>, bullet_time: Res<BulletTime>) {
    for (mut tran, vel) in &mut clouds {
        tran.translation.x += bullet_time.delta_seconds() * vel.0;
    }
}

pub(super) fn register_bg_clouds(app: &mut App) {
    app.add_systems(Update, update_bg_clouds);
}
