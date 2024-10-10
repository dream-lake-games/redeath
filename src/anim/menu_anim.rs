use crate::prelude::*;

derive_anim!(
    pub enum SavefileButtonAnim {
        #[default]
        #[file("menu/savefile_button_idle.png")]
        #[size(180, 48)]
        #[render_layers(MenuLayer)]
        Idle,
        #[file("menu/savefile_button_activate.png")]
        #[size(180, 48)]
        #[length(2)]
        #[render_layers(MenuLayer)]
        #[next(Active)]
        Activate,
        #[file("menu/savefile_button_active.png")]
        #[size(180, 48)]
        #[render_layers(MenuLayer)]
        Active,
    }
);
type SavefileButtonAnimPlugin = AnimDefnPlugin<SavefileButtonAnim, AnimTimeRes>;

pub(super) fn register_menu_anim(app: &mut App) {
    app.add_plugins(SavefileButtonAnimPlugin::default());
}
