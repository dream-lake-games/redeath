use crate::prelude::*;

use super::common::*;

fn on_enter(mut commands: Commands) {
    commands.spawn(MenuImage::new("menu/dreamlake.png"));
    commands.trigger(EndTransition);
}

fn watch_input(butts: Res<ButtInput>, mut commands: Commands) {
    if butts.pressed(ButtKind::Enter) || butts.pressed(ButtKind::A) {
        commands.trigger(StartTransition::to(MenuState::Title.to_meta_state()));
    }
}

fn on_exit(mut commands: Commands) {
    commands.trigger(CleanupMenuTemp);
}

pub(super) fn register_dreamlake(app: &mut App) {
    app.add_systems(OnEnter(MenuStateKind::DreamLake), on_enter);
    app.add_systems(
        Update,
        watch_input
            .after(InputSet)
            .run_if(in_state(MenuStateKind::DreamLake))
            .run_if(in_state(TransitionActiveState::Inactive)),
    );
    app.add_systems(OnExit(MenuStateKind::DreamLake), on_exit);
}
