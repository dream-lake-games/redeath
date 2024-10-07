use crate::prelude::*;

mod bevy;
pub(self) mod common;
mod dreamlake;

pub(super) struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.observe(common::cleanup_menu_temp);

        bevy::register_bevy(app);
        dreamlake::register_dreamlake(app);
    }
}
