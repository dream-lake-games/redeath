use crate::prelude::*;

mod cruncher;
mod dirt;
mod fragile_ice;
mod pass_up;
mod plank_fall;
mod spikes;
mod switch_block;

pub(super) struct PlatformsPlugin;
impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        cruncher::register_cruncher(app);
        dirt::register_dirt(app);
        fragile_ice::register_fragile_ice(app);
        pass_up::register_pass_up(app);
        plank_fall::register_plank_fall(app);
        spikes::register_spikes(app);
        switch_block::register_switch_block(app);
    }
}
