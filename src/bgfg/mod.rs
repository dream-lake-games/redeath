use crate::prelude::*;

mod bg_clouds;
mod bg_stars;
mod parallax;

pub use bg_clouds::BgClouds;
pub use bg_stars::SpawnStarsEvent;
pub use parallax::{BlackScreenImage, ParallaxScreenImage};

pub(super) struct BgFgPlugin;
impl Plugin for BgFgPlugin {
    fn build(&self, app: &mut App) {
        bg_clouds::register_bg_clouds(app);
        bg_stars::register_bg_stars(app);
        parallax::register_parallax(app);
    }
}
