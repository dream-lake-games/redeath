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

pub(super) fn register_viewsavefiles(app: &mut App) {
    app.add_systems(Update, do_view);
}
