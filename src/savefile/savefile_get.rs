//! This file is for getting READ-ONLY access to things in savefiles

use crate::prelude::*;

#[derive(Resource, Default)]
pub struct SavefileCurrentCollectedCoins(pub HashSet<String>);

/// This event signals that we need to recalculate all this "cached" savefile data
/// This lets us avoid reading state every frame
/// NOTE: This will just update from Res<AllSavefiles> and Res<CurrentSavefileKind>.
///       You must make sure these actually have what you want to recalculate before calling this
#[derive(Event)]
pub struct SavefileGetRecalculate;
fn handle_savefile_get_recalculate(
    // Plumbing
    _trigger: Trigger<SavefileGetRecalculate>,
    all_savefiles: Res<AllSavefiles>,
    current_savefile_kind: Res<CurrentSavefileKind>,
    // ALL OUR SHIT
    mut current_world_collected_coins: ResMut<SavefileCurrentCollectedCoins>,
) {
    let savefile = all_savefiles.map[&current_savefile_kind.0].clone();
    let world_savefile_data = savefile.world_savefiles[&savefile.current_world].clone();
    current_world_collected_coins.0 = world_savefile_data.collected_coins;
}

pub(super) fn register_savefile_get(app: &mut App) {
    app.observe(handle_savefile_get_recalculate);

    app.insert_resource(SavefileCurrentCollectedCoins::default());
}
