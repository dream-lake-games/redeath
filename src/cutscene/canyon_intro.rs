use crate::prelude::*;

use super::cutlude::*;

#[derive(Bundle)]
struct FriendBundle {
    name: Name,
    pos: Pos,
    dyno: Dyno,
    spatial: SpatialBundle,
    anim: AnimMan<FriendAnim>,
    light: Light<LightStatic64Anim>,
}
impl FriendBundle {
    fn new(pos: Pos) -> Self {
        Self {
            name: Name::new("friend"),
            pos,
            dyno: Dyno::new(36.0, 0.0),
            spatial: pos.to_spatial(ZIX_PLAYER - 0.5),
            anim: AnimMan::new(FriendAnim::Run),
            light: default(),
        }
    }
}
fn update_friend(
    mut friend_q: Query<(&Pos, &mut AnimMan<FriendAnim>, &mut Dyno)>,
    player_q: Query<&Pos, With<Player>>,
    mut commands: Commands,
) {
    let (Ok((friend_pos, mut friend_anim, mut friend_dyno)), Ok(player_pos)) =
        (friend_q.get_single_mut(), player_q.get_single())
    else {
        return;
    };
    if player_pos.x - 38.0 < friend_pos.x && friend_dyno.vel.x > 10.0 {
        friend_anim.set_state(FriendAnim::Stand);
        friend_dyno.vel.x = 0.0;
        commands.spawn(DoInSeconds::new(StartIntroConvo, 0.2));
    }
}

fn on_enter(root: Res<CutsceneRoot>, mut commands: Commands) {
    commands.observe(start_intro_convo).set_parent(root.eid());
    commands.observe(end_cutscene).set_parent(root.eid());
}

fn init_puppet(mut player_q: Query<(&Pos, &mut AnimMan<PlayerAnim>)>, mut commands: Commands) {
    let (pos, mut player_anim) = player_q.single_mut();
    player_anim.set_state(PlayerAnim::EdgeSitting);
    commands.spawn(FriendBundle::new(pos.translated(Vec2::new(-128.0, 10.0))));
}

decl_cutscene_event!(
    StartIntroConvo,
    fn start_intro_convo(_trigger: Trigger<StartIntroConvo>, mut commands: Commands) {
        commands.trigger(StartConvoCanyon::Intro);
    }
);

fn end_cutscene(
    trigger: Trigger<ConvoCanyonEnded>,
    mut cutscene_state: ResMut<NextState<CutsceneState>>,
) {
    match trigger.event() {
        ConvoCanyonEnded::Intro => {
            cutscene_state.set(CutsceneState::None);
        }
    }
}

fn update() {}

fn on_exit(meta_state: Res<State<MetaState>>, mut next_meta_state: ResMut<NextState<MetaState>>) {
    let MetaState::World(mut world_state) = meta_state.get().clone() else {
        panic!("canyon_intro bad exit");
    };
    world_state.player_meta_state = PlayerMetaState::Playing;
    next_meta_state.set(world_state.to_meta_state());
}

pub(super) fn register_canyon_intro(app: &mut App) {
    add_common_systems!(app, CanyonIntro);
    add_puppet_init!(app, CanyonIntro);

    add_update_systems!(app, CanyonIntro, update_friend);
}
