//! This file is for UPDATING savefiles
//! NOTE: Not

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
) {
    if let Err(e) = store.set(
        current_savefile_kind.0,
        all_savefiles.map.get(&current_savefile_kind.0).unwrap(),
    ) {
        warn!("Uh oh, couldn't save: {e:?}");
    }
}

pub(super) fn register_savefile_set(app: &mut App) {
    app.observe(handle_savefile_collect_coin);
    app.observe(handle_savefile_save_current);
}
