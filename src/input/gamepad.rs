use crate::prelude::*;

use super::in_gamepad_mode;

fn maintain_gamepad_root(
    ents: Query<Entity, (With<Gamepad>, Without<Parent>)>,
    root: Res<GamepadRoot>,
    mut commands: Commands,
) {
    for eid in &ents {
        if let Some(mut comms) = commands.get_entity(eid) {
            comms.set_parent(root.eid());
        }
    }
}

fn update_input_from_gamepad(
    input_mode: Res<InputMode>,
    mut dir_input: ResMut<DirInput>,
    mut butt_input: ResMut<ButtInput>,
    gamepads: Query<&Gamepad>,
) {
    let InputMode::Gamepad(eid) = *input_mode else {
        return;
    };
    let Ok(gamepad) = gamepads.get(eid) else {
        // Will be handled next frame
        return;
    };
    let mut dir = Vec2::ZERO;
    let left_stick = gamepad.left_stick();
    if left_stick.x.abs() > 0.8 {
        dir.x = left_stick.x.signum();
    }
    if left_stick.y.abs() > 0.8 {
        dir.y = left_stick.y.signum();
    }
    dir_input.dir = dir.normalize_or_zero();

    macro_rules! impl_button {
        ($input:expr, $gamepad:expr, $button:ident, $butt_exp:expr) => {
            $input
                .pressed
                .insert(ButtKind::$button, $gamepad.pressed($butt_exp));
            $input
                .just_pressed
                .insert(ButtKind::$button, $gamepad.just_pressed($butt_exp));
        };
    }
    impl_button!(butt_input, gamepad, A, GamepadButton::South);
    impl_button!(butt_input, gamepad, B, GamepadButton::West);
    impl_button!(butt_input, gamepad, Enter, GamepadButton::Start);
    impl_button!(butt_input, gamepad, Escape, GamepadButton::Select);
}

pub(super) fn register_gamepad(app: &mut App) {
    app.add_systems(
        Update,
        (maintain_gamepad_root, update_input_from_gamepad)
            .in_set(InputSet)
            .run_if(in_gamepad_mode),
    );
}
