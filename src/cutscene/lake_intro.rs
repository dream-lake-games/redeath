use crate::prelude::*;

use super::cutlude::*;

fn on_enter(root: Res<ConvoRoot>, mut commands: Commands) {
    commands.observe(start_intro_convo).set_parent(root.eid());

    commands.spawn(DoInSeconds::new(StartIntroConvo, 1.0));
}

fn init_puppet(mut player_anim: Query<&mut AnimMan<PlayerAnim>>) {
    let mut player_anim = player_anim.single_mut();
    player_anim.set_state(PlayerAnim::EdgeSitting);
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
    add_basic_systems!(app, LakeIntro);
    add_puppet_init!(app, LakeIntro);
}
