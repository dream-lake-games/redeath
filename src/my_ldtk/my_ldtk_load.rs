use crate::prelude::*;

#[derive(Resource, Clone, Debug, Default, Reflect, PartialEq, Eq)]
pub enum MyLdtkLoadState {
    #[default]
    Unloaded,
    Loading,
    Loaded,
}

#[derive(Event)]
pub struct StartMyLdtkLoad {
    pub world_path: String,
    pub level_iid: String,
}
#[derive(Event)]
pub struct UnloadMyLdtk;

fn handle_start_my_ldtk_load(
    trigger: Trigger<StartMyLdtkLoad>,
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut my_ldtk_load_state: ResMut<MyLdtkLoadState>,
) {
    commands.spawn((
        Name::new("MyLdtkRoot"),
        LdtkWorldBundle {
            ldtk_handle: ass.load(&trigger.event().world_path),
            ..default()
        },
        BlockMyLdtkLoad::ticks(10),
    ));
    commands.insert_resource(LevelSelection::iid(&trigger.event().level_iid));
    *my_ldtk_load_state = MyLdtkLoadState::Loading;
}

fn handle_unload_my_ldtk(
    _trigger: Trigger<UnloadMyLdtk>,
    mut commands: Commands,
    existing_root: Query<Entity, With<Handle<LdtkProject>>>,
    mut level_rects: ResMut<LevelRects>,
    mut my_ldtk_load_state: ResMut<MyLdtkLoadState>,
) {
    for eid in &existing_root {
        commands.entity(eid).despawn_recursive();
    }
    *level_rects = default();
    *my_ldtk_load_state = MyLdtkLoadState::Unloaded;
}

fn is_loading(res: Res<MyLdtkLoadState>) -> bool {
    res.into_inner() == &MyLdtkLoadState::Loading
}

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct BlockMyLdtkLoad {
    pub ticks: u32,
}
impl BlockMyLdtkLoad {
    pub fn ticks(ticks: u32) -> Self {
        Self { ticks }
    }
}

fn handle_loading(
    mut commands: Commands,
    mut blockers: Query<(Entity, &mut BlockMyLdtkLoad)>,
    level_rects: Res<LevelRects>,
    mut my_ldtk_load_state: ResMut<MyLdtkLoadState>,
) {
    // Check for explicit blockers
    if !blockers.is_empty() {
        for (eid, mut blocker) in &mut blockers {
            if blocker.ticks == 0 {
                commands.entity(eid).remove::<BlockMyLdtkLoad>();
            } else {
                blocker.ticks -= 1;
            }
        }
        return;
    }
    // Check that level rects is non-empty
    if level_rects.is_empty() {
        return;
    }
    // Probably good
    *my_ldtk_load_state = MyLdtkLoadState::Loaded;
}

pub(super) fn register_my_ldtk_load(app: &mut App) {
    reg_types!(app, BlockMyLdtkLoad);

    app.insert_resource(MyLdtkLoadState::default());

    app.observe(handle_start_my_ldtk_load);
    app.observe(handle_unload_my_ldtk);

    app.add_systems(Update, handle_loading.run_if(is_loading));
}
