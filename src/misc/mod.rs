use crate::prelude::*;

mod speedrun_timer;

pub use speedrun_timer::format_time as format_speedrun_time;

pub(super) struct MiscPlugin;
impl Plugin for MiscPlugin {
    fn build(&self, app: &mut App) {
        speedrun_timer::register_speedrun_timer(app);
    }
}
