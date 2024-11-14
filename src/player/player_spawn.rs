use bevy::ecs::query::QuerySingleError;

use crate::prelude::*;

use super::player_bundle::PlayerBundle;

#[derive(Component)]
struct SpawnPoint;

#[derive(Component)]
struct SpawnPointActive;

#[derive(Bundle)]
struct SpawnPointBundle {
    name: Name,
    spawn_point: SpawnPoint,
    pos: Pos,
}
impl MyLdtkEntity for SpawnPointBundle {
    type Root = WorldRoot;
    fn from_ldtk(pos: Pos, _fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        Self {
            name: Name::new("SpawnPoint"),
            spawn_point: SpawnPoint,
            pos,
        }
    }
}

fn calculate_new_spawn_point(
    potential: &Query<(Entity, &Pos), (With<SpawnPoint>, With<SpawnedLidActive>)>,
) -> Option<Entity> {
    // This logic finds the spawn point that is closest to the bottom left
    let mut result = None;
    let mut min_pos = None;
    for (eid, pos) in potential {
        match (result.as_mut(), min_pos.as_mut()) {
            (None, _) => {
                result = Some(eid);
                min_pos = Some(pos);
            }
            (Some(old_eid), Some(old_pos)) => {
                if pos.x < old_pos.x || (pos.x == old_pos.y && pos.y < old_pos.y) {
                    *old_eid = eid;
                    *old_pos = pos;
                }
            }
            _ => unreachable!(),
        }
    }
    result
}

fn set_spawn_point_on_level_change(
    _: Trigger<LevelChangeEvent>,
    mut commands: Commands,
    existing: Query<Entity, With<SpawnPointActive>>,
    potential: Query<(Entity, &Pos), (With<SpawnPoint>, With<SpawnedLidActive>)>,
    meta_state_kind: Res<State<MetaStateKind>>,
) {
    if !matches!(
        meta_state_kind.get(),
        MetaStateKind::WorldLoading | MetaStateKind::World
    ) {
        return;
    }
    for eid in &existing {
        commands.entity(eid).remove::<SpawnPointActive>();
    }
    if let Some(eid) = calculate_new_spawn_point(&potential) {
        commands.entity(eid).insert(SpawnPointActive);
    } else {
        warn!("Unable to find new spawn point to make active after changing level");
    }
}

fn spawn_player(
    mut commands: Commands,
    active_spawn_pos: Query<(&Pos, &SpawnedLid), (With<SpawnPointActive>, Without<DynamicCamera>)>,
    world_state: Res<State<WorldState>>,
    cutscene_state: Res<State<CutsceneState>>,
    mut next_meta_state: ResMut<NextState<MetaState>>,
    root: Res<WorldRoot>,
    mut camera_pos: Query<&mut Pos, (With<DynamicCamera>, Without<SpawnPointActive>)>,
    mut camera_mode: ResMut<DynamicCameraMode>,
    level_rects: Res<LevelRects>,
) {
    let (spawn_pos, spawn_spawned_lid) = match active_spawn_pos.get_single() {
        Ok(pos) => pos,
        Err(QuerySingleError::NoEntities(_)) => {
            warn!("no spawn points");
            return;
        }
        Err(QuerySingleError::MultipleEntities(_)) => {
            warn!("multiple spawn points");
            return;
        }
    };
    let player_eid = commands
        .spawn(PlayerBundle::new(
            spawn_pos.clone(),
            spawn_spawned_lid.iid.clone(),
        ))
        .set_parent(root.eid())
        .id();
    let mut world_state = world_state.get().clone();
    world_state.player_meta_state = match cutscene_state.get() {
        CutsceneState::None => PlayerMetaState::Playing,
        _ => PlayerMetaState::Puppet,
    };
    next_meta_state.set(world_state.to_meta_state());
    let mut cam_pos = camera_pos.single_mut();
    *cam_pos = spawn_pos.clone();
    *camera_mode = DynamicCameraMode::Follow(player_eid);
    camera_clamp_logic(&mut cam_pos, &level_rects);

    commands.spawn(ConvoOneoff::medium(
        player_eid,
        Vec2::new(4.0, 0.0),
        "hello!",
    ));
}

fn exit_spawning(player: Query<&Pos, With<Player>>, mut commands: Commands) {
    let player_pos = player.single();
    commands.trigger(EndTransition::default().with_world_pos(player_pos.as_vec2()));
}

pub(super) fn register_player_spawn(app: &mut App) {
    app.add_plugins(MyLdtkEntityPlugin::<SpawnPointBundle>::new(
        "Entities",
        "SpawnPoint",
    ));
    app.observe(set_spawn_point_on_level_change);

    app.add_systems(
        PreUpdate,
        spawn_player.run_if(in_state(PlayerMetaState::Spawning)),
    );
    app.add_systems(OnExit(PlayerMetaState::Spawning), exit_spawning);
}
