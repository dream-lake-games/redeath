use crate::prelude::*;

#[derive(Component, Clone)]
pub struct ChaseEntity {
    pub eid: Entity,
    pub acc: f32,
    // Deceleration as a speed to make it framerate-independent
    pub dec: f32,
    pub leash: f32,
    pub max_speed: f32,
}
impl ChaseEntity {
    pub fn new(eid: Entity, acc: f32, dec: f32, leash: f32, max_speed: f32) -> Self {
        Self {
            eid,
            acc,
            dec,
            leash,
            max_speed,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub enum ChaseState {
    InLeash,
    OutLeash,
    BadTarget,
}

fn chase_entities(
    mut chasers: Query<(Entity, &Pos, &mut Dyno, &ChaseEntity)>,
    pos_q: Query<&Pos>,
    bullet_time: Res<BulletTime>,
    mut commands: Commands,
) {
    let handle_decel = |vel: &mut Vec2, amt: f32| {
        if vel.length() < amt {
            *vel = Vec2::ZERO;
        } else {
            *vel = *vel - vel.normalize_or_zero() * amt;
        }
    };
    for (eid, chaser_pos, mut chaser_dyno, chase) in &mut chasers {
        let Ok(target_pos) = pos_q.get(chase.eid) else {
            if let Some(mut comms) = commands.get_entity(eid) {
                comms.try_insert(ChaseState::BadTarget);
                handle_decel(&mut chaser_dyno.vel, chase.acc * bullet_time.delta_secs());
            }
            continue;
        };
        // Always decellerate to avoid orbiting
        handle_decel(&mut chaser_dyno.vel, chase.dec * bullet_time.delta_secs());
        if chaser_pos.as_vec2().distance(target_pos.as_vec2()) < chase.leash {
            if let Some(mut comms) = commands.get_entity(eid) {
                comms.try_insert(ChaseState::InLeash);
                handle_decel(&mut chaser_dyno.vel, chase.acc * bullet_time.delta_secs());
            }
        } else {
            if let Some(mut comms) = commands.get_entity(eid) {
                comms.try_insert(ChaseState::OutLeash);
                let norm_diff = (target_pos.as_vec2() - chaser_pos.as_vec2()).normalize_or_zero();
                chaser_dyno.vel += norm_diff * chase.acc * bullet_time.delta_secs();
            }
        }
        // Always clamp max speeds
        chaser_dyno.vel = chaser_dyno.vel.clamp_length(0.0, chase.max_speed);
    }
}

pub(super) fn register_chase(app: &mut App) {
    app.add_systems(
        Update,
        (chase_entities,)
            .chain()
            .in_set(ChaseSet)
            .after(PhysicsSet)
            .run_if(in_state(MetaStateKind::World)),
    );
}
