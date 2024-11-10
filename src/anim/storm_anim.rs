use crate::prelude::*;

derive_anim!(
    pub enum RainTallAnim {
        #[default]
        #[file("environment/rain/rain_tall.png")]
        #[size(8, 32)]
        #[length(32)]
        #[render_layers(BgLayer)]
        #[fps(128.0)]
        Normal,
    }
);
type RainTallAnimPlugin = AnimDefnPlugin<RainTallAnim, AnimTimeRes>;

derive_anim!(
    #[time_class(ANIM_TIME_REAL)]
    pub enum LightningAnim {
        #[default]
        #[file("environment/bg/lightning1.png")]
        #[size(240, 184)]
        #[render_layers(BgLayer)]
        #[fps(6.0)]
        #[next(Despawn)]
        One,
        #[file("environment/bg/lightning2.png")]
        #[size(240, 184)]
        #[render_layers(BgLayer)]
        #[fps(6.0)]
        #[next(Despawn)]
        Two,
    }
);
type LightningAnimPlugin = AnimDefnPlugin<LightningAnim, AnimTimeRes>;

impl_rand_variant!(LightningAnim);

pub(super) fn register_storm_anim(app: &mut App) {
    app.add_plugins(RainTallAnimPlugin::default());
    app.add_plugins(LightningAnimPlugin::default());
}
