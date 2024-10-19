use crate::prelude::*;
use bevy::audio::{PlaybackMode, Volume};

#[derive(Component)]
struct SongMarker;

#[derive(Component)]
struct SongMarkerChild;

#[derive(Clone, Debug, Reflect)]
struct SongTransition {
    to: Song,
    fade_out: Option<Timer>,
    fade_in: Option<Timer>,
}

#[derive(Resource, Debug, Default, Clone, Reflect)]
pub struct SongManager {
    current: Song,
    transition: Option<SongTransition>,
}
impl SongManager {
    const FADE_TIME: f32 = 0.2;

    pub fn fade_to(&mut self, song: Song) {
        if song == self.current && self.transition.is_none() {
            // Don't need to do anything
            return;
        }
        self.transition = Some(SongTransition {
            to: song,
            fade_out: Some(Timer::from_seconds(Self::FADE_TIME, TimerMode::Once)),
            fade_in: None,
        });
    }
}

fn setup_songs(mut commands: Commands, asset_server: Res<AssetServer>, sound_root: Res<SoundRoot>) {
    commands
        .spawn((SongMarker, Name::new("song")))
        .with_children(|parent| {
            parent.spawn((
                AudioBundle {
                    source: asset_server.load("music/draft.ogg"),
                    settings: PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Loop,
                        volume: Volume::new(0.0),
                        ..default()
                    },
                },
                SongMarkerChild,
                Name::new("song_child"),
            ));
        })
        .set_parent(sound_root.eid());
}

fn update_song(
    song_parent: Query<Entity, With<SongMarker>>,
    mut song_child: Query<(&AudioSink, &mut PlaybackSettings), With<SongMarkerChild>>,
    sound_settings: Res<SoundSettings>,
    mut manager: ResMut<SongManager>,
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    let parent_eid = song_parent.single();
    let child = song_child.get_single_mut();
    let kind_factor = manager.current.mult();
    let settings_factor = sound_settings.main_volume * sound_settings.song_volume;
    let set_volume = |x: f32| {
        if let Ok((audio_sink, mut playback_settings)) = child {
            audio_sink.set_volume(kind_factor * settings_factor * x);
            playback_settings.volume = Volume::new(kind_factor * settings_factor * x);
        }
    };
    let mut go_to = Song::default();
    let mut respawn = false;
    let mut stop_transition = false;
    match manager.transition.as_mut() {
        Some(transition) => {
            let mut go_next = false;
            if let Some(fade_out) = transition.fade_out.as_mut() {
                fade_out.tick(time.delta());
                let x = Spleen::EaseInOutCubic.bound_interp(fade_out.fraction(), 1.0, 0.0);
                set_volume(x);
                go_next = fade_out.finished();
            } else if let Some(fade_in) = transition.fade_in.as_mut() {
                fade_in.tick(time.delta());
                let x = Spleen::EaseInOutCubic.bound_interp(fade_in.fraction(), 0.0, 1.0);
                set_volume(x);
                go_next = fade_in.finished();
            }
            if go_next {
                if transition.fade_out.is_some() {
                    transition.fade_out = None;
                    transition.fade_in =
                        Some(Timer::from_seconds(SongManager::FADE_TIME, TimerMode::Once));
                    go_to = transition.to;
                    respawn = true;
                } else if transition.fade_in.is_some() {
                    transition.fade_out = None;
                    transition.fade_in = None;
                    stop_transition = true;
                }
            }
        }
        None => {
            set_volume(1.0);
        }
    }
    if respawn {
        commands.entity(parent_eid).despawn_descendants();
        commands.entity(parent_eid).with_children(|parent| {
            parent.spawn((
                AudioBundle {
                    source: asset_server.load(go_to.path()),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Loop,
                        volume: Volume::new(0.0),
                        paused: false,
                        ..default()
                    },
                },
                SongMarkerChild,
                Name::new("song_child"),
            ));
        });
        manager.current = go_to;
    }
    if stop_transition {
        manager.transition = None;
    }
}

pub(super) struct SongPlugin;

impl Plugin for SongPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SongManager::default());
        app.add_systems(Startup, setup_songs.after(RootInit));
        app.add_systems(Update, update_song);
    }
}
