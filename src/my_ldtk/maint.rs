use crate::prelude::*;

fn handle_spawned_lids(
    mut commands: Commands,
    ents: Query<(Entity, &SpawnedLid)>,
    selection: Option<Res<LevelSelection>>,
) {
    let maybe_iid = selection.map(|selection| match &selection.into_inner() {
        LevelSelection::Iid(iid) => iid.to_string(),
        _ => panic!("use iid to select levels, please"),
    });
    for (eid, spawned_lid) in &ents {
        match &maybe_iid {
            Some(iid) if iid.eq(&spawned_lid.iid) => {
                commands.entity(eid).insert(SpawnedLidActive);
                commands.entity(eid).remove::<SpawnedLidInactive>();
            }
            Some(_) => {
                commands.entity(eid).remove::<SpawnedLidActive>();
                commands.entity(eid).insert(SpawnedLidInactive);
            }
            None => {
                commands.entity(eid).remove::<SpawnedLidActive>();
                commands.entity(eid).remove::<SpawnedLidInactive>();
            }
        }
    }
}

fn handle_physical_lids(
    mut commands: Commands,
    mut ents: Query<(Entity, &mut PhysicalLid, &Pos, &SpawnedLid)>,
    mut level_rects: ResMut<LevelRects>,
    selection: Option<Res<LevelSelection>>,
    levels: Query<(&LevelIid, &GlobalTransform)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    let Some(ldtk_project) = ldtk_project_assets.get(ldtk_projects.single()) else {
        return;
    };
    // Fetch the level rects (probably cache this at some point)
    for (level_iid, level_transform) in levels.iter() {
        if level_rects.get(level_iid.as_str()).is_some() {
            continue;
        }
        let level = ldtk_project
            .get_raw_level_by_iid(level_iid.get())
            .expect("level should exist in only project");
        let level_bounds = Rect {
            min: Vec2::new(
                level_transform.translation().x,
                level_transform.translation().y,
            ),
            max: Vec2::new(
                level_transform.translation().x + level.px_wid as f32,
                level_transform.translation().y + level.px_hei as f32,
            ),
        };
        level_rects.set(level_iid.to_string(), level_bounds);
    }
    let level_rects = level_rects.into_inner();
    // Update PhysicalLid
    for (_, mut plid, pos, slid) in &mut ents {
        if plid.last_known_iid == None {
            // Assume the first tick is fine
            plid.in_bounds = true;
            plid.last_known_iid = Some(slid.iid.clone());
        } else {
            let old_lid = plid.last_known_iid.as_ref().unwrap();
            let old_rect = level_rects[old_lid];
            if old_rect.contains(pos.as_vec2()) {
                plid.in_bounds = true;
            } else {
                plid.in_bounds = false;
                for (other_lid, other_rect) in level_rects.iter() {
                    if other_rect.contains(pos.as_vec2()) {
                        plid.in_bounds = true;
                        plid.last_known_iid = Some(other_lid.clone());
                        break;
                    }
                }
            }
        }
    }
    // Translate PhysicalLid to marker components
    let maybe_selection = selection.map(|selection| match &selection.into_inner() {
        LevelSelection::Iid(iid) => iid.to_string(),
        _ => panic!("use iid to select levels, please"),
    });
    for (eid, plid, _, _) in &ents {
        match (plid.in_bounds, &plid.last_known_iid, &maybe_selection) {
            (_, _, None) => {
                // If there's no selection, we don't know anything
                commands.entity(eid).remove::<PhysicalLidActive>();
                commands.entity(eid).remove::<PhysicalLidInactive>();
                commands.entity(eid).remove::<PhysicalLidOob>();
            }
            (true, Some(lid), Some(selection)) if lid == selection => {
                // We're in bounds in the selected level
                commands.entity(eid).insert(PhysicalLidActive);
                commands.entity(eid).remove::<PhysicalLidInactive>();
                commands.entity(eid).remove::<PhysicalLidOob>();
                commands.entity(eid).remove::<PhysicalLidOob>();
            }
            (true, _, _) => {
                // We're in bounds, not in the selected level
                commands.entity(eid).remove::<PhysicalLidActive>();
                commands.entity(eid).insert(PhysicalLidInactive);
                commands.entity(eid).remove::<PhysicalLidOob>();
            }
            (false, _, _) => {
                // We're out of bounds
                commands.entity(eid).remove::<PhysicalLidActive>();
                commands.entity(eid).remove::<PhysicalLidInactive>();
                commands.entity(eid).insert(PhysicalLidOob);
            }
        }
    }
}

pub(super) fn register_my_ldtk_maint(app: &mut App) {
    app.add_systems(
        Update,
        (handle_spawned_lids, handle_physical_lids).in_set(PhysicsSet),
    );
}
