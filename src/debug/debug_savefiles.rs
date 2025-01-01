use crate::prelude::*;

fn do_view(keyboard: Res<ButtonInput<KeyCode>>, store: Res<PkvStore>) {
    if !keyboard.just_pressed(KeyCode::KeyP) {
        return;
    }
    for kind in SavefileKind::all() {
        let dest = format!("debug/savefiles/{}.json", kind.to_string());
        let val = store.get::<Savefile>(kind).unwrap();
        let json = serde_json::to_string_pretty(&val).unwrap();
        std::fs::write(dest, json).unwrap();
    }
}

fn delete_all_savefiles(
    keyboard: Res<ButtonInput<KeyCode>>,
    store: ResMut<PkvStore>,
    all_saves: Option<ResMut<AllSavefiles>>,
    mut commands: Commands,
) {
    if keyboard.pressed(KeyCode::KeyB)
        && keyboard.pressed(KeyCode::KeyA)
        && keyboard.pressed(KeyCode::KeyD)
    {
        let store = store.into_inner();
        store.clear().ok();
        if let Some(mut all_saves) = all_saves {
            *all_saves = AllSavefiles::load(store);
        }
        commands.trigger(SavefileGetRecalculate);
    }
}

pub(super) fn register_viewsavefiles(app: &mut App) {
    app.add_systems(Update, do_view);
    app.add_systems(Update, delete_all_savefiles);
}
