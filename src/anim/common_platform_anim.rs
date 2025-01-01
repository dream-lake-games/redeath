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
    pub enum WidePlankFallAnim {
        #[default]
        #[file("environment/plank_fall/wide_respawn.png")]
        #[size(24, 12)]
        #[length(5)]
        #[next(Stable)]
        #[render_layers(StaticLayer)]
        Respawn,
        #[file("environment/plank_fall/wide_stable.png")]
        #[size(24, 12)]
        #[render_layers(StaticLayer)]
        Stable,
        #[file("environment/plank_fall/wide_shaking.png")]
        #[size(24, 12)]
        #[length(5)]
        #[next(Falling)]
        #[render_layers(StaticLayer)]
        Shaking,
        #[file("environment/plank_fall/wide_falling.png")]
        #[size(24, 12)]
        #[render_layers(StaticLayer)]
        Falling,
        #[file("environment/plank_fall/wide_fade.png")]
        #[size(24, 12)]
        #[length(5)]
        #[next(Despawn)]
        #[render_layers(StaticLayer)]
        Fade,
    }
);
type WidePlankFallAnimPlugin = AnimDefnPlugin<WidePlankFallAnim, AnimTimeRes>;

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

derive_anim!(
    pub enum SwitchBlockAnim {
        #[default]
        #[file("environment/switch_block/active_outer.png")]
        #[size(8, 8)]
        #[render_layers(StaticLayer)]
        On,
        #[file("environment/switch_block/inactive_outer.png")]
        #[size(8, 8)]
        #[render_layers(PaletteLayer)]
        Off,
    }
);
type SwitchBlockAnimPlugin = AnimDefnPlugin<SwitchBlockAnim, AnimTimeRes>;

derive_anim!(
    pub enum SwitchBlockCoreAnim {
        #[default]
        #[file("environment/switch_block/shared_core.png")]
        #[size(8, 8)]
        #[render_layers(PaletteLayer)]
        On,
    }
);
type SwitchBlockCoreAnimPlugin = AnimDefnPlugin<SwitchBlockCoreAnim, AnimTimeRes>;

derive_anim!(
    pub enum SwitchBlockEffectAnim {
        #[default]
        #[file("environment/switch_block/effect_turn_on.png")]
        #[size(12, 12)]
        #[length(4)]
        #[render_layers(PaletteLayer)]
        #[next(Despawn)]
        TurnOn,
        #[file("environment/switch_block/effect_turn_off.png")]
        #[size(12, 12)]
        #[length(3)]
        #[next(Despawn)]
        TurnOff,
    }
);
type SwitchBlockEffectAnimPlugin = AnimDefnPlugin<SwitchBlockEffectAnim, AnimTimeRes>;

pub(super) fn register_common_platform_anim(app: &mut App) {
    app.add_plugins(PlankFallAnimPlugin::default());
    app.add_plugins(WidePlankFallAnimPlugin::default());
    app.add_plugins(FragileIce8AnimPlugin::default());
    app.add_plugins(SwitchBlockAnimPlugin::default());
    app.add_plugins(SwitchBlockCoreAnimPlugin::default());
    app.add_plugins(SwitchBlockEffectAnimPlugin::default());
}
