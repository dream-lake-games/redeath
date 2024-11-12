use crate::prelude::*;

macro_rules! defn_songs {
    ([$($name:ident, $path:literal, $mult:literal,)*]) => {
        #[derive(Component, Clone, Copy, Debug, Default, Reflect, std::hash::Hash, PartialEq, Eq)]
        pub enum Song {
            #[default]
            $($name,)*
        }
        impl Song {
            pub fn path(&self) -> String {
                match self {
                    $(Self::$name => $path.to_string(),)*
                }
            }
            pub fn mult(&self) -> f32 {
                match self {
                    $(Self::$name => $mult,)*
                }
            }
        }

        #[derive(Resource, Reflect)]
        pub struct SongMults {
            pub map: HashMap<Song, f32>,
        }
        impl Default for SongMults {
            fn default() -> Self {
                let mut map = HashMap::new();
                $(
                    map.insert(Song::$name, $mult);
                )*
                Self { map }
            }
        }
    };
}

defn_songs!([
    NoSong,
    "music/draft.ogg",
    0.0,
    Elegy,
    "music/elegy.ogg",
    0.14,
    // https://opengameart.org/content/10-free-chiptune-tracks-a-bag-of-chips
    SinisterAbode,
    "music/sinister_abode.ogg",
    0.2,
]);

pub(super) fn register_song_defns(app: &mut App) {
    app.insert_resource(SongMults::default());
}
