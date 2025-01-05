//! This is the logic that handles:
//! - When you load from a save, you get the right song
//! - If there are certain levels that should trigger a song change always
//! See the corresponding entities in ldtk

use crate::prelude::*;

/// Every level should have a songinitial
fn level_song_invariants(
    song_initial_q: Query<&SongInitial, With<SpawnedLidActive>>,
    level_selection: Res<LevelSelection>,
) {
    if song_initial_q.iter().count() != 1 {
        warn!(
            "Hmmm seem to be missing song_initial {:?}",
            level_selection.into_inner()
        );
    }
    // debug_assert!(song_initial_q.iter().count() == 1);
}

#[derive(Component)]
struct HasSetInitialSong;

impl From<Option<FieldValue>> for Song {
    fn from(value: Option<FieldValue>) -> Self {
        let Some(FieldValue::Enum(Some(s))) = value else {
            panic!("Bad song field_value from ldtk: {value:?}");
        };
        match s.as_str() {
            "NoSong" => Song::NoSong,
            "SinisterAbode" => Song::SinisterAbode,
            "FightAmidstTheDestructionIntro" => Song::FightAmidstTheDestructionIntro,
            "FightAmidstTheDestructionLoop" => Song::FightAmidstTheDestructionLoop,
            _ => panic!("Bad song string: {s}"),
        }
    }
}

#[derive(Component)]
struct SongInitial {
    song: Song,
}
#[derive(Bundle)]
struct SongInitialBundle {
    name: Name,
    initial: SongInitial,
}
impl MyLdtkEntity for SongInitialBundle {
    type Root = WorldMetaRoot;
    fn from_ldtk(_pos: Pos, fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        let song = Song::from(fields.get("SongInitial").cloned());
        Self {
            name: Name::new("song_initial"),
            initial: SongInitial { song },
        }
    }
}

#[derive(Component)]
struct SongAlways {
    song: Song,
}
#[derive(Bundle)]
struct SongAlwaysBundle {
    name: Name,
    always: SongAlways,
}
impl MyLdtkEntity for SongAlwaysBundle {
    type Root = WorldMetaRoot;
    fn from_ldtk(_pos: Pos, fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        let song = Song::from(fields.get("SongAlways").cloned());
        Self {
            name: Name::new("song_always"),
            always: SongAlways { song },
        }
    }
}

fn set_initial_song(
    mut commands: Commands,
    set_initial_song: Query<&HasSetInitialSong>,
    song_initial_q: Query<&SongInitial, With<SpawnedLidActive>>,
    meta_root: Res<WorldMetaRoot>,
    mut song_manager: ResMut<SongManager>,
) {
    if !set_initial_song.is_empty() {
        return;
    }
    if let Ok(song_initial) = song_initial_q.get_single() {
        commands
            .spawn((Name::new("HasSetInitialSong"), HasSetInitialSong))
            .set_parent(meta_root.eid());
        song_manager.fade_to(song_initial.song);
    }
}

fn set_always_song(
    _trigger: Trigger<EnterOrRespawnLevelEvent>,
    mut commands: Commands,
    song_always_q: Query<&SongAlways, With<SpawnedLidActive>>,
    mut song_manager: ResMut<SongManager>,
    meta_root: Res<WorldMetaRoot>,
) {
    if let Ok(song_always) = song_always_q.get_single() {
        song_manager.fade_to(song_always.song);
        commands
            .spawn((Name::new("HasSetInitialSong"), HasSetInitialSong))
            .set_parent(meta_root.eid());
    }
}

pub(super) fn register_level_change_song_swaps(app: &mut App) {
    app.add_observer(set_always_song);

    app.add_plugins(MyLdtkEntityPlugin::<SongInitialBundle>::new(
        "Entities",
        "SongInitial",
    ));
    app.add_plugins(MyLdtkEntityPlugin::<SongAlwaysBundle>::new(
        "Entities",
        "SongAlways",
    ));

    app.add_systems(
        Update,
        (level_song_invariants, set_initial_song).run_if(in_state(MetaStateKind::World)),
    );
}
