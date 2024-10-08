use crate::prelude::*;

use super::common::*;

fn on_enter(mut commands: Commands) {
    commands.spawn(MenuImage::new("menu/savefile.png"));
    commands.trigger(EndTransition);
}

fn watch_input(butts: Res<ButtInput>, mut commands: Commands) {
    if butts.pressed(ButtKind::Escape) || butts.pressed(ButtKind::B) {
        commands.trigger(StartTransition::to(MenuState::Title.to_meta_state()));
    }
}

fn on_exit(mut commands: Commands) {
    commands.trigger(CleanupMenuTemp);
}

pub(super) fn register_savefile(app: &mut App) {
    app.add_systems(OnEnter(MenuStateKind::Savefile), on_enter);
    app.add_systems(
        Update,
        watch_input
            .after(InputSet)
            .run_if(in_state(MenuStateKind::Savefile))
            .run_if(in_state(TransitionActiveState::Inactive)),
    );
    app.add_systems(OnExit(MenuStateKind::Savefile), on_exit);
}
