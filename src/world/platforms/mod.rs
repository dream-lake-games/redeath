use crate::prelude::*;

mod dirt;

pub(super) struct PlatformsPlugin;
impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        dirt::register_dirt(app);
    }
}
