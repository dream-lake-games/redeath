use crate::prelude::*;

#[derive(Resource, Reflect)]
struct ReaperAttackConsts {
    pad_from_screen_edge: f32,
    body_hover_acc: f32,
    body_charge_acc: f32,
    body_max_speed: f32,
    body_close_destroy: f32,
    scythe_overshoot_turnaround: f32,
    scythe_acc: f32,
    scythe_max_speed_out: f32,
    scythe_max_speed_back: f32,
    simple_attack_time: f32,
}
impl Default for ReaperAttackConsts {
    fn default() -> Self {
        Self {
            pad_from_screen_edge: 24.0,
            body_hover_acc: 240.0,
            body_charge_acc: 180.0,
            body_max_speed: 100.0,
            body_close_destroy: 16.0,
            scythe_overshoot_turnaround: 8.0,
            scythe_acc: 240.0,
            scythe_max_speed_out: 160.0,
            scythe_max_speed_back: 160.0,
            simple_attack_time: 0.75,
        }
    }
}

#[derive(Component, Default)]
struct ReaperAttack {
    next_throw: Option<f32>,
}

#[derive(Bundle)]
struct ReaperAttackBundle {
    name: Name,
    marker: ReaperAttack,
    pos: Pos,
    dyno: Dyno,
    transform: Transform,
    visibility: Visibility,
    anim: AnimMan<ReaperAnim>,
    light: Light<LightStatic64Anim>,
}
impl MyLdtkEntity for ReaperAttackBundle {
    type Root = WorldMetaRoot;
    fn from_ldtk(pos: Pos, _fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        Self {
            name: Name::new("reaper_attack"),
            marker: ReaperAttack::default(),
            pos,
            dyno: default(),
            transform: pos.to_transform(ZIX_PLAYER - 0.5),
            visibility: Visibility::Inherited,
            anim: AnimMan::new(ReaperAnim::None).with_observe_state_changes(),
            light: default(),
        }
    }
}

// For appear:
// - If ONSCREEN, and player is FAR ENOUGH AWAY, go from None -> Appear
// - Otherwise, set None
// NOTE: When appearing, fix the dyno/pos.y
// For dissappear:
// - If ONSCREEN, and NOT NONE/dissapearing, and player is CLOSE ENOUGH, force disappear
fn manage_appear_disappear(
    mut reapers: Query<
        (
            &mut Pos,
            &mut Dyno,
            &mut AnimMan<ReaperAnim>,
            Option<&SpawnedLidActive>,
        ),
        (With<ReaperAttack>, Without<Player>),
    >,
    player_q: Query<&Pos, With<Player>>,
    consts: Res<ReaperAttackConsts>,
    mut wavy_palette_manager: ResMut<WavyPaletteManager>,
) {
    let Ok(player_pos) = player_q.get_single() else {
        return;
    };
    for (mut pos, mut dyno, mut anim, slid_active) in &mut reapers {
        match slid_active {
            Some(_) => {
                match anim.get_state() {
                    ReaperAnim::None => {
                        // Maybe spawn
                        if player_pos.x + 2.0 * consts.body_close_destroy < pos.x {
                            // Yes! Spawn!
                            pos.y = player_pos.y;
                            dyno.vel.y = 0.0;
                            anim.set_state(ReaperAnim::AppearHover);
                            wavy_palette_manager.set(
                                WavyPalette::new(BERRY_NEBULA)
                                    .with_r(-0.06, 0.02, 2.0)
                                    .with_g(-0.18, 0.02, 3.0)
                                    .with_b(-0.18, 0.02, 5.0),
                            );
                        }
                    }
                    ReaperAnim::IdleDisappear | ReaperAnim::HoverDisappear => {
                        // Do nothing
                    }
                    _ => {
                        // Maybe disappear
                        if player_pos.x + consts.body_close_destroy > pos.x {
                            // Yes! Disappear!
                            anim.set_state(ReaperAnim::HoverDisappear);
                            dyno.vel.y *= 0.3;
                        }
                    }
                }
            }
            None => {
                if anim.get_state() != ReaperAnim::None {
                    anim.set_state(ReaperAnim::HoverDisappear);
                }
            }
        }
    }
}

fn manage_onscreen_movement(
    mut reapers: Query<
        (&mut Dyno, &mut Pos, &mut Transform, &AnimMan<ReaperAnim>),
        (
            With<ReaperAttack>,
            With<SpawnedLidActive>,
            Without<Player>,
            Without<DynamicCamera>,
        ),
    >,
    player_q: Query<&Pos, With<Player>>,
    level_scroll: Res<State<LevelScrollStateKind>>,
    camera_q: Query<&Pos, With<DynamicCamera>>,
    consts: Res<ReaperAttackConsts>,
    bullet_time: Res<BulletTime>,
    level_rects: Res<LevelRects>,
    level_selection: Res<LevelSelection>,
) {
    // Figure out what the x should be for the active guy
    let Ok(player_pos) = player_q.get_single() else {
        return;
    };
    let Ok(camera_pos) = camera_q.get_single() else {
        return;
    };
    let LevelSelection::Iid(liid) = level_selection.into_inner() else {
        return;
    };
    let proper_anchor = match level_scroll.get() {
        LevelScrollStateKind::Some => {
            camera_clamp_logic(player_pos, level_rects.get(liid.as_str()).unwrap())
        }
        LevelScrollStateKind::None => camera_pos.clone(),
    };
    let proper_x = proper_anchor.x + SCREEN_WIDTH_f32 / 2.0 - consts.pad_from_screen_edge;

    for (mut dyno, mut pos, mut tran, anim) in &mut reapers {
        // Have to set both pos and transform so it doesn't have weird jitter
        pos.x = proper_x;
        tran.translation.x = proper_x;

        let acc = match anim.get_state() {
            ReaperAnim::Hover => consts.body_hover_acc,
            ReaperAnim::Charge | ReaperAnim::Throw => consts.body_charge_acc,
            _ => continue,
        };

        if (player_pos.y - pos.y).abs() < 3.0 {
            dyno.vel.y -= dyno.vel.y.signum() * acc * bullet_time.delta_secs();
        } else {
            let dir = (player_pos.y - pos.y).signum();
            dyno.vel.y += dir * acc * bullet_time.delta_secs();
        }
        dyno.vel.y = dyno
            .vel
            .y
            .clamp(-consts.body_max_speed, consts.body_max_speed);
    }
}

// Resets the y and appears
fn on_respawn(
    mut reapers: Query<&mut AnimMan<ReaperAnim>, With<ReaperAttack>>,
    mut commands: Commands,
    mut scythes: Query<Entity, With<Scythe>>,
) {
    for mut anim in &mut reapers {
        anim.set_state(ReaperAnim::None);
    }
    for eid in &mut scythes {
        if let Some(comms) = commands.get_entity(eid) {
            comms.despawn_recursive();
        }
    }
}

fn simple_throw(
    mut reapers: Query<
        (Entity, &mut AnimMan<ReaperAnim>, &mut ReaperAttack),
        With<SpawnedLidActive>,
    >,
    consts: Res<ReaperAttackConsts>,
    bullet_time: Res<BulletTime>,
    mut camera_shake: ResMut<CameraShake>,
    mut commands: Commands,
    mut global_shift: ResMut<GlobalPaletteShift>,
) {
    for (eid, mut anim, mut attack) in &mut reapers {
        match (anim.get_state(), attack.next_throw.as_mut()) {
            (ReaperAnim::Hover, None) => {
                attack.next_throw = Some(consts.simple_attack_time);
            }
            (ReaperAnim::Hover, Some(next)) => {
                *next = *next - bullet_time.delta_secs();
                if *next < 0.0 {
                    anim.set_state(ReaperAnim::Charge);
                    attack.next_throw = None;
                    let time_to_do_shit = 0.7;
                    global_shift.add(time_to_do_shit, -1);
                    global_shift.add(0.05, -1);
                    camera_shake.shake(time_to_do_shit, -1..=1, -1..=1);
                    commands
                        .entity(eid)
                        .insert(AnimMan::new(ReaperChargeIndicatorAnim::Charge));
                    commands.spawn(SoundEffect::ReaperChargeStart);
                }
            }
            (_, _) => {
                attack.next_throw = None;
            }
        }
    }
}

fn reign_in_charge_indicator(
    mut reapers: Query<(
        Option<&AnimMan<ReaperAnim>>,
        &mut AnimMan<ReaperChargeIndicatorAnim>,
    )>,
) {
    for (reap_anim, mut ind_anim) in &mut reapers {
        if reap_anim.map(|a| a.get_state()) != Some(ReaperAnim::Charge) {
            ind_anim.set_state(ReaperChargeIndicatorAnim::BeGone);
        }
    }
}

fn watch_reaper_anim_to_spawn_scythe(
    trigger: Trigger<AnimStateChange<ReaperAnim>>,
    mut commands: Commands,
    pos_q: Query<&Pos>,
    slid_q: Query<&SpawnedLid>,
    consts: Res<ReaperAttackConsts>,
    root: Res<WorldMetaRoot>,
    mut global_shift: ResMut<GlobalPaletteShift>,
) {
    let AnimStateChange { next, .. } = trigger.event();
    if !matches!(next, ReaperAnim::Throw) {
        return;
    }
    let Ok(pos) = pos_q.get(trigger.entity()) else {
        return;
    };
    let Ok(slid) = slid_q.get(trigger.entity()) else {
        return;
    };
    commands
        .spawn(ScytheBundle::new(
            pos.clone(),
            -consts.scythe_max_speed_out,
            trigger.entity(),
            slid.iid.clone(),
        ))
        .set_parent(root.eid());
    global_shift.add(0.175, 1);
    // global_shift.add(0.25, 1);
    commands.spawn(SoundEffect::ReaperThrow);
}

#[derive(Component)]
struct Scythe {
    // Just to make sure it despawns at some point
    ttl: f32,
    fake_parent: Entity,
    has_turned_around: bool,
}

#[derive(Bundle)]
struct ScytheBundle {
    name: Name,
    marker: Scythe,
    spawned_lid: SpawnedLid,
    pos: Pos,
    dyno: Dyno,
    transform: Transform,
    visibility: Visibility,
    anim: AnimMan<ScytheAnim>,
    light: Light<LightStatic64Anim>,
    trigger_tx: TriggerTx,
}
impl ScytheBundle {
    fn new(pos: Pos, x_vel: f32, fake_parent: Entity, spawned_lid: String) -> Self {
        Self {
            name: Name::new("scythe"),
            marker: Scythe {
                ttl: 30.0,
                fake_parent,
                has_turned_around: false,
            },
            spawned_lid: SpawnedLid::new(spawned_lid),
            pos,
            dyno: Dyno::new(x_vel, 0.0),
            transform: pos.to_transform(ZIX_PLAYER - 0.4),
            visibility: Visibility::Inherited,
            anim: AnimMan::default()
                .with_observe_state_changes()
                .with_observe_ix_changes(),
            light: default(),
            trigger_tx: TriggerTx::single(TriggerTxKind::Kill, HBox::new(18, 18)),
        }
    }
}

fn maybe_despawn_scythes(
    mut commands: Commands,
    mut scythes: Query<(Entity, &Pos, &mut Scythe)>,
    pos_q: Query<&Pos>,
    time: Res<Time>,
) {
    for (eid, pos, mut scythe) in &mut scythes {
        scythe.ttl -= time.delta_secs();
        let parent_pos = pos_q.get(scythe.fake_parent);
        if scythe.ttl < 0.0 || parent_pos.is_err() || parent_pos.unwrap().x < pos.x - 8.0 {
            if let Some(comms) = commands.get_entity(eid) {
                comms.despawn_recursive();
            }
        }
    }
}

fn maybe_turnaround_scythes(
    mut scythes: Query<(&Pos, &mut Scythe)>,
    player_q: Query<&Pos, With<Player>>,
    consts: Res<ReaperAttackConsts>,
) {
    let Ok(player_pos) = player_q.get_single() else {
        return;
    };
    for (pos, mut scythe) in &mut scythes {
        if !scythe.has_turned_around && player_pos.x - pos.x > consts.scythe_overshoot_turnaround {
            scythe.has_turned_around = true;
        }
    }
}

fn scythe_movement(
    mut scythes: Query<(&mut Dyno, &Scythe), With<Scythe>>,
    bullet_time: Res<BulletTime>,
    consts: Res<ReaperAttackConsts>,
) {
    for (mut dyno, scythe) in &mut scythes {
        if scythe.has_turned_around {
            dyno.vel.x += consts.scythe_acc * bullet_time.delta_secs();
            dyno.vel.x = dyno
                .vel
                .x
                .clamp(-consts.scythe_max_speed_out, consts.scythe_max_speed_back);
        }
    }
}

fn spawn_scythe_particles(
    trigger: Trigger<AnimIxChange<ScytheAnim>>,
    mut commands: Commands,
    pos_q: Query<&Pos>,
    root: Res<WorldDetailRoot>,
) {
    let AnimIxChange { state, ix } = trigger.event().clone();
    let part_anim = match (state, ix) {
        (ScytheAnim::Out, 0) => ScythePartAnim::Part0,
        (ScytheAnim::Out, 1) => ScythePartAnim::Part1,
        (ScytheAnim::Out, 2) => ScythePartAnim::Part2,
        (ScytheAnim::Out, 3) => ScythePartAnim::Part3,
        (ScytheAnim::Out, 4) => ScythePartAnim::Part4,
        (ScytheAnim::Out, 5) => ScythePartAnim::Part5,
        (ScytheAnim::Out, 6) => ScythePartAnim::Part6,
        (ScytheAnim::Out, 7) => ScythePartAnim::Part7,
        _ => return,
    };
    let Ok(pos) = pos_q.get(trigger.entity()) else {
        return;
    };
    commands
        .spawn(EphemeralAnim::<ScythePartAnim>::new(
            part_anim,
            false,
            pos.clone(),
            ZIX_PLAYER - 0.9,
        ))
        .set_parent(root.eid());
}

fn despawn_scythes_out_of_level(
    level_scroll: Res<State<LevelScrollStateKind>>,
    scythes_q: Query<Entity, (With<Scythe>, With<SpawnedLidInactive>)>,
    mut commands: Commands,
) {
    if matches!(level_scroll.get(), LevelScrollStateKind::Some) {
        // Wait till we finish scrolling
        return;
    }
    for eid in &scythes_q {
        if let Some(comms) = commands.get_entity(eid) {
            comms.despawn_recursive();
        }
    }
}

pub(super) fn register_reaper_attack(app: &mut App) {
    app.insert_resource(ReaperAttackConsts::default());
    debug_resource!(app, ReaperAttackConsts);

    app.add_plugins(MyLdtkEntityPlugin::<ReaperAttackBundle>::new(
        "Entities",
        "ReaperAttack",
    ));

    app.add_observer(watch_reaper_anim_to_spawn_scythe);
    app.add_observer(spawn_scythe_particles);

    app.add_systems(OnEnter(PlayerMetaState::Spawning), on_respawn);

    app.add_systems(
        Update,
        (
            manage_appear_disappear,
            manage_onscreen_movement,
            simple_throw,
            reign_in_charge_indicator,
            maybe_despawn_scythes,
            maybe_turnaround_scythes,
            scythe_movement,
            despawn_scythes_out_of_level,
        )
            .chain()
            .run_if(in_state(MetaStateKind::World))
            .after(PhysicsSet)
            .after(CameraSet),
    );
}
