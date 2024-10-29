use crate::prelude::*;

mod dirt;
mod plank_fall;

pub(super) struct PlatformsPlugin;
impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        dirt::register_dirt(app);
        plank_fall::register_plank_fall(app);
    }
}
