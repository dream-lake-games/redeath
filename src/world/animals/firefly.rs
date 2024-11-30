use crate::prelude::*;

#[derive(Resource, Clone, Debug, Reflect)]
struct FireflyConsts {
    /// How long between flaps?
    flap_time_range: Range<f32>,
    /// How strong is a flap?
    flap_strength_range: Range<f32>,
    /// What is the range in x speed when taking off?
    launch_x_range: Range<f32>,
    /// How strong is gravity?
    firefly_gravity: f32,
    max_ver_speed: f32,
    /// Chance of going towards player from ground
    ground_wrong_x_prob: f32,
}
impl Default for FireflyConsts {
    fn default() -> Self {
        Self {
            flap_time_range: 0.05..0.7,
            flap_strength_range: 3.0..40.0,
            firefly_gravity: 0.2,
            launch_x_range: 15.0..30.0,
            max_ver_speed: 100.0,
            ground_wrong_x_prob: 0.16,
        }
    }
}

#[derive(Component, Default)]
struct Firefly {
    time_till_next_flap: f32,
    leave_before_fly: bool,
}

#[derive(Bundle)]
struct FireflyBundle {
    name: Name,
    firefly: Firefly,
    pos: Pos,
    spatial: SpatialBundle,
    dyno: Dyno,
    gravity: Gravity,
    static_rx: StaticRx,
    trigger_rx: TriggerRx,
    anim: AnimMan<FireflyAnim>,
    light: Light<FireflyLightAnim>,
}
impl FireflyBundle {
    pub fn new(pos: Pos) -> Self {
        let static_hbox = HBox::new(2, 2).translated(0.0, -1.0);
        let fly_hbox = HBox::new(40, 40);

        Self {
            name: Name::new("firefly"),
            firefly: Firefly::default(),
            pos,
            spatial: pos.to_spatial(ZIX_PLAYER + thread_rng().gen_range(0.1..0.9)),
            dyno: default(),
            gravity: Gravity::new(0.2), // Will get overridden
            static_rx: StaticRx::single(StaticRxKind::Default, static_hbox),
            trigger_rx: TriggerRx::single(TriggerRxKind::Firefly, fly_hbox),
            anim: AnimMan::new(FireflyAnim::RestGround).with_observe_state_changes(),
            light: default(),
        }
    }
}
impl MyLdtkEntity for FireflyBundle {
    type Root = WorldDetailRoot;
    fn from_ldtk(pos: Pos, _fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        FireflyBundle::new(pos)
    }
}

fn update_fireflies(
    mut fireflies: Query<
        (
            Entity,
            &mut Dyno,
            &mut AnimMan<FireflyAnim>,
            &mut Firefly,
            &StaticRxCtrl,
            &TriggerRxCtrl,
        ),
        With<SpawnedLidActive>,
    >,
    static_colls: Res<StaticColls>,
    trigger_colls: Res<TriggerColls>,
    consts: Res<FireflyConsts>,
    mut commands: Commands,
    pos: Query<&Pos>,
    bullet_time: Res<BulletTime>,
) {
    let mut rng = thread_rng();
    for (eid, mut dyno, mut anim, mut firefly, srx_ctrl, trx_ctrl) in &mut fireflies {
        match anim.get_state() {
            FireflyAnim::RestGround | FireflyAnim::RestWall => {
                let player_trigger = trigger_colls
                    .get_refs(&trx_ctrl.coll_keys)
                    .into_iter()
                    .filter(|coll| coll.tx_kind == TriggerTxKind::Player)
                    .next();
                if player_trigger.is_some() && !firefly.leave_before_fly {
                    // Takeoff
                    let player_trigger = player_trigger.unwrap();
                    let x_mult = rng.gen_range(consts.launch_x_range.clone());
                    let x_signum = match anim.get_state() {
                        FireflyAnim::RestGround => {
                            let player_pos = pos.get(player_trigger.tx_ctrl).unwrap();
                            let flip_mult = if rng.gen_bool(consts.ground_wrong_x_prob as f64) {
                                -1.0
                            } else {
                                1.0
                            };
                            if player_pos.x < player_trigger.pos.x {
                                1.0 * flip_mult
                            } else {
                                -1.0 * flip_mult
                            }
                        }
                        FireflyAnim::RestWall => {
                            if anim.get_flip_x() {
                                -1.0
                            } else {
                                1.0
                            }
                        }
                        _ => 1.0,
                    };
                    dyno.vel = Vec2::new(x_mult * x_signum, 50.0);
                    anim.set_flip_x(x_signum < 0.0);
                    anim.set_state(FireflyAnim::Flap);
                    firefly.time_till_next_flap = rng.gen_range(consts.flap_time_range.clone());
                    commands
                        .entity(eid)
                        .insert(Gravity::new(consts.firefly_gravity));
                } else {
                    // Do nothing
                    firefly.leave_before_fly = firefly.leave_before_fly && player_trigger.is_some();
                }
            }
            FireflyAnim::Flap => {
                dyno.vel.y =
                    dyno.vel.y.abs().clamp(0.0, consts.max_ver_speed) * dyno.vel.y.signum();
                if firefly.time_till_next_flap > 0.0 {
                    firefly.time_till_next_flap -= bullet_time.delta_seconds();
                } else {
                    dyno.vel.y = rng.gen_range(consts.flap_strength_range.clone());
                    firefly.time_till_next_flap += rng.gen_range(consts.flap_time_range.clone());
                }
                let coll = static_colls
                    .get_refs(&srx_ctrl.coll_keys)
                    .into_iter()
                    .next();
                if let Some(coll) = coll {
                    if coll.push.y >= 0.0 {
                        // Not pushing down
                        commands.entity(eid).remove::<Gravity>();
                        dyno.vel.x = 0.0;
                        dyno.vel.y = 0.0;
                        firefly.leave_before_fly = true;
                        if coll.push.y > 0.0 {
                            // Pushing up. Assuming ground landing
                            anim.set_state(FireflyAnim::RestGround);
                        } else {
                            // Wall landing
                            anim.set_state(FireflyAnim::RestWall);
                            anim.set_flip_x(coll.push.x < 0.0);
                        }
                    }
                }
            }
        }
    }
}

fn update_firefly_lights(
    trigger: Trigger<AnimStateChange<FireflyAnim>>,
    mut anims: Query<&mut AnimMan<FireflyLightAnim>>,
) {
    let event = trigger.event();
    let light_anim = match event.next {
        FireflyAnim::Flap => FireflyLightAnim::Grow,
        FireflyAnim::RestGround | FireflyAnim::RestWall => FireflyLightAnim::Shrink,
    };
    let mut anim = anims.get_mut(trigger.entity()).unwrap();
    anim.set_state(light_anim);
}

pub(super) fn register_firefly(app: &mut App) {
    app.add_plugins(MyLdtkEntityPlugin::<FireflyBundle>::new(
        "Entities", "Firefly",
    ));
    app.insert_resource(FireflyConsts::default());
    debug_resource!(app, FireflyConsts);

    app.observe(update_firefly_lights);

    app.add_systems(Update, update_fireflies.after(PhysicsSet));
}
