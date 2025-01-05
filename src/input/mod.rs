//! ALL OTHER PLACES IN THE GAME SHOULD NOT READ RAW INPUT
//! They should operate only off the resources + events here
//! Otherwise it'll be impossible to do keybindings or multiple input modes
//! (Except some evil dev hacking keyboard stuff whoops)

use bevy::input::gamepad::{GamepadConnection, GamepadEvent};

use crate::prelude::*;

use std::ops::Deref;

mod gamepad;
mod keyboard;

#[derive(Resource, Default)]
pub enum InputMode {
    #[default]
    Keyboard,
    Gamepad(Gamepad),
}

fn in_keyboard_mode(input_mode: Res<InputMode>) -> bool {
    matches!(input_mode.into_inner(), InputMode::Keyboard)
}

fn in_gamepad_mode(input_mode: Res<InputMode>) -> bool {
    matches!(input_mode.into_inner(), InputMode::Gamepad(_))
}

/// The set that contains all input
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct DirInput {
    dir: Vec2,
}
impl Deref for DirInput {
    type Target = Vec2;
    fn deref(&self) -> &Self::Target {
        &self.dir
    }
}
impl DirInput {
    pub fn as_vec2(&self) -> Vec2 {
        self.dir
    }
}

#[derive(Clone, Copy, Debug, Reflect, PartialEq, Eq, std::hash::Hash)]
pub enum ButtKind {
    A,
    B,
    Enter,
    Escape,
}

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct ButtInput {
    // A little cursed but it's fine
    pressed: HashMap<ButtKind, bool>,
    just_pressed: HashMap<ButtKind, bool>,
}
impl ButtInput {
    pub fn pressed(&self, butt: ButtKind) -> bool {
        self.pressed[&butt].clone()
    }
    pub fn just_pressed(&self, butt: ButtKind) -> bool {
        self.just_pressed[&butt].clone()
    }
}

// pomegranate
// fn update_input_mode(
//     mut input_mode: ResMut<InputMode>,
//     mut evr_gamepad: EventReader<GamepadEvent>,
// ) {
//     for ev in evr_gamepad.read() {
//         // we only care about connection events
//         let GamepadEvent::Connection(ev_conn) = ev else {
//             continue;
//         };
//         match &ev_conn.connection {
//             GamepadConnection::Connected(info) => {
//                 debug!(
//                     "New gamepad connected: {:?}, name: {}",
//                     ev_conn.gamepad, info.name,
//                 );
//                 *input_mode = InputMode::Gamepad(ev_conn.gamepad);
//             }
//             GamepadConnection::Disconnected => {
//                 debug!("Lost connection with gamepad: {:?}", ev_conn.gamepad);
//                 *input_mode = InputMode::Keyboard;
//             }
//         }
//     }
// }

pub(super) struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        reg_types!(app, ButtKind);

        app.insert_resource(InputMode::default());
        app.insert_resource(DirInput::default());
        app.insert_resource(ButtInput::default());

        // app.add_systems(Update, update_input_mode.in_set(InputSet));

        // gamepad::register_gamepad(app);
        keyboard::register_keyboard(app);
    }
}
