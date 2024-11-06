use crate::prelude::*;

mod dirt;
mod fragile_ice;
mod plank_fall;
mod spikes;

pub(super) struct PlatformsPlugin;
impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        dirt::register_dirt(app);
        fragile_ice::register_fragile_ice(app);
        plank_fall::register_plank_fall(app);
        spikes::register_spikes(app);
    }
}
