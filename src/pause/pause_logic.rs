use crate::prelude::*;

use super::{
    pause_button::{PauseButtonBundle, PauseButtonInput, PauseText},
    pause_image::PauseImage,
};

fn maybe_start_pause(
    butt_input: Res<ButtInput>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
) {
    if butt_input.just_pressed(ButtKind::Escape) {
        next_pause_state.set(PauseState::Paused);
    }
}

fn maybe_stop_pause(
    butt_input: Res<ButtInput>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
) {
    if butt_input.just_pressed(ButtKind::Escape) {
        next_pause_state.set(PauseState::Unpaused);
    }
}

fn on_enter(mut commands: Commands, camera_pos: Query<&Pos, With<DynamicCamera>>) {
    let camera_pos = camera_pos.single();
    commands.spawn(
        PauseImage::new("menu/pause/pause_bg.png")
            .with_render_layers(PaletteLayer::to_render_layers())
            .with_pos(camera_pos.clone()),
    );
    // commands.trigger(EndTransition::center());
    commands.insert_resource(PauseButtonInput::default());
    commands.spawn(PauseButtonBundle::new("Continue", 32.0).selected());
    commands.spawn(PauseButtonBundle::new("Overworld", -16.0));
    commands.spawn(PauseButtonBundle::new("Main Menu", -64.0));
}

fn watch_input(
    dir: Res<DirInput>,
    time: Res<Time>,
    mut input: ResMut<PauseButtonInput>,
    mut buttons: Query<(
        &Transform,
        &PauseText,
        &mut AnimMan<PauseButtonAnim>,
        Entity,
    )>,
) {
    // Maybe turn off block move
    if let Some((signum, time_left)) = input.block_move.as_mut() {
        if (dir.length_squared() < 0.5) || (dir.y.signum() != *signum) || (*time_left <= 0.0) {
            input.block_move = None;
        } else {
            *time_left -= time.delta_secs();
        }
    }

    // Maybe switch button
    let Some(current_tran) = buttons
        .iter()
        .filter(|query| {
            matches!(
                query.2.get_state(),
                PauseButtonAnim::Active | PauseButtonAnim::Activate
            )
        })
        .next()
        .map(|query| query.0.translation)
    else {
        warn!("nothing selected in pause");
        return;
    };

    // Super ugly but basically just set one button to "activate" anim
    let mut did_activate = None;
    if input.block_move.is_none() && dir.y.abs() != 0.0 {
        if dir.y < 0.0 {
            let mut maybe_next = buttons
                .iter_mut()
                .filter(|query| query.0.translation.y < current_tran.y)
                .collect::<Vec<_>>();
            maybe_next.sort_by(|a, b| a.0.translation.y.total_cmp(&b.0.translation.y));
            if let Some(query) = maybe_next.last_mut() {
                query.2.set_state(PauseButtonAnim::Activate);
                did_activate = Some(query.3);
            }
        } else {
            let mut maybe_next = buttons
                .iter_mut()
                .filter(|query| query.0.translation.y > current_tran.y)
                .collect::<Vec<_>>();
            maybe_next.sort_by(|a, b| a.0.translation.y.total_cmp(&b.0.translation.y));
            if let Some(query) = maybe_next.first_mut() {
                query.2.set_state(PauseButtonAnim::Activate);
                did_activate = Some(query.3);
            }
        }
    }
    // Update the buttons if we activated something
    if let Some(activated_eid) = did_activate {
        for (_, _, mut anim, eid) in &mut buttons {
            if eid != activated_eid {
                anim.set_state(PauseButtonAnim::Idle);
            }
        }
        input.block_move = Some((dir.y.signum(), 0.28));
    }
}

fn maybe_handle_input(
    butt_input: Res<ButtInput>,
    buttons: Query<(&PauseText, &AnimMan<PauseButtonAnim>)>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
    mut commands: Commands,
) {
    if !butt_input.just_pressed(ButtKind::Enter) && !butt_input.just_pressed(ButtKind::A) {
        return;
    }
    let Some(text) = buttons
        .iter()
        .filter(|query| {
            matches!(
                query.1.get_state(),
                PauseButtonAnim::Activate | PauseButtonAnim::Active
            )
        })
        .map(|query| query.0 .0.clone())
        .next()
    else {
        return;
    };

    match text.as_str() {
        "Continue" => {
            next_pause_state.set(PauseState::Unpaused);
        }
        "Overworld" => {
            commands.trigger(StartTransition::to(
                MenuState::OverworldLoading.to_meta_state(),
            ));
        }
        "Main Menu" => {
            commands.trigger(StartTransition::to(MenuState::Title.to_meta_state()));
        }
        other => {
            warn!("unknown pause button: {other:?}");
        }
    }
}

fn on_exit(mut commands: Commands, pause_root: Res<PauseRoot>) {
    commands.entity(pause_root.eid()).despawn_descendants();
    commands.remove_resource::<PauseButtonInput>();
}

pub(super) fn register_pause(app: &mut App) {
    app.add_systems(
        PreUpdate,
        maybe_start_pause
            .run_if(in_state(MetaStateKind::World))
            .run_if(in_state(PlayerMetaState::Playing))
            .run_if(in_state(PauseState::Unpaused)),
    );
    app.add_systems(
        PreUpdate,
        maybe_stop_pause
            .run_if(in_state(MetaStateKind::World))
            .run_if(in_state(PlayerMetaState::Playing))
            .run_if(in_state(PauseState::Paused)),
    );

    app.add_systems(OnEnter(PauseState::Paused), on_enter);
    app.add_systems(
        Update,
        (watch_input, maybe_handle_input)
            .after(InputSet)
            .run_if(in_state(PauseState::Paused))
            .run_if(in_state(TransitionActiveState::Inactive)),
    );
    app.add_systems(OnExit(PauseState::Paused), on_exit);
}
