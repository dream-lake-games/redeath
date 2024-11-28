use crate::prelude::*;

pub mod reaper_spooky_appear;

pub use reaper_spooky_appear::*;

pub(super) struct ReaperPlugin;
impl Plugin for ReaperPlugin {
    fn build(&self, app: &mut App) {
        reaper_spooky_appear::register_reaper_spooky_appear(app);
    }
}
