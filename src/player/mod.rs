use crate::prelude::*;

mod player_animation;
mod player_bundle;
mod player_death;
mod player_invariants;
mod player_juice;
mod player_level;
mod player_movement;
mod player_spawn;

// NOTE: Even though stuff in here is pub it's only available in player
// A.k.a I'm too lazy to write pub(super)
mod playerlude {
    use super::*;

    /// For handling all player movement
    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct PlayerMovementSet;

    /// For anything that can kill the player
    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct PlayerDeathSet;

    /// For handling all player animation
    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct PlayerAnimationSet;

    // Consts for helping figure out how the player is currently interacting with solids
    pub const PLAYER_MAIN_HBOX: u32 = 1;
    pub const PLAYER_RIGHT_HBOX: u32 = 2;
    pub const PLAYER_ABOVE_HBOX: u32 = 3;
    pub const PLAYER_LEFT_HBOX: u32 = 4;
    pub const PLAYER_BELOW_HBOX: u32 = 5;

    /// Can jump vertically off a ground below
    #[derive(Component, Clone, Debug, Reflect)]
    pub struct CanRegularJump {
        pub coyote_time: f32,
    }

    /// Can wall jump from left
    #[derive(Component, Clone, Debug, Reflect)]
    pub struct CanWallJumpFromLeft {
        pub coyote_time: f32,
    }

    /// Can wall jump from right
    #[derive(Component, Clone, Debug, Reflect)]
    pub struct CanWallJumpFromRight {
        pub coyote_time: f32,
    }

    #[derive(Component, Clone, Debug, Reflect)]
    pub struct PostJump {
        pub kind: JumpKind,
        pub time_left: f32,
    }

    #[derive(Component, Clone, Debug, Reflect)]
    pub struct ResponsiveJump {
        pub max_speed: f32,
        pub max_time: f32,
        pub time_left: f32,
        pub jump_applied: f32,
    }
    impl ResponsiveJump {
        pub fn new(max_speed: f32, max_time: f32) -> Self {
            Self {
                max_speed,
                max_time,
                time_left: max_time,
                jump_applied: 0.0,
            }
        }
    }

    #[derive(Component, Clone, Debug, Reflect)]
    pub struct CanDash;

    #[derive(Component, Clone, Debug, Reflect)]
    pub struct Dashing {
        pub time_left: f32,
    }

    #[derive(Clone, Copy, Debug, Reflect)]
    pub enum JumpKind {
        Regular,
        FromLeftWall,
        FromRightWall,
    }

    #[derive(Event, Clone, Debug, Reflect)]
    pub struct JumpEvent {
        pub kind: JumpKind,
    }

    #[derive(Event, Clone, Debug, Reflect)]
    pub struct DashEvent {
        pub dir: CardDir,
    }

    #[derive(Component, Clone, Debug, Default, Reflect)]
    pub struct TouchingDir {
        map: HashMap<Dir4, bool>,
    }
    impl TouchingDir {
        pub fn with_up(mut self, val: bool) -> Self {
            self.map.insert(Dir4::Up, val);
            self
        }
        pub fn with_down(mut self, val: bool) -> Self {
            self.map.insert(Dir4::Down, val);
            self
        }
        pub fn with_left(mut self, val: bool) -> Self {
            self.map.insert(Dir4::Left, val);
            self
        }
        pub fn with_right(mut self, val: bool) -> Self {
            self.map.insert(Dir4::Right, val);
            self
        }
        #[allow(dead_code)]
        pub fn up(&self) -> bool {
            *self.map.get(&Dir4::Up).unwrap_or(&false)
        }
        pub fn down(&self) -> bool {
            *self.map.get(&Dir4::Down).unwrap_or(&false)
        }
        pub fn left(&self) -> bool {
            *self.map.get(&Dir4::Left).unwrap_or(&false)
        }
        pub fn right(&self) -> bool {
            *self.map.get(&Dir4::Right).unwrap_or(&false)
        }
    }
    #[derive(Component, Default)]
    pub struct ForcefulTouchingDir {
        inner: TouchingDir,
    }
    impl ForcefulTouchingDir {
        pub fn set_up(&mut self, val: bool) {
            self.inner.map.insert(Dir4::Up, val);
        }
        pub fn set_down(&mut self, val: bool) {
            self.inner.map.insert(Dir4::Down, val);
        }
        pub fn set_left(&mut self, val: bool) {
            self.inner.map.insert(Dir4::Left, val);
        }
        pub fn set_right(&mut self, val: bool) {
            self.inner.map.insert(Dir4::Right, val);
        }
        #[allow(dead_code)]
        pub fn up(&self) -> bool {
            self.inner.up()
        }
        pub fn down(&self) -> bool {
            self.inner.down()
        }
        pub fn left(&self) -> bool {
            self.inner.left()
        }
        pub fn right(&self) -> bool {
            self.inner.right()
        }
    }
}

/// The set that contains all player related systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerSet;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct SpawnPointActive;

pub(super) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        reg_types!(
            app,
            playerlude::CanRegularJump,
            playerlude::CanWallJumpFromLeft,
            playerlude::CanWallJumpFromRight,
            playerlude::CanDash,
            playerlude::Dashing,
            playerlude::TouchingDir
        );

        player_animation::register_player_animation(app);
        player_death::register_player_death(app);
        player_juice::register_player_juice(app);
        player_invariants::register_player_invariants(app);
        player_level::register_player_level(app);
        player_movement::register_player_movement(app);
        player_spawn::register_player_spawn(app);
    }
}
