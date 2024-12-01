use crate::prelude::*;

#[derive(Resource, Reflect)]
struct ReaperAttackConsts {
    pad_from_screen_edge: f32,
    body_acc: f32,
    body_max_speed: f32,
    body_close_destroy: f32,
    scythe_acc: f32,
    scythe_max_speed: f32,
}
impl Default for ReaperAttackConsts {
    fn default() -> Self {
        Self {
            pad_from_screen_edge: 24.0,
            body_acc: 240.0,
            body_max_speed: 80.0,
            body_close_destroy: 16.0,
            scythe_acc: 160.0,
            scythe_max_speed: 160.0,
        }
    }
}

#[derive(Component)]
struct ReaperAttack;

#[derive(Bundle)]
struct ReaperAttackBundle {
    name: Name,
    marker: ReaperAttack,
    pos: Pos,
    dyno: Dyno,
    spatial: SpatialBundle,
    anim: AnimMan<ReaperAnim>,
    light: Light<LightStatic64Anim>,
}
impl MyLdtkEntity for ReaperAttackBundle {
    type Root = WorldMetaRoot;
    fn from_ldtk(pos: Pos, _fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        Self {
            name: Name::new("reaper_attack"),
            marker: ReaperAttack,
            pos,
            dyno: default(),
            spatial: pos.to_spatial(ZIX_PLAYER - 0.5),
            anim: AnimMan::default().with_state(ReaperAnim::None),
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
                            anim.set_state(ReaperAnim::Appear);
                        }
                    }
                    ReaperAnim::DisappearDespawn | ReaperAnim::DisappearNone => {
                        // Do nothing
                    }
                    _ => {
                        // Maybe despawn
                        if player_pos.x + consts.body_close_destroy > pos.x {
                            // Yes! Despawn!
                            anim.set_state(ReaperAnim::DisappearNone);
                            dyno.vel.y *= 0.3;
                        }
                    }
                }
            }
            None => {
                anim.set_state(ReaperAnim::None);
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

        if !matches!(anim.get_state(), ReaperAnim::Idle) {
            continue;
        }
        if (player_pos.y - pos.y).abs() < 3.0 {
            dyno.vel.y -= dyno.vel.y.signum() * consts.body_acc * bullet_time.delta_seconds();
        } else {
            let dir = (player_pos.y - pos.y).signum();
            dyno.vel.y += dir * consts.body_acc * bullet_time.delta_seconds();
        }
        dyno.vel.y = dyno
            .vel
            .y
            .clamp(-consts.body_max_speed, consts.body_max_speed);
    }
}

// Resets the y and appears
fn on_respawn(mut reapers: Query<&mut AnimMan<ReaperAnim>, With<ReaperAttack>>) {
    for mut anim in &mut reapers {
        anim.set_state(ReaperAnim::None);
    }
}

pub(super) fn register_reaper_attack(app: &mut App) {
    app.insert_resource(ReaperAttackConsts::default());
    debug_resource!(app, ReaperAttackConsts);

    app.add_plugins(MyLdtkEntityPlugin::<ReaperAttackBundle>::new(
        "Entities",
        "ReaperAttack",
    ));

    app.add_systems(
        Update,
        (manage_appear_disappear, manage_onscreen_movement)
            .chain()
            .run_if(in_state(MetaStateKind::World))
            .after(PhysicsSet)
            .after(CameraSet),
    );
    app.add_systems(OnEnter(PlayerMetaState::Spawning), on_respawn);
}
