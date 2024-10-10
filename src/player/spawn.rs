use crate::prelude::*;

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
        println!("yay! according to plan");
        commands.entity(eid).insert(SpawnPointActive);
    } else {
        warn!("Unable to find new spawn point to make active after changing level");
    }
}

pub(super) fn register_spawn(app: &mut App) {
    app.add_plugins(MyLdtkEntityPlugin::<SpawnPointBundle>::new(
        "Entities",
        "SpawnPoint",
    ));
    app.observe(set_spawn_point_on_level_change);
}
