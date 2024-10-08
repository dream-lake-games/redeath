use crate::prelude::*;

derive_anim!(
    pub enum MenuAnim {
        #[default]
        #[file("menu/savefile_button_idle.png")]
        #[size(180, 48)]
        #[render_layers(MenuLayer)]
        Clear,
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
type MenuAnimPlugin = AnimDefnPlugin<MenuAnim, AnimTimeRes>;

pub(super) fn register(app: &mut App) {
    app.add_plugins(MenuAnimPlugin::default());
}
