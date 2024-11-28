use crate::prelude::*;

fn level_change_song_swaps(
    trigger: Trigger<LevelChangeEvent>,
    mut song_manager: ResMut<SongManager>,
) {
    match trigger.event().iid.as_str() {
        // Fade to nothing before seeing reaper for first time
        "e181a7c0-9b00-11ef-909d-8bb1d8b68648" => {
            if song_manager.get_current() == Song::SinisterAbode {
                song_manager.fade_to(Song::NoSong);
            }
        }
        _ => (),
    }
}

pub(super) fn register_level_change_song_swaps(app: &mut App) {
    app.observe(level_change_song_swaps);
}
