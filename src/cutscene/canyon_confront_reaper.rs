use crate::prelude::*;

use super::cutlude::*;

fn on_enter(
    root: Res<CutsceneRoot>,
    mut commands: Commands,
    meta_state: Res<State<MetaState>>,
    mut next_meta_state: ResMut<NextState<MetaState>>,
) {
    let MetaState::World(mut world_state) = meta_state.get().clone() else {
        panic!("canyon_intro bad exit");
    };
    world_state.player_meta_state = PlayerMetaState::Puppet;
    next_meta_state.set(world_state.to_meta_state());

    commands
        .observe(start_confront_convo)
        .set_parent(root.eid());
    commands.observe(end_cutscene).set_parent(root.eid());
    commands.spawn(DoInSeconds::new(StartConfrontConvo, 0.5));
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
    mut player_anim: Query<&mut AnimMan<PlayerAnim>>,
) {
    if matches!(trigger.event(), ConvoCanyonEnded::ConfrontReaper) {
        cutscene_state.set(CutsceneState::None);
        for mut anim in &mut player_anim {
            anim.set_state(PlayerAnim::EdgeSitup);
        }
    }
}

fn update() {}

fn on_exit(
    meta_state: Res<State<MetaState>>,
    mut next_meta_state: ResMut<NextState<MetaState>>,
    mut song_manager: ResMut<SongManager>,
    mut reapers: Query<&mut AnimMan<ReaperAnim>>,
) {
    let MetaState::World(mut world_state) = meta_state.get().clone() else {
        panic!("canyon_intro bad exit");
    };
    world_state.player_meta_state = PlayerMetaState::Playing;
    next_meta_state.set(world_state.to_meta_state());
    song_manager.fade_to(Song::SinisterAbode);
    for mut anim in &mut reapers {
        anim.set_state(ReaperAnim::Disappear);
    }
}

pub(super) fn register_canyon_confront_reaper(app: &mut App) {
    add_common_systems!(app, CanyonIntro);
}
