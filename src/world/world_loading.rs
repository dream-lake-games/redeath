use crate::prelude::*;

fn on_enter_loading(mut commands: Commands) {
    commands.trigger(UnloadMyLdtk);
    commands.trigger(StartMyLdtkLoad {
        world_path: "worlds/lake.ldtk".to_string(),
        level_iid: "d32f7850-73f0-11ef-ab29-c106faf0247d".to_string(),
    });
}

fn update_loading(
    my_ldtk_load_state: Res<MyLdtkLoadState>,
    level_rects: Res<LevelRects>,
    level_selection: Res<LevelSelection>,
    mut cam_pos_q: Query<&mut Pos, With<DynamicCamera>>,
    mut meta_state: ResMut<NextState<MetaState>>,
    world_loading: Res<State<WorldLoadingState>>,
) {
    if my_ldtk_load_state.into_inner() != &MyLdtkLoadState::Loaded {
        return;
    }
    if let Some(rect) = level_rects.get(level_selection.to_iid().as_str()) {
        let center = rect.center();
        let mut cam_pos = cam_pos_q.single_mut();
        *cam_pos = Pos::new(center.x, center.y);
    }
    meta_state.set(
        WorldState {
            kind: world_loading.get().kind,
            level_state: LevelState {},
        }
        .to_meta_state(),
    );
}

fn on_exit_loading(mut commands: Commands) {
    commands.trigger(EndTransition);
}

pub(super) fn register_world_loading(app: &mut App) {
    app.add_systems(OnEnter(MetaStateKind::WorldLoading), on_enter_loading);
    app.add_systems(
        Update,
        update_loading.run_if(in_state(MetaStateKind::WorldLoading)),
    );
    app.add_systems(OnExit(MetaStateKind::WorldLoading), on_exit_loading);
}
