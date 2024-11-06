use super::playerlude::*;
use crate::prelude::*;

macro_rules! go_next {
    ($meta_state:expr, $next_state:expr, $state:ident) => {{
        let MetaState::World(mut world_state) = $meta_state.get().clone() else {
            warn!("can't die bc not in world state");
            return;
        };
        world_state.player_meta_state = PlayerMetaState::$state;
        $next_state.set(world_state.to_meta_state());
    }};
}

fn oob_death(
    oob_player: Query<Entity, (With<Player>, With<PhysicalLidOob>)>,
    meta_state: Res<State<MetaState>>,
    mut next_state: ResMut<NextState<MetaState>>,
) {
    if !oob_player.is_empty() {
        go_next!(meta_state, next_state, Dying);
    }
}

fn spike_death(
    trx_ctl_q: Query<&TriggerRxCtrl, With<Player>>,
    trigger_colls: Res<TriggerColls>,
    meta_state: Res<State<MetaState>>,
    mut next_state: ResMut<NextState<MetaState>>,
) {
    let trx_ctl = trx_ctl_q.single();
    if trigger_colls
        .get_refs(&trx_ctl.coll_keys)
        .iter()
        .any(|coll| matches!(coll.tx_kind, TriggerTxKind::Spikes))
    {
        go_next!(meta_state, next_state, Dying);
    }
}

fn enter_dying(
    mut player: Query<(Entity, &mut Dyno, &mut AnimMan<PlayerAnim>, &Pos), With<Player>>,
    meta_state: Res<State<MetaState>>,
    mut commands: Commands,
) {
    let (eid, mut dyno, mut anim, player_pos) = player.single_mut();
    commands.entity(eid).remove::<Gravity>();
    dyno.vel = Vec2::ZERO;
    anim.set_state(PlayerAnim::Death);
    let MetaState::World(mut world_state) = meta_state.get().clone() else {
        warn!("can't stop dying because not in world meta state");
        return;
    };
    world_state.player_meta_state = PlayerMetaState::Spawning;
    commands.trigger(
        StartTransition::to(world_state.to_meta_state()).with_world_pos(player_pos.as_vec2()),
    );
}

fn exit_dying(mut commands: Commands, player: Query<Entity, With<Player>>) {
    let eid = player.single();
    commands.entity(eid).despawn_recursive();
}

pub(super) fn register_player_death(app: &mut App) {
    app.add_systems(
        Update,
        (oob_death, spike_death)
            .chain()
            .before(AnimSet)
            .in_set(PlayerSet)
            .in_set(PlayerDeathSet)
            .after(PlayerMovementSet)
            .after(InputSet)
            .after(PhysicsSet)
            .run_if(in_state(PlayerMetaState::Playing)),
    );
    app.add_systems(OnEnter(PlayerMetaState::Dying), enter_dying);
    app.add_systems(OnExit(PlayerMetaState::Dying), exit_dying);
}
