use crate::prelude::*;

use super::cutlude::*;

fn on_enter(root: Res<CutsceneRoot>, mut commands: Commands) {
    commands
        .add_observer(start_reaper_conclusion_convo)
        .set_parent(root.eid());
    commands.add_observer(end_cutscene).set_parent(root.eid());
    commands.spawn(DoInSeconds::new(StartReaperConclusionConvo, 0.5));
}

fn init_puppet(mut player_q: Query<&mut Dyno, With<Player>>) {
    let mut dyno = player_q.single_mut();
    dyno.vel.x = 0.0;
}

decl_cutscene_event!(
    StartReaperConclusionConvo,
    fn start_reaper_conclusion_convo(
        _trigger: Trigger<StartReaperConclusionConvo>,
        mut commands: Commands,
    ) {
        commands.trigger(StartConvoCanyon::ReaperConclusion);
    }
);

fn end_cutscene(
    trigger: Trigger<ConvoCanyonEnded>,
    mut cutscene_state: ResMut<NextState<CutsceneState>>,
) {
    if !matches!(trigger.event(), ConvoCanyonEnded::ReaperConclusion) {
        return;
    }
    cutscene_state.set(CutsceneState::None);
}

fn update() {}

fn on_exit(mut commands: Commands) {
    commands.trigger(SavefileCompleteWorldEvent);
    commands.trigger(StartTransition::to(MetaState::Menu(
        MenuState::OverworldLoading,
    )));
}

pub(super) fn register_canyon_reaper_conclusion(app: &mut App) {
    add_common_systems!(app, CanyonReaperConclusion);
    add_puppet_init!(app, CanyonReaperConclusion);
}
