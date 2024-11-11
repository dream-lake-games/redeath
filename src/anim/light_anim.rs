use crate::prelude::*;

pub trait LightAnimRadius: AnimStateMachine {
    const RADIUS: f32;
}

derive_anim!(
    pub enum PlayerLightAnim {
        #[default]
        #[file("play/circle_light.png")]
        #[size(128, 128)]
        #[render_layers(LightLayer)]
        Static128,
    }
);
type PlayerLightAnimPlugin = AnimDefnPlugin<PlayerLightAnim, AnimTimeRes>;
impl LightAnimRadius for PlayerLightAnim {
    const RADIUS: f32 = 64.0;
}

derive_anim!(
    pub enum LightStatic64Anim {
        #[default]
        #[file("light/static64.png")]
        #[size(64, 64)]
        #[render_layers(LightLayer)]
        Static64,
    }
);
type LightStatic64AnimPlugin = AnimDefnPlugin<LightStatic64Anim, AnimTimeRes>;
impl LightAnimRadius for LightStatic64Anim {
    const RADIUS: f32 = 32.0;
}

derive_anim!(
    pub enum FireflyLightAnim {
        #[default]
        #[file("clear.png")]
        #[size(1, 1)]
        #[render_layers(MenuLayer)]
        None,
        #[file("light/firefly16_grow.png")]
        #[size(16, 16)]
        #[fps(8.0)]
        #[length(2)]
        #[render_layers(LightLayer)]
        #[next(Flap)]
        Grow,
        #[file("light/firefly16_flap.png")]
        #[size(16, 16)]
        #[fps(8.0)]
        #[length(4)]
        #[render_layers(LightLayer)]
        Flap,
        #[file("light/firefly16_shrink.png")]
        #[size(16, 16)]
        #[fps(8.0)]
        #[length(2)]
        #[render_layers(LightLayer)]
        #[next(None)]
        Shrink,
    }
);
type FireflyLightAnimPlugin = AnimDefnPlugin<FireflyLightAnim, AnimTimeRes>;
impl LightAnimRadius for FireflyLightAnim {
    const RADIUS: f32 = 8.0;
}

pub(super) fn register_light_anim(app: &mut App) {
    app.add_plugins(PlayerLightAnimPlugin::default());
    app.add_plugins(LightStatic64AnimPlugin::default());
    app.add_plugins(FireflyLightAnimPlugin::default());
}
