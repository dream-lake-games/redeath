use crate::prelude::*;

#[derive(Component)]
pub struct Bob {
    a: Pos,
    b: Pos,
    scratch_time: f32,
    going_up: bool,
    total_time: f32,
}
impl Bob {
    pub fn vert(a: Pos, height: f32, total_time: f32) -> Self {
        Self {
            a,
            b: a.translated(Vec2::new(0.0, height)),
            scratch_time: 0.0,
            going_up: true,
            total_time,
        }
    }
}

fn update_bobs(mut bob_q: Query<(&mut Pos, &mut Bob)>, bullet_time: Res<BulletTime>) {
    for (mut pos, mut bob) in &mut bob_q {
        bob.scratch_time =
            bob.scratch_time + if bob.going_up { 1.0 } else { -1.0 } * bullet_time.delta_secs();
        let frac = (bob.scratch_time / bob.total_time).clamp(0.0, 1.0);
        if (bob.scratch_time <= 0.01 && !bob.going_up) || (bob.scratch_time >= 0.99 && bob.going_up)
        {
            bob.going_up = !bob.going_up;
        }
        let new_pos = bob.a.as_vec2()
            + (bob.b.as_vec2() - bob.a.as_vec2()) * Spleen::EaseInOutQuad.interp(frac);
        *pos = Pos::new(new_pos.x, new_pos.y);
    }
}

pub(super) fn register_bob(app: &mut App) {
    app.add_systems(Update, update_bobs.after(PhysicsSet));
}
