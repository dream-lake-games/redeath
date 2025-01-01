use crate::prelude::*;

mod pause_button;
mod pause_image;
mod pause_logic;

pub(super) struct PausePlugin;
impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        pause_logic::register_pause(app);
    }
}
