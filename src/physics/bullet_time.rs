use bevy::ecs::schedule::ScheduleLabel;

use crate::prelude::*;

/// A schedule that will run every FRAMERATE of IN-GAME time
/// So things like gravity and drag will be applied consistently in and out of bullet time
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BulletUpdate;

#[derive(Clone, Copy, Debug, Reflect)]
pub enum BulletTimeSpeed {
    Normal,
    Slow,
    Stopped,
}
impl BulletTimeSpeed {
    fn to_factor(&self) -> f32 {
        match self {
            Self::Normal => 1.0,
            Self::Slow => 0.2,
            Self::Stopped => 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Reflect)]
pub enum BulletTimeMode {
    Stable {
        speed: BulletTimeSpeed,
    },
    Temp {
        speed: BulletTimeSpeed,
        time_left: f32,
    },
}
impl BulletTimeMode {
    fn to_factor(&self) -> f32 {
        match self {
            BulletTimeMode::Stable { speed } | BulletTimeMode::Temp { speed, .. } => {
                speed.to_factor()
            }
        }
    }
}

/// How much in-game time has happened. Basically time but accounts for slowdown.
#[derive(Resource, Debug, Clone, Reflect)]
pub struct BulletTime {
    mode: BulletTimeMode,
    duration: Duration,
}
impl BulletTime {
    pub fn new() -> Self {
        Self {
            mode: BulletTimeMode::Stable {
                speed: BulletTimeSpeed::Normal,
            },
            duration: Duration::ZERO,
        }
    }

    pub fn delta(&self) -> Duration {
        self.duration
    }
    pub fn delta_seconds(&self) -> f32 {
        self.duration.as_secs_f32()
    }

    pub fn reset(&mut self) {
        self.mode = BulletTimeMode::Stable {
            speed: BulletTimeSpeed::Normal,
        }
    }
    pub fn set_stable(&mut self, speed: BulletTimeSpeed) {
        self.mode = BulletTimeMode::Stable { speed }
    }
    pub fn set_temp(&mut self, speed: BulletTimeSpeed, time: f32) {
        self.mode = BulletTimeMode::Temp {
            speed,
            time_left: time,
        }
    }
}

fn update_bullet_time(mut bullet_time: ResMut<BulletTime>, time: Res<Time>) {
    let reset = match &mut bullet_time.mode {
        BulletTimeMode::Stable { .. } => false,
        BulletTimeMode::Temp { time_left, .. } => {
            *time_left -= time.delta_seconds();
            if *time_left < 0.0 {
                true
            } else {
                false
            }
        }
    };
    if reset {
        bullet_time.reset();
    }
    bullet_time.duration = time.delta().mul_f32(bullet_time.mode.to_factor());
}

// TODO: I think this is bad
// #[derive(Resource)]
// struct InGameTimePassed(f32);
// fn shephard_bullet_update(world: &mut World) {
//     let in_game_time = world.resource::<BulletTime>().delta_seconds();
//     let mut time_passed = world.resource_mut::<InGameTimePassed>();
//     time_passed.0 += in_game_time;
//     let mut num_runs = 0;
//     while time_passed.0 >= 1.0 / FRAMERATE {
//         time_passed.0 -= 1.0 / FRAMERATE;
//         num_runs += 1;
//     }
//     for _ in 0..num_runs {
//         world.run_schedule(BulletUpdate);
//     }
// }

pub(super) struct BulletTimePlugin;
impl Plugin for BulletTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BulletTime::new());
        app.add_systems(First, update_bullet_time);

        // app.init_schedule(BulletUpdate);
        // app.insert_resource(InGameTimePassed(0.0));
        // app.add_systems(Update, shephard_bullet_update.in_set(PhysicsSet));
    }
}
