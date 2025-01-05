use crate::prelude::*;

mod bevy;
mod dreamlake;
pub(self) mod menu_common;
mod overworld;
mod savefile;
mod title;

fn fuck_it_elegy(mut song_manager: ResMut<SongManager>) {
    song_manager.fade_to(Song::Elegy);
}

pub(super) struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(menu_common::cleanup_menu_temp);

        app.add_systems(OnEnter(MetaStateKind::Menu), fuck_it_elegy);

        app.add_systems(Update, menu_common::watch_auto_transitions);

        bevy::register_bevy(app);
        dreamlake::register_dreamlake(app);
        overworld::register_overworld(app);
        savefile::register_savefile(app);
        title::register_title(app);
    }
}
