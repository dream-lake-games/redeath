use crate::{input::in_keyboard_mode, prelude::*};

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

pub(super) fn register_keyboard(app: &mut App) {
    app.add_systems(
        Update,
        (update_input_from_keyboard,)
            .in_set(InputSet)
            .run_if(in_keyboard_mode),
    );
}
