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
        #[next(Straight)]
        #[render_layers(StaticLayer)]
        Spawn,
        #[file("items/egg_ghost_straight.png")]
        #[size(24, 24)]
        #[render_layers(StaticLayer)]
        Straight,
        #[file("items/egg_ghost_left.png")]
        #[size(24, 24)]
        #[render_layers(StaticLayer)]
        Left,
        #[file("items/egg_ghost_right.png")]
        #[size(24, 24)]
        #[render_layers(StaticLayer)]
        Right,
        #[file("items/egg_ghost_up.png")]
        #[size(24, 24)]
        #[render_layers(StaticLayer)]
        Up,
        #[file("items/egg_ghost_down.png")]
        #[size(24, 24)]
        #[render_layers(StaticLayer)]
        Down,
        #[file("items/egg_ghost_popped.png")]
        #[size(24, 24)]
        #[length(9)]
        #[fps(8.0)]
        #[render_layers(StaticLayer)]
        #[next(Despawn)]
        Popped,
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

derive_anim!(
    pub enum EggBlockAnim {
        #[default]
        #[file("environment/egg_block/size_16x16_solid.png")]
        #[size(16, 16)]
        #[render_layers(StaticLayer)]
        Solid,
        #[file("environment/egg_block/size_16x16_pop.png")]
        #[size(16, 16)]
        #[length(6)]
        #[render_layers(StaticLayer)]
        #[next(None)]
        Pop,
        #[file("clear.png")]
        #[size(1, 1)]
        None,
    }
);
type EggBlockAnimPlugin = AnimDefnPlugin<EggBlockAnim, AnimTimeRes>;

derive_anim!(
    pub enum CoinAnim {
        #[default]
        #[file("items/coin_spin.png")]
        #[size(24, 24)]
        #[length(9)]
        #[fps(8.0)]
        #[render_layers(StaticLayer)]
        Spin,
        #[file("items/coin_spin_empty.png")]
        #[size(24, 24)]
        #[length(9)]
        #[fps(8.0)]
        #[render_layers(StaticLayer)]
        SpinEmpty,
        #[file("items/coin_pop.png")]
        #[size(24, 24)]
        #[length(5)]
        #[render_layers(StaticLayer)]
        #[next(None)]
        Pop,
        #[file("items/coin_pop_empty.png")]
        #[size(24, 24)]
        #[length(5)]
        #[render_layers(StaticLayer)]
        #[next(None)]
        PopEmpty,
        #[file("clear.png")]
        #[size(1, 1)]
        None,
    }
);
type CoinAnimPlugin = AnimDefnPlugin<CoinAnim, AnimTimeRes>;

derive_anim!(
    pub enum CoinSmolAnim {
        #[default]
        #[file("items/coin_smol.png")]
        #[size(24, 24)]
        #[render_layers(StaticLayer)]
        Follow,
        #[file("items/coin_smol_empty.png")]
        #[size(24, 24)]
        #[render_layers(StaticLayer)]
        FollowEmpty,
        #[file("items/coin_smol_pop.png")]
        #[size(24, 24)]
        #[length(6)]
        #[next(Despawn)]
        #[render_layers(StaticLayer)]
        Pop,
    }
);
type CoinSmolAnimPlugin = AnimDefnPlugin<CoinSmolAnim, AnimTimeRes>;

derive_anim!(
    pub enum BankAnim {
        #[default]
        #[file("items/coin_bank_idle.png")]
        #[size(24, 24)]
        #[render_layers(StaticLayer)]
        Idle,
        #[file("items/coin_bank_grow.png")]
        #[size(24, 24)]
        #[length(12)]
        #[next(Idle)]
        #[render_layers(StaticLayer)]
        Grow,
        #[file("items/coin_bank_shrink.png")]
        #[size(24, 24)]
        #[length(9)]
        #[next(None)]
        #[render_layers(StaticLayer)]
        Shrink,
        #[file("clear.png")]
        #[size(1, 1)]
        None,
        #[file("items/coin_bank_celebrate.png")]
        #[size(24, 24)]
        #[length(6)]
        #[next(Despawn)]
        #[render_layers(StaticLayer)]
        Celebrate,
    }
);
type BankAnimPlugin = AnimDefnPlugin<BankAnim, AnimTimeRes>;

pub(super) fn register_items_anim(app: &mut App) {
    app.add_plugins(ReplenishAnimPlugin::default());
    app.add_plugins(EggAnimPlugin::default());
    app.add_plugins(EggGhostAnimPlugin::default());
    app.add_plugins(EggGhostFadeAnimPlugin::default());
    app.add_plugins(EggBlockAnimPlugin::default());
    app.add_plugins(CoinAnimPlugin::default());
    app.add_plugins(CoinSmolAnimPlugin::default());
    app.add_plugins(BankAnimPlugin::default());
}
