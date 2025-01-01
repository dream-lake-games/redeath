use crate::prelude::*;

fn autosave_on_level_change(
    trigger: Trigger<LevelChangeEvent>,
    current_savefile_kind: Res<CurrentSavefileKind>,
    mut all_savefiles: ResMut<AllSavefiles>,
    mut commands: Commands,
    meta_state_kind: Res<State<MetaStateKind>>,
) {
    if meta_state_kind.get() != &MetaStateKind::World {
        return;
    }
    let lid = trigger.event().iid.clone();
    let savefile = all_savefiles.map.get_mut(&current_savefile_kind.0).unwrap();
    let world = savefile
        .world_savefiles
        .get_mut(&savefile.current_world)
        .unwrap();
    world.current_run = Some(lid);
    commands.trigger(SavefileSaveCurrentEvent);
}

pub(super) fn register_autosave(app: &mut App) {
    app.observe(autosave_on_level_change);
}
