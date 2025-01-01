use crate::prelude::*;

pub mod effect;
pub mod effect_defns;
mod level_songs;
pub mod song;
pub mod song_defns;

pub use effect::*;
pub use effect_defns::*;
pub use song::*;
pub use song_defns::*;

#[derive(Debug, Resource)]
pub struct SoundSettings {
    pub main_volume: f32,
    pub effect_volume: f32,
    pub song_volume: f32,
}
impl Default for SoundSettings {
    fn default() -> Self {
        Self {
            main_volume: 0.6,
            effect_volume: 0.6,
            song_volume: 0.6,
        }
    }
}

pub(super) struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SoundSettings::default());
        app.add_plugins(SongPlugin);
        app.add_plugins(SoundEffectPlugin);
        effect_defns::register_effect_defns(app);
        level_songs::register_level_change_song_swaps(app);
        song_defns::register_song_defns(app);
    }
}
