//! ALL OTHER PLACES IN THE GAME SHOULD NOT READ RAW INPUT
//! They should operate only off the resources + events here
//! Otherwise it'll be impossible to do keybindings or multiple input modes
//! (Except some evil dev hacking keyboard stuff whoops)

use crate::prelude::*;

use std::ops::Deref;

mod gamepad;
mod keyboard;

#[derive(Resource, Default, Clone)]
pub enum InputMode {
    #[default]
    Keyboard,
    Gamepad(Entity),
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
    buffered_just_pressed: HashMap<ButtKind, f32>,
}
impl ButtInput {
    pub fn pressed(&self, butt: ButtKind) -> bool {
        self.pressed.get(&butt).cloned().unwrap_or(false)
    }
    pub fn just_pressed(&self, butt: ButtKind) -> bool {
        self.just_pressed.get(&butt).cloned().unwrap_or(false)
    }
    pub fn buffered_just_pressed(&self, butt: ButtKind) -> bool {
        self.buffered_just_pressed.contains_key(&butt)
    }
}

fn update_input_mode(mut input_mode: ResMut<InputMode>, gamepads: Query<Entity, With<Gamepad>>) {
    match input_mode.clone() {
        InputMode::Keyboard => {
            if !gamepads.is_empty() {
                *input_mode = InputMode::Gamepad(gamepads.iter().next().unwrap());
            }
        }
        InputMode::Gamepad(eid) => {
            if !gamepads.contains(eid) {
                *input_mode = InputMode::Keyboard;
            }
        }
    }
}

const BUFFER_TIME: f32 = 0.1;
fn update_buffered_shit(mut butt_input: ResMut<ButtInput>, time: Res<Time>) {
    for butt in [ButtKind::A, ButtKind::B] {
        if butt_input.just_pressed(butt) {
            butt_input.buffered_just_pressed.insert(butt, BUFFER_TIME);
        } else {
            if let Some(mr) = butt_input.buffered_just_pressed.get_mut(&butt) {
                *mr -= time.delta_secs();
            }
            if let Some(r) = butt_input.buffered_just_pressed.get(&butt) {
                if *r <= 0.0 {
                    butt_input.buffered_just_pressed.remove(&butt);
                }
            }
        }
    }
}

pub(super) struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        reg_types!(app, ButtKind);

        app.insert_resource(InputMode::default());
        app.insert_resource(DirInput::default());
        app.insert_resource(ButtInput::default());

        app.add_systems(Update, update_input_mode.in_set(InputSet));
        app.add_systems(Update, update_buffered_shit.in_set(InputSet));

        gamepad::register_gamepad(app);
        keyboard::register_keyboard(app);
    }
}
