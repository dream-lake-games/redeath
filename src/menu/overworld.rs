use crate::prelude::*;

use super::menu_common::*;

// STARTER LEVEL
// const HACK_LOAD: &'static str = "d32f7850-73f0-11ef-ab29-c106faf0247d";

// SCRATCH LEVEL
const HACK_LOAD: &'static str = "5a85e440-9b00-11ef-b43c-cda5db3a9172";

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

fn on_enter_loading(mut commands: Commands, current_savefile: Res<CurrentSavefile>) {
    if current_savefile.0.is_new_savefile() {
        // Don't load the overworld on a completely fresh savefile
        commands.trigger(ReplaceTransition::to(
            WorldLoadingState {
                kind: WorldKind::Canyon,
                level_iid: current_savefile.0.get_new_run_lid(),
            }
            .to_meta_state(),
        ));
        return;
    }
    commands.trigger(UnloadMyLdtk);
    commands.trigger(StartMyLdtkLoad {
        world_path: "menu/overworld.ldtk".to_string(),
        level_iid: "d32f7850-73f0-11ef-ab29-c106faf0247d".to_string(),
    });
}

fn update_loading(
    my_ldtk_load_state: Res<MyLdtkLoadState>,
    level_rects: Res<LevelRects>,
    level_selection: Option<Res<LevelSelection>>,
    mut cam_pos_q: Query<&mut Pos, With<DynamicCamera>>,
    mut meta_state: ResMut<NextState<MetaState>>,
    current_savefile_kind: Res<CurrentSavefileKind>,
    all_savefiles: Res<AllSavefiles>,
) {
    if my_ldtk_load_state.into_inner() != &MyLdtkLoadState::Loaded {
        return;
    }
    let Some(level_selection) = level_selection else {
        // Probably means we replaced the transition bc skipping overworld
        return;
    };
    if let Some(rect) = level_rects.get(level_selection.to_iid().as_str()) {
        let center = rect.center();
        let mut cam_pos = cam_pos_q.single_mut();
        *cam_pos = Pos::new(center.x, center.y);
    }
    let kind = all_savefiles.map[&current_savefile_kind.0]
        .current_world
        .clone();
    meta_state.set(MenuState::Overworld(OverworldState::from_world_kind(kind)).to_meta_state());
}

fn on_exit_loading(mut commands: Commands) {
    commands.trigger(CleanupMenuTemp);
    commands.trigger(EndTransition::center());
}

fn on_enter_overworld(
    menu_state: Res<State<MenuState>>,
    mut next_meta_state: ResMut<NextState<MetaState>>,
    mut commands: Commands,
) {
    let MenuState::Overworld(mut overworld) = menu_state.get().clone() else {
        panic!("whoops overworld state is fucked");
    };
    overworld.player_meta_state = PlayerMetaState::Spawning;
    next_meta_state.set(MenuState::Overworld(overworld).to_meta_state());
    // TODO: This is wrong but oh well
    commands.trigger(SetupCanyonBg);
}

fn update_overworld(
    butts: Res<ButtInput>,
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    savefile_current: Res<CurrentSavefile>,
    conts: Query<&AnimMan<OverworldContinueAnim>>,
    restarts: Query<&AnimMan<OverworldRestartAnim>>,
) {
    // Watch for going out of overworld
    if butts.pressed(ButtKind::Escape) {
        commands.trigger(StartTransition::to(MenuState::Savefile.to_meta_state()));
        return;
    }
    if butts.pressed(ButtKind::Enter) {
        if conts
            .iter()
            .any(|query| query.get_state() == OverworldContinueAnim::Open)
        {
            commands.trigger(StartTransition::to(
                WorldLoadingState {
                    kind: WorldKind::Canyon,
                    level_iid: savefile_current.0.get_current_run_lid(),
                }
                .to_meta_state(),
            ));
        } else if restarts
            .iter()
            .any(|query| query.get_state() == OverworldRestartAnim::Active)
        {
            commands.trigger(StartTransition::to(
                WorldLoadingState {
                    kind: WorldKind::Canyon,
                    level_iid: savefile_current.0.get_new_run_lid(),
                }
                .to_meta_state(),
            ));
        }
    }

    #[cfg(debug_assertions)]
    {
        // HACK to get to a specific level
        if keyboard.just_pressed(KeyCode::KeyH) {
            commands.trigger(StartTransition::to(
                WorldLoadingState {
                    kind: WorldKind::Canyon,
                    level_iid: HACK_LOAD.to_string(),
                }
                .to_meta_state(),
            ));
        }
    }
}

fn on_exit_overworld(mut commands: Commands) {
    commands.trigger(CleanupMenuTemp);
    commands.trigger(CleanupWorld);
}

#[derive(Bundle)]
struct OverworldContinueBundle {
    name: Name,
    pos: Pos,
    transform: Transform,
    visibility: Visibility,
    trigger: TriggerTx,
    anim: AnimMan<OverworldContinueAnim>,
}
impl MyLdtkEntity for OverworldContinueBundle {
    type Root = WorldMetaRoot;
    fn from_ldtk(pos: Pos, _fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        Self {
            name: Name::new("continue"),
            pos,
            transform: pos.to_transform(ZIX_OVERWORLDIES),
            visibility: Visibility::Inherited,
            trigger: TriggerTx::single(
                TriggerTxKind::Observe,
                HBox::new(32, 32).translated(0.0, -16.0),
            ),
            anim: default(),
        }
    }
}

#[derive(Bundle)]
struct OverworldRestartBundle {
    name: Name,
    pos: Pos,
    transform: Transform,
    visibility: Visibility,
    trigger: TriggerTx,
    anim: AnimMan<OverworldRestartAnim>,
}
impl MyLdtkEntity for OverworldRestartBundle {
    type Root = WorldMetaRoot;
    fn from_ldtk(pos: Pos, _fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        Self {
            name: Name::new("restart"),
            pos,
            transform: pos.to_transform(ZIX_OVERWORLDIES),
            visibility: Visibility::Inherited,
            trigger: TriggerTx::single(
                TriggerTxKind::Observe,
                HBox::new(32, 32).translated(0.0, -16.0),
            ),
            anim: default(),
        }
    }
}

fn update_continues(
    mut conts: Query<(&TriggerTxCtrl, &mut AnimMan<OverworldContinueAnim>)>,
    trigger_colls: Res<TriggerColls>,
    savefile_current: Res<CurrentSavefile>,
) {
    if savefile_current.0.get_current_run_lid_opt().is_none() {
        for (_, mut anim) in &mut conts {
            anim.set_state(OverworldContinueAnim::Disabled);
        }
    } else {
        for (ttx_ctrl, mut anim) in &mut conts {
            let has_player = trigger_colls
                .get_refs(&ttx_ctrl.coll_keys)
                .iter()
                .any(|coll| coll.rx_kind == TriggerRxKind::Player);
            match (anim.get_state(), has_player) {
                (OverworldContinueAnim::Disabled, _) => {
                    anim.set_state(OverworldContinueAnim::Closed);
                }
                (OverworldContinueAnim::Closed, false) => {}
                (OverworldContinueAnim::Closed, true) => {
                    anim.set_state(OverworldContinueAnim::Opening);
                }
                (OverworldContinueAnim::Opening, false) => {
                    anim.set_state(OverworldContinueAnim::Closing);
                }
                (OverworldContinueAnim::Opening, true) => {}
                (OverworldContinueAnim::Open, false) => {
                    anim.set_state(OverworldContinueAnim::Closing);
                }
                (OverworldContinueAnim::Open, true) => {}
                (OverworldContinueAnim::Closing, false) => {}
                (OverworldContinueAnim::Closing, true) => {
                    anim.set_state(OverworldContinueAnim::Opening);
                }
            }
        }
    }
}

fn update_restarts(
    mut restarts: Query<(&TriggerTxCtrl, &mut AnimMan<OverworldRestartAnim>)>,
    trigger_colls: Res<TriggerColls>,
) {
    for (ttx_ctrl, mut anim) in &mut restarts {
        let has_player = trigger_colls
            .get_refs(&ttx_ctrl.coll_keys)
            .iter()
            .any(|coll| coll.rx_kind == TriggerRxKind::Player);
        if has_player {
            anim.set_state(OverworldRestartAnim::Active);
        } else {
            anim.set_state(OverworldRestartAnim::Idle);
        }
    }
}

fn manage_oneoffs(
    conts: Query<&AnimMan<OverworldContinueAnim>>,
    restarts: Query<&AnimMan<OverworldRestartAnim>>,
    existing: Query<Entity, With<ConvoOneoffText>>,
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
) {
    let can_continue = conts
        .iter()
        .any(|query| query.get_state() == OverworldContinueAnim::Open);
    let can_restart = !can_continue
        && restarts
            .iter()
            .any(|query| query.get_state() == OverworldRestartAnim::Active);
    match (can_continue, can_restart) {
        (false, false) => {
            for eid in &existing {
                if let Some(comms) = commands.get_entity(eid) {
                    comms.despawn_recursive();
                }
            }
        }
        (true, _) => {
            if existing.iter().count() == 0 {
                commands.spawn(ConvoOneoff::medium(
                    player.single(),
                    Vec2::new(4.0, 1.0),
                    "Press enter to continue",
                ));
            }
        }
        (false, true) => {
            if existing.iter().count() == 0 {
                commands.spawn(ConvoOneoff::medium(
                    player.single(),
                    Vec2::new(4.0, 1.0),
                    "Press enter to restart chapter",
                ));
            }
        }
    }
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
    app.add_plugins(MyLdtkEntityPlugin::<OverworldContinueBundle>::new(
        "Entities",
        "OverworldContinue",
    ));
    app.add_plugins(MyLdtkEntityPlugin::<OverworldRestartBundle>::new(
        "Entities",
        "OverworldRestart",
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
        (
            update_overworld,
            update_continues,
            update_restarts,
            manage_oneoffs,
        )
            .after(InputSet)
            .after(PhysicsSet)
            .run_if(in_state(MenuStateKind::Overworld))
            .run_if(in_state(TransitionActiveState::Inactive)),
    );
    app.add_systems(OnExit(MenuStateKind::Overworld), on_exit_overworld);
}
