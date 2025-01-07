use crate::{input::in_keyboard_mode, prelude::*};

#[derive(Resource, Reflect, Serialize, Deserialize)]
pub struct BindingsKeyboard {
    up: KeyCode,
    right: KeyCode,
    down: KeyCode,
    left: KeyCode,
    a: KeyCode,
    b: KeyCode,
    enter: KeyCode,
    escape: KeyCode,
}
impl Default for BindingsKeyboard {
    fn default() -> Self {
        Self {
            up: KeyCode::KeyW,
            right: KeyCode::KeyD,
            down: KeyCode::KeyS,
            left: KeyCode::KeyA,
            a: KeyCode::KeyJ,
            b: KeyCode::KeyK,
            enter: KeyCode::Enter,
            escape: KeyCode::Escape,
        }
    }
}
impl Persable for BindingsKeyboard {
    const KEY: &'static str = "BindingsKeyboard";
}
impl BindingsKeyboard {
    fn celestey() -> Self {
        Self {
            up: KeyCode::ArrowUp,
            right: KeyCode::ArrowRight,
            down: KeyCode::ArrowDown,
            left: KeyCode::ArrowLeft,
            a: KeyCode::KeyC,
            b: KeyCode::KeyX,
            enter: KeyCode::Enter,
            escape: KeyCode::Escape,
        }
    }
}

/// TODO: Eventually you'll make a proper UI for this
/// ...right?
fn hacky_prebuilts(mut binds: ResMut<Pers<BindingsKeyboard>>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.all_pressed([KeyCode::KeyA, KeyCode::KeyW, KeyCode::KeyD]) {
        binds.set(BindingsKeyboard::default());
    } else if keyboard.all_pressed([KeyCode::ArrowLeft, KeyCode::ArrowUp, KeyCode::ArrowRight]) {
        binds.set(BindingsKeyboard::celestey());
    }
}

fn update_input_from_keyboard(
    mut dir_input: ResMut<DirInput>,
    mut butt_input: ResMut<ButtInput>,
    keyboard: Res<ButtonInput<KeyCode>>,
    binds: Res<Pers<BindingsKeyboard>>,
) {
    let mut dir = Vec2::ZERO;
    let binds = binds.get();
    if keyboard.pressed(binds.up) {
        dir.y += 1.0;
    }
    if keyboard.pressed(binds.down) {
        dir.y -= 1.0;
    }
    if keyboard.pressed(binds.left) {
        dir.x -= 1.0;
    }
    if keyboard.pressed(binds.right) {
        dir.x += 1.0;
    }
    dir_input.dir = dir.normalize_or_zero();

    macro_rules! impl_button {
        ($input:expr, $keyboard:expr, $button:ident, $key:ident) => {
            $input
                .pressed
                .insert(ButtKind::$button, $keyboard.pressed(binds.$key));
            $input
                .just_pressed
                .insert(ButtKind::$button, $keyboard.just_pressed(binds.$key));
        };
    }
    impl_button!(butt_input, keyboard, A, a);
    impl_button!(butt_input, keyboard, B, b);
    impl_button!(butt_input, keyboard, Enter, enter);
    impl_button!(butt_input, keyboard, Escape, escape);
}

pub(super) fn register_keyboard(app: &mut App) {
    app.insert_resource(Pers::<BindingsKeyboard>::load());

    app.add_systems(
        Update,
        (hacky_prebuilts, update_input_from_keyboard)
            .chain()
            .in_set(InputSet)
            .run_if(in_keyboard_mode),
    );
}
