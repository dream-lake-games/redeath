use crate::prelude::*;

use super::cutlude::*;

fn on_enter(root: Res<ConvoRoot>, mut commands: Commands) {
    commands.observe(start_intro_convo).set_parent(root.eid());

    commands.spawn(DoInSeconds::new(StartIntroConvo, 1.0));
}

decl_cutscene_event!(
    StartIntroConvo,
    fn start_intro_convo(_trigger: Trigger<StartIntroConvo>, mut commands: Commands) {
        commands.trigger(StartConvoLake::Hello);
    }
);

fn update() {}

fn on_exit() {}

pub(super) fn register_lake_intro(app: &mut App) {
    reg_basic_fns!(app, LakeIntro);
}
