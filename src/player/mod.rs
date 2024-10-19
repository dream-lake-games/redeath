use crate::prelude::*;

mod player_animation;
mod player_bundle;
mod player_invariants;
mod player_movement;
mod scratch;
mod spawn;

// NOTE: Even though stuff in here is pub it's only available in player
// A.k.a I'm too lazy to write pub(super)
mod playerlude {
    use super::*;

    /// For handling all player movement
    #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
    pub struct PlayerMovementSet;

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
        pub event: JumpEvent,
        pub time_left: f32,
    }

    #[derive(Component, Clone, Debug, Reflect)]
    pub struct CanDash;

    #[derive(Component, Clone, Debug, Reflect)]
    pub struct Dashing {
        pub time_left: f32,
    }

    #[derive(Event, Clone, Copy, Debug, Reflect)]
    pub enum JumpEvent {
        Regular,
        FromLeftWall,
        FromRightWall,
    }

    #[derive(Event, Clone, Debug, Reflect)]
    pub struct DashEvent {
        dir: CardDir,
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
}

/// The set that contains all player related systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerSet;

#[derive(Component)]
pub struct Player;

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
        player_invariants::register_player_invariants(app);
        player_movement::register_player_movement(app);
        // scratch::register_scratch(app);
        spawn::register_spawn(app);
    }
}
