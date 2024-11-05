use crate::prelude::*;

derive_anim!(
    pub enum PlankFallAnim {
        #[default]
        #[file("environment/plank_fall/respawn.png")]
        #[size(12, 12)]
        #[length(5)]
        #[next(Stable)]
        #[render_layers(StaticLayer)]
        Respawn,
        #[file("environment/plank_fall/stable.png")]
        #[size(12, 12)]
        #[render_layers(StaticLayer)]
        Stable,
        #[file("environment/plank_fall/shaking.png")]
        #[size(12, 12)]
        #[length(5)]
        #[next(Falling)]
        #[render_layers(StaticLayer)]
        Shaking,
        #[file("environment/plank_fall/falling.png")]
        #[size(12, 12)]
        #[render_layers(StaticLayer)]
        Falling,
        #[file("environment/plank_fall/fade.png")]
        #[size(12, 12)]
        #[length(5)]
        #[next(Despawn)]
        #[render_layers(StaticLayer)]
        Fade,
    }
);
type PlankFallAnimPlugin = AnimDefnPlugin<PlankFallAnim, AnimTimeRes>;

derive_anim!(
    pub enum FragileIce8Anim {
        #[default]
        #[file("clear.png")]
        #[size(1, 1)]
        ReadyToRespawn,
        #[file("environment/ice/fragile8_respawn.png")]
        #[size(14, 14)]
        #[length(3)]
        #[next(Stable)]
        #[render_layers(StaticLayer)]
        Respawn,
        #[file("environment/ice/fragile8_stable.png")]
        #[size(14, 14)]
        #[render_layers(StaticLayer)]
        Stable,
        #[file("environment/ice/fragile8_shatter.png")]
        #[size(14, 14)]
        #[length(4)]
        #[next(Despawn)]
        #[render_layers(StaticLayer)]
        Shatter,
    }
);
type FragileIce8AnimPlugin = AnimDefnPlugin<FragileIce8Anim, AnimTimeRes>;

pub(super) fn register_common_platform_anim(app: &mut App) {
    app.add_plugins(PlankFallAnimPlugin::default());
    app.add_plugins(FragileIce8AnimPlugin::default());
}
