use crate::prelude::*;

mod bevy;
mod dreamlake;
pub(self) mod menu_common;
mod overworld;
mod savefile;
mod title;

pub(super) struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.observe(menu_common::cleanup_menu_temp);

        bevy::register_bevy(app);
        dreamlake::register_dreamlake(app);
        overworld::register_overworld(app);
        savefile::register_savefile(app);
        title::register_title(app);
    }
}
