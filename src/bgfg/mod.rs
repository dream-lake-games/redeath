use crate::prelude::*;

mod bg_clouds;
mod bg_prebuilt;
mod bg_stars;
mod parallax;
mod storm;

pub use bg_clouds::BgClouds;
pub use bg_prebuilt::*;
pub use bg_stars::SpawnStarsEvent;
pub use parallax::{BlackScreenImage, ParallaxScreenImage};
pub use storm::{Lightning, StormManager};

pub(super) struct BgFgPlugin;
impl Plugin for BgFgPlugin {
    fn build(&self, app: &mut App) {
        bg_clouds::register_bg_clouds(app);
        bg_prebuilt::register_bg_prebuilt(app);
        bg_stars::register_bg_stars(app);
        parallax::register_parallax(app);
        storm::register_rain(app);
    }
}
