use crate::prelude::*;

fn maybe_reload(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MetaState>>,
    level_selection: Option<Res<LevelSelection>>,
    mut commands: Commands,
) {
    let Some(LevelSelection::Iid(iid)) = level_selection.map(|thing| thing.into_inner()) else {
        return;
    };
    if keyboard.just_pressed(KeyCode::Backspace) {
        commands.trigger(CleanupWorld);
        next_state.set(
            WorldLoadingState {
                kind: WorldKind::Canyon,
                level_iid: iid.to_string(),
            }
            .to_meta_state(),
        );
    }
}

pub(super) fn register_reload(app: &mut App) {
    app.add_systems(
        Last,
        maybe_reload
            .after(InputSet)
            .run_if(in_state(MetaStateKind::World)),
    );
}
