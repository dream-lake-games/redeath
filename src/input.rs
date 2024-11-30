use std::ops::Deref;

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

pub(super) struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        reg_types!(app, ButtKind);

        app.insert_resource(DirInput::default());
        app.insert_resource(ButtInput::default());

        app.add_systems(Update, (update_input_from_keyboard,).in_set(InputSet));
    }
}
