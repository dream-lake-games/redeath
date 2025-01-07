use crate::prelude::*;

use super::egg_block::{reset_egg_blocks, EggBlocksPop};

#[derive(Bundle)]
struct EggBundle {
    name: Name,
    pos: Pos,
    transform: Transform,
    visibility: Visibility,
    trigger_tx: TriggerTx,
    anim: AnimMan<EggAnim>,
    light: Light<ReplenishLightAnim>,
    bob: Bob,
}
impl MyLdtkEntity for EggBundle {
    type Root = ItemsRoot;
    fn from_ldtk(pos: Pos, _fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        Self {
            name: Name::new("egg"),
            pos,
            transform: pos.to_transform(ZIX_ITEMS + 0.1),
            visibility: Visibility::Inherited,
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
    transform: Transform,
    visibility: Visibility,
    anim: AnimMan<EggGhostAnim>,
    chase: ChaseEntity,
    parent: EggGhostParent,
    youngest: YoungestEggGhost,
    slid: SpawnedLid,
    state: EggGhostState,
}
impl EggGhostBundle {
    fn new(pos: Pos, chase: Entity, slid: String) -> Self {
        Self {
            name: Name::new("egg_ghost"),
            pos,
            dyno: default(),
            transform: pos.to_transform(ZIX_ITEMS + 1.1),
            visibility: Visibility::Inherited,
            anim: default(),
            chase: ChaseEntity::new(chase, 300.0, 250.0, 16.0, 256.0),
            parent: EggGhostParent,
            youngest: YoungestEggGhost,
            slid: SpawnedLid::new(slid),
            state: EggGhostState::Collected,
        }
    }
}

#[derive(Component)]
struct YoungestEggGhost;
#[derive(Component)]
struct EggGhostParent;
#[derive(Component, Clone, Copy)]
pub enum EggGhostState {
    // The egg is collected and is chasing either the player or another egg
    // NOTE: This is a stupid enum from back when I had egg going back to where it started for no reason
    //       To lazy to fully deprecate
    Collected,
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
    root: Res<WorldMetaRoot>,
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

    let mut any_unbroken = false;
    let mut any_broken = false;
    for (_, pos, mut anim, mut light, trx_ctrl, slid) in &mut eggs {
        if !matches!(anim.get_state(), EggAnim::Spin) {
            continue;
        }
        if trigger_colls
            .get_refs(&trx_ctrl.coll_keys)
            .iter()
            .any(|coll| coll.rx_kind == TriggerRxKind::Player)
        {
            any_broken = true;
            anim.set_state(EggAnim::Break);
            light.set_state(ReplenishLightAnim::None);
            clear_youngest(&mut commands);
            commands
                .spawn(EggGhostBundle::new(*pos, go_chase, slid.iid.clone()))
                .set_parent(root.eid());
        } else {
            any_unbroken = true;
        }
    }
    if any_broken && any_unbroken {
        commands.spawn(SoundEffect::EggBreakSingle);
    }
}

pub(super) fn egg_ghost_juice(
    mut ghosts: Query<(&Pos, &Dyno, &mut AnimMan<EggGhostAnim>)>,
    mut commands: Commands,
    world_detail_root: Res<WorldDetailRoot>,
) {
    for (pos, dyno, mut anim) in &mut ghosts {
        if anim.get_state() == EggGhostAnim::Popped {
            continue;
        }
        if dyno.vel.length() < 16.0 {
            anim.set_state(EggGhostAnim::Straight);
        } else {
            if dyno.vel.x.abs() > dyno.vel.y.abs() {
                if dyno.vel.x < 0.0 {
                    anim.set_state(EggGhostAnim::Left);
                } else {
                    anim.set_state(EggGhostAnim::Right);
                }
            } else {
                if dyno.vel.y < 0.0 {
                    anim.set_state(EggGhostAnim::Down);
                } else {
                    anim.set_state(EggGhostAnim::Up);
                }
            }
        }
        commands
            .spawn(EphemeralAnim::new(
                EggGhostFadeAnim::Fade,
                false,
                *pos,
                ZIX_ITEMS + 1.0,
            ))
            .set_parent(world_detail_root.eid());
    }
}

fn observe_block_pops(
    trigger: Trigger<EggBlocksPop>,
    mut ghosts: Query<(Entity, &mut AnimMan<EggGhostAnim>, &mut Dyno, &SpawnedLid)>,
    mut bullet_time: ResMut<BulletTime>,
    mut commands: Commands,
    mut camera_shake: ResMut<CameraShake>,
) {
    let iid = &trigger.event().iid;
    for (eid, mut anim, mut dyno, slid) in &mut ghosts {
        if !slid.iid.eq(iid) {
            continue;
        }
        anim.set_state(EggGhostAnim::Popped);
        dyno.vel *= 0.1;
        commands.entity(eid).remove::<ChaseEntity>();
    }
    bullet_time.set_temp(BulletTimeSpeed::Stopped, 0.2);
    commands.spawn(SoundEffect::EggBreakAll);
    camera_shake.shake(0.15, -1..=1, -1..=1);
}

fn reset_eggs_helper(
    iid: &str,
    eggs: &mut Query<(&mut AnimMan<EggAnim>, &SpawnedLid)>,
    ghosts: &mut Query<(Entity, &SpawnedLid), With<AnimMan<EggGhostAnim>>>,
    commands: &mut Commands,
) {
    for (mut anim, slid) in eggs {
        if !slid.iid.eq(iid) {
            continue;
        }
        anim.set_state(EggAnim::Spin);
    }
    for (eid, slid) in ghosts {
        if slid.iid.eq(iid) {
            continue;
        }
        commands.entity(eid).despawn_recursive();
    }
}

fn maybe_reset_eggs(
    trigger: Trigger<LevelChangeEvent>,
    mut eggs: Query<(&mut AnimMan<EggAnim>, &SpawnedLid)>,
    mut ghosts: Query<(Entity, &SpawnedLid), With<AnimMan<EggGhostAnim>>>,
    mut commands: Commands,
    mut blocks: Query<(
        Entity,
        &mut AnimMan<EggBlockAnim>,
        &SpawnedLid,
        Option<&StaticTxCtrl>,
    )>,
) {
    let iid = &trigger.event().iid;
    reset_eggs_helper(iid, &mut eggs, &mut ghosts, &mut commands);
    reset_egg_blocks(&iid, &mut blocks, &mut commands);
}

fn reset_eggs_after_dying(
    level_selection: Option<Res<LevelSelection>>,
    mut eggs: Query<(&mut AnimMan<EggAnim>, &SpawnedLid)>,
    mut ghosts: Query<(Entity, &SpawnedLid), With<AnimMan<EggGhostAnim>>>,
    mut commands: Commands,
    mut blocks: Query<(
        Entity,
        &mut AnimMan<EggBlockAnim>,
        &SpawnedLid,
        Option<&StaticTxCtrl>,
    )>,
) {
    let Some(level_selection) = level_selection else {
        // Idk man this comes up when I try to backspace hot reload while the player is dead
        return;
    };
    let iid = level_selection.to_iid();
    reset_eggs_helper(&iid, &mut eggs, &mut ghosts, &mut commands);
    reset_egg_blocks(&iid, &mut blocks, &mut commands);
}

pub(super) fn register_egg(app: &mut App) {
    app.add_observer(observe_block_pops);
    app.add_observer(maybe_reset_eggs);
    app.add_systems(OnExit(PlayerMetaState::Dying), reset_eggs_after_dying);

    app.add_plugins(MyLdtkEntityPlugin::<EggBundle>::new("Entities", "Egg"));
    app.add_systems(
        Update,
        (break_eggs, egg_ghost_juice)
            .chain()
            .after(PhysicsSet)
            .after(ChaseSet)
            .run_if(in_state(MetaStateKind::World))
            .run_if(in_state(PhysicsState::Active)),
    );
}
