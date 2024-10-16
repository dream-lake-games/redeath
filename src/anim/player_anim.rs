use crate::prelude::*;

derive_anim!(
    pub enum PlayerCircleLightAnim {
        #[default]
        #[file("play/circle_light.png")]
        #[size(128, 128)]
        #[render_layers(PaletteLayer)]
        Circle,
    }
);
type PlayerCircleLightAnimPLugin = AnimDefnPlugin<PlayerCircleLightAnim, AnimTimeRes>;

pub(super) fn register_player_anim(app: &mut App) {
    app.add_plugins(PlayerCircleLightAnimPLugin::default());
}
