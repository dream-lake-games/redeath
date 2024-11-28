use crate::prelude::*;

use super::cutlude::*;

fn on_enter(
    root: Res<CutsceneRoot>,
    mut commands: Commands,
    mut song_manager: ResMut<SongManager>,
) {
    commands
        .observe(start_confront_convo)
        .set_parent(root.eid());
    commands.observe(end_cutscene).set_parent(root.eid());
    commands.spawn(DoInSeconds::new(StartConfrontConvo, 0.5));
    song_manager.fade_to(Song::NoSong);
}

fn init_puppet(mut player_q: Query<&mut Dyno, With<Player>>) {
    let mut dyno = player_q.single_mut();
    dyno.vel.x = 0.0;
}

decl_cutscene_event!(
    StartConfrontConvo,
    fn start_confront_convo(_trigger: Trigger<StartConfrontConvo>, mut commands: Commands) {
        commands.trigger(StartConvoCanyon::ConfrontReaper);
    }
);

fn end_cutscene(
    trigger: Trigger<ConvoCanyonEnded>,
    mut cutscene_state: ResMut<NextState<CutsceneState>>,
) {
    if matches!(trigger.event(), ConvoCanyonEnded::ConfrontReaper) {
        cutscene_state.set(CutsceneState::None);
    }
}

fn update() {}

fn on_exit(
    meta_state: Res<State<MetaState>>,
    mut next_meta_state: ResMut<NextState<MetaState>>,
    mut song_manager: ResMut<SongManager>,
) {
    let MetaState::World(mut world_state) = meta_state.get().clone() else {
        panic!("canyon_intro bad exit");
    };
    world_state.player_meta_state = PlayerMetaState::Playing;
    next_meta_state.set(world_state.to_meta_state());
    song_manager.fade_to(Song::FightAmidstTheDestructionLoop);
}

pub(super) fn register_canyon_confront_reaper(app: &mut App) {
    add_common_systems!(app, CanyonConfrontReaper);
    add_puppet_init!(app, CanyonConfrontReaper);
}
