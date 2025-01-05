//! This file is for UPDATING savefiles

use crate::prelude::*;

#[derive(Event)]
pub struct SavefileCollectCoinEvent {
    pub iid: String,
}
fn handle_savefile_collect_coin(
    trigger: Trigger<SavefileCollectCoinEvent>,
    current_savefile_kind: Res<CurrentSavefileKind>,
    mut all_savefiles: ResMut<AllSavefiles>,
    mut commands: Commands,
) {
    let savefile = all_savefiles.map.get_mut(&current_savefile_kind.0).unwrap();
    let current_world = savefile
        .world_savefiles
        .get_mut(&savefile.current_world)
        .unwrap();
    current_world
        .collected_coins
        .insert(trigger.event().iid.clone());
    // NOTE: Although we update the current GET, we don't actually write to a file
    //       We want to only be writing to files during transitions (which happens with autosave)
    commands.trigger(SavefileGetRecalculate);
}

#[derive(Event)]
pub struct SavefileSaveCurrentEvent;
fn handle_savefile_save_current(
    _trigger: Trigger<SavefileSaveCurrentEvent>,
    current_savefile_kind: Res<CurrentSavefileKind>,
    all_savefiles: Res<AllSavefiles>,
    mut store: ResMut<PkvStore>,
    mut commands: Commands,
) {
    if let Err(e) = store.set(
        current_savefile_kind.0,
        all_savefiles.map.get(&current_savefile_kind.0).unwrap(),
    ) {
        warn!("Uh oh, couldn't save: {e:?}");
    }
    commands.trigger(SavefileGetRecalculate);
}

#[derive(Event)]
pub struct SavefileResetEvent {
    pub kind: SavefileKind,
}
fn handle_savefile_reset_event(
    trigger: Trigger<SavefileResetEvent>,
    mut all_savefiles: ResMut<AllSavefiles>,
    mut store: ResMut<PkvStore>,
) {
    all_savefiles
        .map
        .insert(trigger.event().kind, Savefile::default());
    if let Err(e) = store.set(
        trigger.event().kind,
        all_savefiles.map.get(&trigger.event().kind).unwrap(),
    ) {
        warn!("Uh oh, couldn't save: {e:?}");
    }
}

/// Completes the current world of the active savefile
#[derive(Event)]
pub struct SavefileCompleteWorldEvent;
fn handle_savefile_complete_world_event(
    _trigger: Trigger<SavefileCompleteWorldEvent>,
    current_savefile_kind: Res<CurrentSavefileKind>,
    mut all_savefiles: ResMut<AllSavefiles>,
    mut commands: Commands,
) {
    let savefile = all_savefiles.map.get_mut(&current_savefile_kind.0).unwrap();
    let world_data = savefile
        .world_savefiles
        .get_mut(&savefile.current_world)
        .unwrap();
    world_data.ever_finished = true;
    world_data.current_run = None;
    commands.trigger(SavefileSaveCurrentEvent);
}

/// Register a death
#[derive(Event)]
pub struct SavefileRecordDeathEvent;
fn handle_savefile_record_death_event(
    _trigger: Trigger<SavefileRecordDeathEvent>,
    current_savefile_kind: Res<CurrentSavefileKind>,
    mut all_savefiles: ResMut<AllSavefiles>,
    mut commands: Commands,
) {
    let savefile = all_savefiles.map.get_mut(&current_savefile_kind.0).unwrap();
    savefile.num_deaths += 1;
    commands.trigger(SavefileGetRecalculate);
}

pub(super) fn register_savefile_set(app: &mut App) {
    app.add_observer(handle_savefile_collect_coin);
    app.add_observer(handle_savefile_save_current);
    app.add_observer(handle_savefile_reset_event);
    app.add_observer(handle_savefile_complete_world_event);
    app.add_observer(handle_savefile_record_death_event);
}
