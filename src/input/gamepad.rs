use crate::prelude::*;

use super::in_gamepad_mode;

fn update_input_from_gamepad(
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    input_mode: Res<InputMode>,
    mut dir_input: ResMut<DirInput>,
    mut butt_input: ResMut<ButtInput>,
) {
    let InputMode::Gamepad(gamepad) = *input_mode else {
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

    let mut left_stick = Vec2::ZERO;
    if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        left_stick = Vec2::new(x, y);
    }

    if left_stick.length() > 0.9 {
        dir_input.dir = left_stick.normalize_or_zero();
    } else {
        dir_input.dir = Vec2::ZERO;
    }

    let a_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::South,
    };
    let b_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::West,
    };
    let enter_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::Start,
    };
    let escape_button = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::Select,
    };

    macro_rules! impl_button {
        ($input:expr, $keyboard:expr, $button:ident, $butt_exp:expr) => {
            $input
                .pressed
                .insert(ButtKind::$button, $keyboard.pressed($butt_exp));
            $input
                .just_pressed
                .insert(ButtKind::$button, $keyboard.just_pressed($butt_exp));
        };
    }
    impl_button!(butt_input, buttons, A, a_button);
    impl_button!(butt_input, buttons, B, b_button);
    impl_button!(butt_input, buttons, Enter, enter_button);
    impl_button!(butt_input, buttons, Escape, escape_button);
}

pub(super) fn register_gamepad(app: &mut App) {
    app.add_systems(
        Update,
        update_input_from_gamepad
            .in_set(InputSet)
            .run_if(in_gamepad_mode),
    );
}
