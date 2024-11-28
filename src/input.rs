use std::ops::Deref;

use bevy::input::gamepad::{GamepadConnection, GamepadEvent};

use crate::prelude::*;

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

fn update_input_from_keyboard(
    mut dir_input: ResMut<DirInput>,
    mut butt_input: ResMut<ButtInput>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut dir = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }
    dir_input.dir = dir.normalize_or_zero();

    macro_rules! impl_button {
        ($input:expr, $keyboard:expr, $button:ident, $key:ident) => {
            $input
                .pressed
                .insert(ButtKind::$button, $keyboard.pressed(KeyCode::$key));
            $input
                .just_pressed
                .insert(ButtKind::$button, $keyboard.just_pressed(KeyCode::$key));
        };
    }
    impl_button!(butt_input, keyboard, A, KeyJ);
    impl_button!(butt_input, keyboard, B, KeyK);
    impl_button!(butt_input, keyboard, Enter, Enter);
    impl_button!(butt_input, keyboard, Escape, Escape);
}

#[derive(Resource, Default)]
struct MyGamepad {
    gamepad: Option<Gamepad>,
}

fn update_my_gamepad(
    mut my_gamepad: ResMut<MyGamepad>,
    mut evr_gamepad: EventReader<GamepadEvent>,
) {
    for ev in evr_gamepad.read() {
        // we only care about connection events
        let GamepadEvent::Connection(ev_conn) = ev else {
            continue;
        };
        match &ev_conn.connection {
            GamepadConnection::Connected(_info) => {
                // Just always use this one (naive)
                my_gamepad.gamepad = Some(ev_conn.gamepad);
                println!("should be connecting {:?}", ev_conn.gamepad);
            }
            GamepadConnection::Disconnected => {
                // Just always assume we disconnected (naive)
                my_gamepad.gamepad = None;
                println!("should be disconnecting {:?}", ev_conn.gamepad);
            }
        }
    }
}

fn update_input_from_gamepad(
    my_gamepad: Res<MyGamepad>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<ButtonInput<GamepadButton>>,
) {
    let Some(gamepad) = my_gamepad.gamepad else {
        println!("bad no gamepad");
        return;
    };

    let axis_lx = GamepadAxis {
        gamepad,
        axis_type: GamepadAxisType::LeftStickX,
    };
    let axis_ly = GamepadAxis {
        gamepad,
        axis_type: GamepadAxisType::LeftStickY,
    };
    let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) else {
        println!("bad no axes");
        return;
    };
    let left_stick = Vec2::new(x, y);
    if left_stick.length() > 0.0 {
        println!("left_stick is {:?}", left_stick);
    } else {
        println!("what the {:?}", axes);
    }
}

fn gamepad_system(
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
) {
    println!("num of gamepads: {:?}", gamepads.iter().count());
    for gamepad in gamepads.iter() {
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
            info!("{:?} just pressed South", gamepad);
        } else if button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::South))
        {
            info!("{:?} just released South", gamepad);
        }

        let right_trigger = button_axes
            .get(GamepadButton::new(
                gamepad,
                GamepadButtonType::RightTrigger2,
            ))
            .unwrap();
        if right_trigger.abs() > 0.01 {
            info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
        }

        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x.abs() > 0.01 {
            info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
        }
    }
}

fn gamepad_input_events(mut evr_gamepad: EventReader<GamepadEvent>) {
    for ev in evr_gamepad.read() {
        match ev {
            GamepadEvent::Axis(ev_axis) => {
                println!(
                    "Axis {:?} on gamepad {:?} is now at {:?}",
                    ev_axis.axis_type, ev_axis.gamepad, ev_axis.value
                );
            }
            GamepadEvent::Button(ev_button) => {
                // The "value" of a button is typically `0.0` or `1.0`, but it
                // is a `f32` because some gamepads may have buttons that are
                // pressure-sensitive or otherwise analog somehow.
                println!(
                    "Button {:?} on gamepad {:?} is now at {:?}",
                    ev_button.button_type, ev_button.gamepad, ev_button.value
                );
            }
            _ => {
                // we don't care about other events here (connect/disconnect)
            }
        }
    }
}

pub(super) struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        reg_types!(app, ButtKind);

        app.insert_resource(DirInput::default());
        app.insert_resource(ButtInput::default());
        app.insert_resource(MyGamepad::default());

        app.add_systems(
            Update,
            (
                update_input_from_keyboard,
                // update_my_gamepad,
                // update_input_from_gamepad,
                // gamepad_system,
                // gamepad_input_events,
            )
                .in_set(InputSet),
        );
    }
}
