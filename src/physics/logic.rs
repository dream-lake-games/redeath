use crate::prelude::*;

#[derive(Resource, Clone, Debug, Reflect)]
struct PhysicsConsts {
    gravity_strength: f32,
}
impl Default for PhysicsConsts {
    fn default() -> Self {
        Self {
            gravity_strength: 540.0,
        }
    }
}

/// A helpful function to make sure physics things exist as we expect them to
fn invariants(
    dyno_without_pos: Query<Entity, (With<Dyno>, Without<Pos>)>,
    static_rx_n_tx: Query<Entity, (With<StaticRxCtrl>, With<StaticTxCtrl>)>,
    moving_static_tx_vert_only: Query<&Dyno, With<StaticTxCtrl>>,
) {
    debug_assert!(dyno_without_pos.is_empty());
    debug_assert!(static_rx_n_tx.is_empty());
    for dyno in &moving_static_tx_vert_only {
        debug_assert!(dyno.vel.x.abs() == 0.0);
    }
}

/// Moves dynos that have no receivers or static providers
fn move_uninteresting_dynos(
    bullet_time: Res<BulletTime>,
    mut ents: Query<
        (&Dyno, &mut Pos),
        (
            Without<StaticRxCtrl>,
            Without<TriggerRxCtrl>,
            Without<StaticTxCtrl>,
        ),
    >,
) {
    for (dyno, mut pos) in &mut ents {
        *pos += dyno.vel * bullet_time.delta_secs();
    }
}

/// Moves static txs
fn move_static_txs(
    bullet_time: Res<BulletTime>,
    mut ents: Query<(&Dyno, &mut Pos), (Without<StaticRxCtrl>, With<StaticTxCtrl>)>,
) {
    for (dyno, mut pos) in &mut ents {
        *pos += dyno.vel * bullet_time.delta_secs();
    }
}

/// Resolves collisions for a single entity.
/// If it has statics, it resolves static collisions and may update pos and vel
/// If it has triggers, it will trigger as needed (duh)
fn resolve_collisions(
    my_eid: Entity,
    my_pos: &mut Pos,
    my_vel: &mut Vec2,
    my_srx_comps: &[&StaticRxComp],
    my_trx_comps: &[&TriggerRxComp],
    pos_q: &Query<
        &mut Pos,
        Or<(
            With<StaticRxCtrl>,
            With<StaticTxCtrl>,
            With<TriggerRxCtrl>,
            With<TriggerTxCtrl>,
        )>,
    >,
    stx_comps: &Query<&StaticTxComp>,
    ttx_comps: &Query<&TriggerTxComp>,
    static_coll_counter: &mut CollKey,
    trigger_coll_counter: &mut CollKey,
    static_colls: &mut ResMut<StaticColls>,
    trigger_colls: &mut ResMut<TriggerColls>,
    srx_ctrls: &mut Query<&mut StaticRxCtrl>,
    stx_ctrls: &mut Query<&mut StaticTxCtrl>,
    trx_ctrls: &mut Query<&mut TriggerRxCtrl>,
    ttx_ctrls: &mut Query<&mut TriggerTxCtrl>,
    dyno_q: &Query<&mut Dyno>,
    commands: &mut Commands,
) {
    macro_rules! translate_other {
        ($comp:expr) => {{
            let tmp_pos = pos_q
                .get($comp.ctrl)
                .expect("Bad pos in translate_other")
                .clone();
            $comp.hbox.translated(tmp_pos.x, tmp_pos.y)
        }};
    }
    macro_rules! add_ctrl_coll {
        ($q:expr, $eid:expr, $key:expr) => {{
            match $q.get_mut($eid) {
                Ok(mut thing) => {
                    thing.coll_keys.push($key);
                }
                Err(e) => {
                    warn!("fucky stuff happening in resolve_collisions::add_ctrl_coll: {e:?}");
                }
            };
        }};
    }

    // First handle static collisions
    for my_srx_comp in my_srx_comps {
        let mut my_thbox = my_srx_comp.hbox.translated(my_pos.x, my_pos.y);
        // TODO: Performance engineer if needed
        // In order to avoid weird behavior when sliding along a straight edge, do this
        // First filter to only things it's colliding with
        let mut can_possibly_collide = stx_comps
            .iter()
            .filter(|other_stx_comp| {
                let other_hbox = translate_other!(other_stx_comp);
                my_thbox.overlaps_with(&other_hbox)
            })
            .collect::<Vec<_>>();
        // Then sort by area overlapping
        can_possibly_collide.sort_by(|a, b| {
            let ahbox = translate_other!(a);
            let bhbox = translate_other!(b);
            let dist_a = ahbox.area_overlapping_assuming_overlap(&my_thbox);
            let dist_b = bhbox.area_overlapping_assuming_overlap(&my_thbox);
            dist_b.total_cmp(&dist_a)
        });
        for other_stx_comp in can_possibly_collide {
            if other_stx_comp.ctrl == my_eid {
                // Don't collide with ourselves, stupid
                continue;
            }
            let other_thbox = translate_other!(other_stx_comp);
            if let Some(push) = my_thbox.get_push_out(&other_thbox) {
                // STATIC COLLISION HERE (maybe)
                let tx_dyno = dyno_q.get(other_stx_comp.ctrl).cloned().unwrap_or_default();

                let mut old_perp = my_vel.dot(push.normalize_or_zero()) * push.normalize_or_zero();
                let old_par = *my_vel - old_perp;
                if push.y.abs() > 0.0 {
                    old_perp.y -= tx_dyno.vel.y;
                }

                let coll_rec = StaticCollRec {
                    pos: my_pos.clone(),
                    push,
                    rx_perp: old_perp,
                    rx_par: old_par,
                    rx_ctrl: my_srx_comp.ctrl,
                    rx_kind: my_srx_comp.kind,
                    rx_hbox: my_srx_comp.hbox.get_marker(),
                    tx_ctrl: other_stx_comp.ctrl,
                    tx_kind: other_stx_comp.kind,
                    tx_hbox: other_stx_comp.hbox.get_marker(),
                };

                let add_coll_rec = || {
                    let key = *static_coll_counter;
                    *static_coll_counter += 1;
                    static_colls.insert(key, coll_rec);
                    add_ctrl_coll!(srx_ctrls, my_srx_comp.ctrl, key);
                    add_ctrl_coll!(stx_ctrls, other_stx_comp.ctrl, key);
                };

                let mut do_push = |grr: &mut HBox| {
                    *my_pos += push;
                    // NOTE: HAVE TO UPDATE MY_THBOX HERE SINCE POS CHANGED
                    *grr = grr.translated(push.x, push.y);
                };

                match (my_srx_comp.kind, other_stx_comp.kind) {
                    (StaticRxKind::Default, StaticTxKind::Solid | StaticTxKind::SolidFragile)
                    | (StaticRxKind::DefaultBreaker, StaticTxKind::Solid) => {
                        // Solid collision, no breaking
                        add_coll_rec();
                        do_push(&mut my_thbox);
                        *my_vel = old_par + Vec2::new(0.0, tx_dyno.vel.y);
                        if old_perp.dot(push) > 0.0 {
                            *my_vel += old_perp;
                        }
                    }
                    (StaticRxKind::DefaultBreaker, StaticTxKind::SolidFragile) => {
                        commands.entity(other_stx_comp.ctrl).insert(FragileBroken);
                    }
                    (
                        StaticRxKind::Default | StaticRxKind::DefaultBreaker,
                        StaticTxKind::PassUp,
                    ) => {
                        // Any kind of passup
                        if push.y > 0.0
                            && old_perp.y < 0.0
                            && other_thbox.max_y() - 1.1 < my_thbox.min_y()
                        {
                            add_coll_rec();
                            do_push(&mut my_thbox);
                            *my_vel = old_par + Vec2::new(0.0, tx_dyno.vel.y);
                        }
                    }
                    (StaticRxKind::Observe, _) => {
                        add_coll_rec();
                    }
                }
            }
        }
    }

    // Then handle trigger collisions
    for my_trx_comp in my_trx_comps {
        let my_thbox = my_trx_comp.hbox.translated(my_pos.x, my_pos.y);
        for other_ttx_comp in ttx_comps {
            if other_ttx_comp.ctrl == my_eid {
                // Don't collide with ourselves, stupid
                continue;
            }
            let other_thbox = translate_other!(other_ttx_comp);
            if my_thbox.overlaps_with(&other_thbox) {
                // TRIGGER COLLISION HERE
                let coll_rec = TriggerCollRec {
                    pos: my_pos.clone(),
                    rx_ctrl: my_trx_comp.ctrl,
                    rx_kind: my_trx_comp.kind,
                    rx_hbox: my_trx_comp.hbox.get_marker(),
                    tx_ctrl: other_ttx_comp.ctrl,
                    tx_kind: other_ttx_comp.kind,
                    tx_hbox: other_ttx_comp.hbox.get_marker(),
                };
                let key = *trigger_coll_counter;
                *trigger_coll_counter += 1;
                trigger_colls.insert(key, coll_rec);
                add_ctrl_coll!(trx_ctrls, my_trx_comp.ctrl, key);
                add_ctrl_coll!(ttx_ctrls, other_ttx_comp.ctrl, key);
            }
        }
    }
}

/// Moves the interesting stuff and handles collisions
fn move_interesting_dynos(
    bullet_time: Res<BulletTime>,
    mut pos_q: Query<
        &mut Pos,
        Or<(
            With<StaticRxCtrl>,
            With<StaticTxCtrl>,
            With<TriggerRxCtrl>,
            With<TriggerTxCtrl>,
        )>,
    >,
    mut dyno_q: Query<&mut Dyno>,
    mut srx_ctrls: Query<&mut StaticRxCtrl>,
    mut stx_ctrls: Query<&mut StaticTxCtrl>,
    mut trx_ctrls: Query<&mut TriggerRxCtrl>,
    mut ttx_ctrls: Query<&mut TriggerTxCtrl>,
    srx_comps: Query<&StaticRxComp>,
    stx_comps: Query<&StaticTxComp>,
    trx_comps: Query<&TriggerRxComp>,
    ttx_comps: Query<&TriggerTxComp>,
    // Objects that have a static rx. They may also have a trigger rx.
    ents: Query<
        Entity,
        (
            With<Pos>,
            Without<StaticTxCtrl>,
            Or<(With<StaticRxCtrl>, With<TriggerRxCtrl>)>,
            Without<Offscreen>,
        ),
    >,
    mut static_colls: ResMut<StaticColls>,
    mut trigger_colls: ResMut<TriggerColls>,
    mut commands: Commands,
) {
    let mut static_coll_counter: CollKey = 0;
    let mut trigger_coll_counter: CollKey = 0;

    // First move static rxs
    for eid in &ents {
        // Get the data
        let mut scratch_pos = pos_q.get(eid).expect("No pos on interesting ent").clone();
        let mut scratch_vel = dyno_q.get(eid).unwrap_or(&Dyno::default()).vel.clone();
        macro_rules! get_comps {
            ($ctrl:expr, $comp_query:expr) => {{
                $ctrl
                    .map(|ctrl| {
                        ctrl.comps
                            .iter()
                            .map(|comp_eid| $comp_query.get(*comp_eid))
                            .filter_map(Result::ok)
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or(vec![])
            }};
        }
        let srx_ctrl = srx_ctrls.get(eid).ok();
        let trx_ctrl = trx_ctrls.get(eid).ok();
        let my_srx_comps = get_comps!(srx_ctrl, srx_comps);
        let my_trx_comps = get_comps!(trx_ctrl, trx_comps);
        // Inch
        macro_rules! call_resolve_collisions {
            () => {{
                resolve_collisions(
                    eid,
                    &mut scratch_pos,
                    &mut scratch_vel,
                    &my_srx_comps,
                    &my_trx_comps,
                    &pos_q,
                    &stx_comps,
                    &ttx_comps,
                    &mut static_coll_counter,
                    &mut trigger_coll_counter,
                    &mut static_colls,
                    &mut trigger_colls,
                    &mut srx_ctrls,
                    &mut stx_ctrls,
                    &mut trx_ctrls,
                    &mut ttx_ctrls,
                    &dyno_q,
                    &mut commands,
                );
            }};
        }
        const DELTA_PER_INCH: f32 = 1.0;
        // Resolve collisions once always so stationary objects are still pushed out of each other
        call_resolve_collisions!();
        // Inch horizontally
        let mut amt_moved_hor: f32 = 0.0;
        let max_inch_hor = scratch_vel.x.abs() * bullet_time.delta_secs();
        while amt_moved_hor < max_inch_hor.min(scratch_vel.x.abs()) {
            let dont_overshoot = (max_inch_hor.min(scratch_vel.x.abs()) - amt_moved_hor).max(0.0);
            let moving_this_step = DELTA_PER_INCH.min(dont_overshoot);
            amt_moved_hor += moving_this_step;
            scratch_pos.x += scratch_vel.x.signum() * moving_this_step;
            call_resolve_collisions!();
        }
        // Then inch vertically
        let mut amt_moved_ver: f32 = 0.0;
        let max_inch_ver = scratch_vel.y.abs() * bullet_time.delta_secs();
        while amt_moved_ver < max_inch_ver.min(scratch_vel.y.abs()) {
            let dont_overshoot = (max_inch_ver.min(scratch_vel.y.abs()) - amt_moved_ver).max(0.0);
            let moving_this_step = DELTA_PER_INCH.min(dont_overshoot);
            amt_moved_ver += moving_this_step;
            scratch_pos.y += scratch_vel.y.signum() * moving_this_step;
            call_resolve_collisions!();
        }
        // NOTE: Why do this (inch horizontally then vertically)? Stops bugs going up and down against wall.
        // ^read: celeste does this
        // Set the data
        let mut set_pos = pos_q.get_mut(eid).expect("No pos on interesting ent");
        *set_pos = scratch_pos;
        if let Ok(mut set_dyno) = dyno_q.get_mut(eid) {
            set_dyno.vel = scratch_vel;
        }
    }
}

fn apply_gravity(
    mut ents: Query<(&mut Dyno, &Gravity)>,
    consts: Res<PhysicsConsts>,
    bullet_time: Res<BulletTime>,
) {
    for (mut dyno, grav) in &mut ents {
        dyno.vel.y -= grav.mult * consts.gravity_strength * bullet_time.delta_secs();
    }
}

pub(super) fn register_logic(app: &mut App) {
    app.add_systems(
        Update,
        (
            invariants,
            move_uninteresting_dynos,
            move_static_txs,
            move_interesting_dynos,
            apply_gravity,
        )
            .chain()
            .in_set(PhysicsSet)
            .in_set(super::CollSet)
            .before(super::PosSet)
            .before(MyLdtkMaintSet)
            .run_if(in_state(PhysicsState::Active)),
    );

    app.insert_resource(PhysicsConsts::default());
}
