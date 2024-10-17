use crate::prelude::*;

derive_anim!(
    pub enum LightAnim {
        #[default]
        #[file("play/circle_light.png")]
        #[size(128, 128)]
        #[render_layers(LightLayer)]
        Static128,
    }
);
type LightAnimPlugin = AnimDefnPlugin<LightAnim, AnimTimeRes>;

impl LightAnim {
    pub fn to_radius(&self) -> f32 {
        match self {
            Self::Static128 => 64.0,
        }
    }
}

pub(super) fn register_light_anim(app: &mut App) {
    app.add_plugins(LightAnimPlugin::default());
}
