use crate::prelude::*;

/// The set that contains all ldtk level maintainence
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MyLdtkLevelMaint;

/// Updates the map of known level bounds
fn update_my_level_rects(
    levels: Query<(&LevelIid, &GlobalTransform)>,
    mut level_rects: ResMut<LevelRects>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    selection: Option<Res<LevelSelection>>,
) {
    let Ok(project) = ldtk_projects.get_single() else {
        return;
    };
    let Some(ldtk_project) = ldtk_project_assets.get(project) else {
        return;
    };
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
    level_rects.current = None;
    if let Some(LevelSelection::Iid(iid)) = selection.map(|inner| inner.into_inner()) {
        level_rects.current = level_rects.map.get(&iid.to_string()).cloned();
    }
}

/// Updates the `SpawnLid(In)Active` components
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

/// Updates the `PhysicalLid(In)Active` components
fn handle_physical_lids(
    mut commands: Commands,
    mut ents: Query<(Entity, &mut PhysicalLid, &Pos, &SpawnedLid)>,
    level_rects: Res<LevelRects>,
    selection: Option<Res<LevelSelection>>,
) {
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

/// This event gets fired whenever the LevelSelection changes, in Update
/// NOTE: You CAN assume that this is fired only after all things in that level spawn
/// NOTE: You CAN assume that when this is called, SpawnedLid(In)Active and PhysicalLid(In)Active are correct
#[derive(Event)]
pub struct LevelChangeEvent {
    pub iid: String,
    pub last_iid: Option<String>,
}
#[derive(Resource)]
pub(super) struct LastLevelSelection(String);
fn watch_level_selection(
    mut commands: Commands,
    level_selection: Option<Res<LevelSelection>>,
    mut last_level_selection: Option<ResMut<LastLevelSelection>>,
    my_ldtk_load: Res<MyLdtkLoadState>,
) {
    match (level_selection.as_ref(), last_level_selection.as_mut()) {
        (Some(ls), Some(lls)) => {
            if ls.to_iid() != lls.0 {
                commands.trigger(LevelChangeEvent {
                    iid: ls.to_iid(),
                    last_iid: Some(lls.0.clone()),
                });
                lls.0 = ls.to_iid();
            }
        }
        (Some(ls), None) => match my_ldtk_load.into_inner() {
            MyLdtkLoadState::Loaded => {
                commands.trigger(LevelChangeEvent {
                    iid: ls.to_iid(),
                    last_iid: None,
                });
                commands.insert_resource(LastLevelSelection(ls.to_iid()));
            }
            _ => {
                // Do nothing, wait until load finishes to trigger
            }
        },
        (None, Some(_)) => {
            commands.remove_resource::<LastLevelSelection>();
        }
        (None, None) => (),
    }
}

/// I should've made this earlier, but oh well, I guess it's forward looking.
/// There's a lot of logic that should happen when player enters a level
/// OR respawns in a level
#[derive(Event)]
pub struct EnterOrRespawnLevelEvent {
    pub iid: String,
}
fn do_enter_or_respawn_level_change(trigger: Trigger<LevelChangeEvent>, mut commands: Commands) {
    commands.trigger(EnterOrRespawnLevelEvent {
        iid: trigger.event().iid.clone(),
    });
}
fn do_enter_or_respawn_respawn(mut commands: Commands, level: Option<Res<LevelSelection>>) {
    let Some(LevelSelection::Iid(iid)) = level.map(|inner| inner.into_inner()) else {
        warn!("huh, do_enter_or_respawn_respawn");
        return;
    };
    commands.trigger(EnterOrRespawnLevelEvent {
        iid: iid.as_str().to_string(),
    });
}

pub(super) fn register_my_ldtk_level_maint(app: &mut App) {
    app.insert_resource(LevelRects::default());

    app.add_systems(Update, update_my_level_rects.in_set(MyLdtkLevelMaint));
    app.add_systems(
        Update,
        (handle_spawned_lids, handle_physical_lids)
            .after(update_my_level_rects)
            .in_set(MyLdtkLevelMaint)
            .run_if(in_state(MetaStateKind::World).or_else(in_state(MetaStateKind::Menu))),
    );
    app.add_systems(
        Update,
        watch_level_selection
            .after(handle_spawned_lids)
            .after(handle_physical_lids)
            .in_set(MyLdtkLevelMaint)
            .run_if(in_state(MetaStateKind::World).or_else(in_state(MetaStateKind::Menu))),
    );

    // Enter or respawn
    app.observe(do_enter_or_respawn_level_change);
    app.add_systems(
        OnEnter(PlayerMetaState::Spawning),
        do_enter_or_respawn_respawn,
    );
}
