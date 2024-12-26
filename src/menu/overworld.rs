use crate::prelude::*;

use super::menu_common::*;

// STARTER LEVEL
// const HACK_LOAD: &'static str = "d32f7850-73f0-11ef-ab29-c106faf0247d";

// SCRATCH LEVEL
const HACK_LOAD: &'static str = "0b6da6c0-c210-11ef-b43c-373358fc514e";

#[derive(Bundle)]
struct MenuPlatformBundle<const DUMMY: u32> {}
impl<const DUMMY: u32> MyLdtkIntCell for MenuPlatformBundle<DUMMY> {
    type Root = MenuRoot;
    type RenderLayers = MainLayer;
    type LeftoverRenderLayers = MainLayer;
    fn from_ldtk(_pos: Pos, _value: i32) -> Self {
        Self {}
    }
}

fn on_enter_loading(mut commands: Commands) {
    commands.trigger(UnloadMyLdtk);
    commands.trigger(StartMyLdtkLoad {
        world_path: "menu/overworld.ldtk".to_string(),
        level_iid: "d32f7850-73f0-11ef-ab29-c106faf0247d".to_string(),
    });
}

fn update_loading(
    my_ldtk_load_state: Res<MyLdtkLoadState>,
    level_rects: Res<LevelRects>,
    level_selection: Res<LevelSelection>,
    mut cam_pos_q: Query<&mut Pos, With<DynamicCamera>>,
    mut meta_state: ResMut<NextState<MetaState>>,
    current_savefile: Res<CurrentSavefileKind>,
    all_savefiles: Res<AllSavefiles>,
) {
    if my_ldtk_load_state.into_inner() != &MyLdtkLoadState::Loaded {
        return;
    }
    if let Some(rect) = level_rects.get(level_selection.to_iid().as_str()) {
        let center = rect.center();
        let mut cam_pos = cam_pos_q.single_mut();
        *cam_pos = Pos::new(center.x, center.y);
    }
    let kind = all_savefiles.map[&current_savefile.0].current_world.clone();
    meta_state.set(MenuState::Overworld(OverworldState::from_world_kind(kind)).to_meta_state());
}

fn on_exit_loading(mut commands: Commands) {
    commands.trigger(CleanupMenuTemp);
    commands.trigger(EndTransition::center());
}

fn on_enter_overworld(
    menu_state: Res<State<MenuState>>,
    mut next_meta_state: ResMut<NextState<MetaState>>,
) {
    let MenuState::Overworld(mut overworld) = menu_state.get().clone() else {
        panic!("whoops overworld state is fucked");
    };
    overworld.player_meta_state = PlayerMetaState::Spawning;
    next_meta_state.set(MenuState::Overworld(overworld).to_meta_state());
}

fn update_overworld(butts: Res<ButtInput>, mut commands: Commands) {
    // Watch for going out of overworld
    if butts.pressed(ButtKind::Escape) || butts.pressed(ButtKind::B) {
        commands.trigger(StartTransition::to(MenuState::Savefile.to_meta_state()));
        return;
    }
    // For now just hack in something so we can start
    if butts.pressed(ButtKind::Enter) || butts.pressed(ButtKind::A) {
        commands.trigger(StartTransition::to(
            WorldLoadingState {
                kind: WorldKind::Canyon,
                level_iid: HACK_LOAD.to_string(),
            }
            .to_meta_state(),
        ));
        return;
    }
}

fn on_exit_overworld(mut commands: Commands) {
    commands.trigger(CleanupMenuTemp);
    commands.trigger(CleanupWorld);
}

pub(super) fn register_overworld(app: &mut App) {
    // Ldtk
    app.add_plugins(MyLdtkIntCellPlugin::<MenuPlatformBundle<1>>::single(
        "MenuPlatforms",
        1,
    ));
    app.add_plugins(MyLdtkIntCellPlugin::<MenuPlatformBundle<2>>::single(
        "MenuPlatforms",
        2,
    ));

    // Systems
    app.add_systems(OnEnter(MenuStateKind::OverworldLoading), on_enter_loading);
    app.add_systems(
        Update,
        update_loading.run_if(in_state(MenuStateKind::OverworldLoading)),
    );
    app.add_systems(OnExit(MenuStateKind::OverworldLoading), on_exit_loading);

    app.add_systems(OnEnter(MenuStateKind::Overworld), on_enter_overworld);
    app.add_systems(
        Update,
        update_overworld
            .after(InputSet)
            .run_if(in_state(MenuStateKind::Overworld))
            .run_if(in_state(TransitionActiveState::Inactive)),
    );
    app.add_systems(OnExit(MenuStateKind::Overworld), on_exit_overworld);
}
