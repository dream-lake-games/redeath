use crate::prelude::*;

#[derive(Bundle)]
struct EggBundle {
    name: Name,
    pos: Pos,
    spatial: SpatialBundle,
    trigger_tx: TriggerTx,
    anim: AnimMan<EggAnim>,
    light: Light<ReplenishLightAnim>,
    bob: Bob,
}
impl MyLdtkEntity for EggBundle {
    type Root = WorldRoot;
    fn from_ldtk(pos: Pos, _fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        Self {
            name: Name::new("egg"),
            pos,
            spatial: pos.to_spatial(ZIX_ITEMS),
            trigger_tx: TriggerTx::single(TriggerTxKind::Egg, HBox::new(12, 16)),
            anim: AnimMan::default().with_initial_ix(thread_rng().gen_range(0..10)),
            light: default(),
            bob: Bob::vert(pos, 5.0, 1.0),
        }
    }
}

#[derive(Bundle)]
struct EggGhostBundle {
    name: Name,
    pos: Pos,
    dyno: Dyno,
    spatial: SpatialBundle,
    anim: AnimMan<EggGhostAnim>,
    chase: ChaseEntity,
    parent: EggGhostParent,
    youngest: YoungestEggGhost,
    slid: SpawnedLid,
    state: EggGhostState,
}
impl EggGhostBundle {
    fn new(pos: Pos, parent: Entity, chase: Entity, slid: String) -> Self {
        Self {
            name: Name::new("egg_ghost"),
            pos,
            dyno: default(),
            spatial: pos.to_spatial(ZIX_ITEMS),
            anim: default(),
            chase: ChaseEntity::new(chase, 300.0, 250.0, 16.0, 256.0),
            parent: EggGhostParent { eid: parent },
            youngest: YoungestEggGhost,
            slid: SpawnedLid::new(slid),
            state: EggGhostState::Collected,
        }
    }
}

#[derive(Component, Clone)]
struct ChaseEntity {
    eid: Entity,
    acc: f32,
    // Deceleration as a speed to make it framerate-independent
    dec: f32,
    leash: f32,
    max_speed: f32,
}
impl ChaseEntity {
    fn new(eid: Entity, acc: f32, dec: f32, leash: f32, max_speed: f32) -> Self {
        Self {
            eid,
            acc,
            dec,
            leash,
            max_speed,
        }
    }
}

#[derive(Component, Clone, Copy)]
enum ChaseState {
    InLeash,
    OutLeash,
    BadTarget,
}
#[derive(Component)]
struct YoungestEggGhost;
#[derive(Component)]
struct EggGhostParent {
    eid: Entity,
}
#[derive(Component, Clone, Copy)]
enum EggGhostState {
    // The egg is collected and is chasing either the player or another egg
    Collected,
    // They player died or left the room, go back to spawn
    Returning,
}

fn break_eggs(
    mut eggs: Query<(
        Entity,
        &Pos,
        &mut AnimMan<EggAnim>,
        &mut AnimMan<ReplenishLightAnim>,
        &TriggerTxCtrl,
        &SpawnedLid,
    )>,
    trigger_colls: Res<TriggerColls>,
    existing_youngest: Query<Entity, With<YoungestEggGhost>>,
    player: Query<Entity, With<Player>>,
    mut commands: Commands,
) {
    let would_chase = existing_youngest
        .get_single()
        .ok()
        .or(player.get_single().ok());
    let Some(go_chase) = would_chase else {
        return;
    };

    let clear_youngest = |commands: &mut Commands| {
        for eid in &existing_youngest {
            commands.entity(eid).remove::<YoungestEggGhost>();
        }
    };

    for (eid, pos, mut anim, mut light, trx_ctrl, slid) in &mut eggs {
        if !matches!(anim.get_state(), EggAnim::Spin) {
            continue;
        }
        if trigger_colls
            .get_refs(&trx_ctrl.coll_keys)
            .iter()
            .any(|coll| coll.rx_kind == TriggerRxKind::Player)
        {
            anim.set_state(EggAnim::Break);
            light.set_state(ReplenishLightAnim::None);
            clear_youngest(&mut commands);
            commands.spawn(EggGhostBundle::new(*pos, eid, go_chase, slid.iid.clone()));
        }
    }
}

fn chase_entities(
    mut chasers: Query<(Entity, &Pos, &mut Dyno, &ChaseEntity)>,
    pos_q: Query<&Pos>,
    bullet_time: Res<BulletTime>,
    mut commands: Commands,
) {
    let handle_decel = |vel: &mut Vec2, amt: f32| {
        if vel.length() < amt {
            *vel = Vec2::ZERO;
        } else {
            *vel = *vel - vel.normalize_or_zero() * amt;
        }
    };
    for (eid, chaser_pos, mut chaser_dyno, chase) in &mut chasers {
        let Ok(target_pos) = pos_q.get(chase.eid) else {
            commands.entity(eid).insert(ChaseState::BadTarget);
            handle_decel(
                &mut chaser_dyno.vel,
                chase.acc * bullet_time.delta_seconds(),
            );
            continue;
        };
        // Always decellerate to avoid orbiting
        handle_decel(
            &mut chaser_dyno.vel,
            chase.dec * bullet_time.delta_seconds(),
        );
        if chaser_pos.as_vec2().distance(target_pos.as_vec2()) < chase.leash {
            commands.entity(eid).insert(ChaseState::InLeash);
            handle_decel(
                &mut chaser_dyno.vel,
                chase.acc * bullet_time.delta_seconds(),
            );
        } else {
            commands.entity(eid).insert(ChaseState::OutLeash);
            let norm_diff = (target_pos.as_vec2() - chaser_pos.as_vec2()).normalize_or_zero();
            chaser_dyno.vel += norm_diff * chase.acc * bullet_time.delta_seconds();
        }
        // Always clamp max speeds
        chaser_dyno.vel = chaser_dyno.vel.clamp_length(0.0, chase.max_speed);
    }
}

fn start_returning_all_egg_ghosts(
    mut ghosts: Query<(
        &mut ChaseEntity,
        &mut ChaseState,
        &SpawnedLid,
        &EggGhostParent,
        &mut EggGhostState,
    )>,
    plids: Query<&PhysicalLid>,
    youngest: Query<Entity, With<YoungestEggGhost>>,
    mut commands: Commands,
) {
    let need_to_return = ghosts
        .iter()
        .any(|(chase, chase_state, slid, _parent, egg_state)| {
            if matches!(egg_state, EggGhostState::Returning) {
                // Ignore things that are already returning
                return false;
            }
            if matches!(chase_state, ChaseState::BadTarget) {
                // The thing we're chasing is gone (player died)
                return true;
            }
            if let Ok(plid) = plids.get(chase.eid) {
                if let Some(pid) = plid.last_known_iid.as_ref() {
                    if !pid.eq(&slid.iid) {
                        return true;
                    }
                } else {
                    return true;
                }
            } else {
                // Probably means following another ghost
                return false;
            }
            false
        });
    if need_to_return {
        for eid in &youngest {
            commands.entity(eid).remove::<YoungestEggGhost>();
        }
        for (mut chase, mut chase_state, _, parent, mut egg_state) in &mut ghosts {
            if matches!(*egg_state, EggGhostState::Returning) {
                continue;
            }
            chase.eid = parent.eid;
            chase.acc *= 2.0;
            chase.dec *= 2.0;
            chase.leash = 5.0;
            *chase_state = ChaseState::OutLeash;
            *egg_state = EggGhostState::Returning;
        }
    }
}

fn finish_returning_egg_ghosts(
    ghosts: Query<(Entity, &EggGhostState, &ChaseState, &EggGhostParent)>,
    mut commands: Commands,
    mut egg_anims: Query<&mut AnimMan<EggAnim>>,
) {
    for (eid, egg_state, chase_state, parent) in &ghosts {
        if matches!(
            (egg_state, chase_state),
            (
                EggGhostState::Returning,
                ChaseState::InLeash | ChaseState::BadTarget
            )
        ) {
            commands.entity(eid).despawn_recursive();
            if let Ok(mut anim) = egg_anims.get_mut(parent.eid) {
                anim.set_state(EggAnim::Spin);
            }
        }
    }
}

fn egg_ghost_juice(
    mut ghosts: Query<(&Pos, &Dyno, &mut AnimMan<EggGhostAnim>)>,
    mut commands: Commands,
) {
    for (pos, dyno, mut anim) in &mut ghosts {
        if dyno.vel.length() < 16.0 {
            anim.set_state(EggGhostAnim::EggGhostStraight);
        } else {
            if dyno.vel.x.abs() > dyno.vel.y.abs() {
                if dyno.vel.x < 0.0 {
                    anim.set_state(EggGhostAnim::EggGhostLeft);
                } else {
                    anim.set_state(EggGhostAnim::EggGhostRight);
                }
            } else {
                if dyno.vel.y < 0.0 {
                    anim.set_state(EggGhostAnim::EggGhostDown);
                } else {
                    anim.set_state(EggGhostAnim::EggGhostUp);
                }
            }
        }
        commands.spawn(EphemeralAnim::new(
            EggGhostFadeAnim::Fade,
            false,
            *pos,
            ZIX_ITEMS + 1.0,
        ));
    }
}

pub(super) fn register_egg(app: &mut App) {
    app.add_plugins(MyLdtkEntityPlugin::<EggBundle>::new("Entities", "Egg"));
    app.add_systems(
        Update,
        (
            break_eggs,
            chase_entities,
            start_returning_all_egg_ghosts,
            finish_returning_egg_ghosts,
            egg_ghost_juice,
        )
            .chain()
            .after(PhysicsSet)
            .run_if(in_state(MetaStateKind::World)),
    );
}
