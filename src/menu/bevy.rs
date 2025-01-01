use crate::prelude::*;

use super::menu_common::*;

fn on_enter(mut commands: Commands) {
    commands.spawn(MenuImage::new("menu/bevy.png"));
    commands.spawn(AutoTransition(3.0, MenuState::DreamLake.to_meta_state()));
}

fn watch_input(
    butts: Res<ButtInput>,
    mut commands: Commands,
    auto_transitions: Query<&AutoTransition>,
) {
    if auto_transitions.is_empty() {
        // Already going
        return;
    }
    if butts.pressed(ButtKind::Enter) || butts.pressed(ButtKind::A) {
        commands.trigger(StartTransition::to(MenuState::DreamLake.to_meta_state()));
        return;
    }
}

fn on_exit(mut commands: Commands) {
    commands.trigger(CleanupMenuTemp);
}

pub(super) fn register_bevy(app: &mut App) {
    app.add_systems(OnEnter(MenuStateKind::Bevy), on_enter);
    app.add_systems(
        Update,
        watch_input
            .after(InputSet)
            .run_if(in_state(MenuStateKind::Bevy))
            .run_if(in_state(TransitionActiveState::Inactive)),
    );
    app.add_systems(OnExit(MenuStateKind::Bevy), on_exit);
}
