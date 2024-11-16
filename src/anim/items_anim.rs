use crate::prelude::*;

derive_anim!(
    pub enum ReplenishAnim {
        #[default]
        #[file("items/replenish_spawn.png")]
        #[size(16, 16)]
        #[length(3)]
        #[render_layers(StaticLayer)]
        #[next(Pulse)]
        Spawn,
        #[file("items/replenish_pulse.png")]
        #[size(16, 16)]
        #[length(10)]
        #[fps(8.0)]
        #[render_layers(StaticLayer)]
        Pulse,
        #[file("items/replenish_break.png")]
        #[size(16, 16)]
        #[length(7)]
        #[render_layers(StaticLayer)]
        #[next(None)]
        Break,
        #[file("items/replenish_none.png")]
        #[size(16, 16)]
        #[length(10)]
        #[fps(8.0)]
        #[render_layers(StaticLayer)]
        None,
    }
);
type ReplenishAnimPlugin = AnimDefnPlugin<ReplenishAnim, AnimTimeRes>;

derive_anim!(
    pub enum EggAnim {
        #[default]
        #[file("items/egg_spin.png")]
        #[size(24, 24)]
        #[length(10)]
        #[fps(8.0)]
        #[render_layers(StaticLayer)]
        Spin,
        #[file("items/egg_break.png")]
        #[size(24, 24)]
        #[length(7)]
        #[render_layers(StaticLayer)]
        #[next(None)]
        Break,
        #[file("clear.png")]
        #[size(1, 1)]
        None,
    }
);
type EggAnimPlugin = AnimDefnPlugin<EggAnim, AnimTimeRes>;

derive_anim!(
    pub enum EggGhostAnim {
        #[default]
        #[file("items/egg_ghost_spawn.png")]
        #[size(24, 24)]
        #[length(4)]
        #[next(EggGhostStraight)]
        #[render_layers(StaticLayer)]
        Spawn,
        #[file("items/egg_ghost_straight.png")]
        #[size(24, 24)]
        #[render_layers(StaticLayer)]
        EggGhostStraight,
        #[file("items/egg_ghost_left.png")]
        #[size(24, 24)]
        #[render_layers(StaticLayer)]
        EggGhostLeft,
        #[file("items/egg_ghost_right.png")]
        #[size(24, 24)]
        #[render_layers(StaticLayer)]
        EggGhostRight,
        #[file("items/egg_ghost_up.png")]
        #[size(24, 24)]
        #[render_layers(StaticLayer)]
        EggGhostUp,
        #[file("items/egg_ghost_down.png")]
        #[size(24, 24)]
        #[render_layers(StaticLayer)]
        EggGhostDown,
        #[file("items/egg_ghost_grabbed.png")]
        #[size(24, 24)]
        #[length(9)]
        #[fps(8.0)]
        #[render_layers(StaticLayer)]
        #[next(Despawn)]
        EggGhostGrabbed,
    }
);
type EggGhostAnimPlugin = AnimDefnPlugin<EggGhostAnim, AnimTimeRes>;

derive_anim!(
    pub enum EggGhostFadeAnim {
        #[default]
        #[file("items/egg_ghost_fade.png")]
        #[size(24, 24)]
        #[length(3)]
        #[next(Despawn)]
        #[render_layers(PaletteLayer)]
        #[fps(6.0)]
        Fade,
    }
);
type EggGhostFadeAnimPlugin = AnimDefnPlugin<EggGhostFadeAnim, AnimTimeRes>;

pub(super) fn register_items_anim(app: &mut App) {
    app.add_plugins(ReplenishAnimPlugin::default());
    app.add_plugins(EggAnimPlugin::default());
    app.add_plugins(EggGhostAnimPlugin::default());
    app.add_plugins(EggGhostFadeAnimPlugin::default());
}
