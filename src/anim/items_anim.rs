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

pub(super) fn register_items_anim(app: &mut App) {
    app.add_plugins(ReplenishAnimPlugin::default());
}
