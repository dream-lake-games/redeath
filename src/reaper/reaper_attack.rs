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
            anim: default(),
            light: default(),
        }
    }
}

fn update_onscreen_dynos(
    mut reapers: Query<
        (
            &mut Dyno,
            &mut Pos,
            &mut Transform,
            &mut AnimMan<ReaperAnim>,
        ),
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
    for (mut dyno, mut pos, mut tran, mut anim) in &mut reapers {
        // Have to set both pos and transform so it doesn't have weird jitter
        pos.x = proper_x;
        tran.translation.x = proper_x;

        if !matches!(anim.get_state(), ReaperAnim::Idle) {
            continue;
        }
        if (player_pos.y - pos.y).abs() < 6.0 {
            dyno.vel.y -= dyno.vel.y.signum() * consts.body_acc * bullet_time.delta_seconds();
        } else {
            let dir = (player_pos.y - pos.y).signum();
            dyno.vel.y += dir * consts.body_acc * bullet_time.delta_seconds();
        }
        dyno.vel.y = dyno
            .vel
            .y
            .clamp(-consts.body_max_speed, consts.body_max_speed);

        // Maybe destroy if player is close
        if (player_pos.x - pos.x).abs() < consts.body_close_destroy {
            anim.set_state(ReaperAnim::DisappearNone);
            dyno.vel.y *= 0.3;
        }
    }
}

fn update_offscreen_dynos(
    mut reapers: Query<
        (&mut Dyno, &mut Pos),
        (
            With<ReaperAttack>,
            With<SpawnedLidInactive>,
            Without<Player>,
        ),
    >,
    player_q: Query<&Pos, With<Player>>,
) {
    let Ok(player_pos) = player_q.get_single() else {
        return;
    };
    for (mut dyno, mut pos) in &mut reapers {
        dyno.vel = Vec2::ZERO;
        pos.y = player_pos.y;
    }
}

fn appear_on_spawned_lid_active(
    _trigger: Trigger<LevelChangeEvent>,
    mut reapers: Query<&mut AnimMan<ReaperAnim>, (With<ReaperAttack>, With<SpawnedLidActive>)>,
) {
    for mut anim in &mut reapers {
        anim.set_state(ReaperAnim::Appear);
    }
}

// Resets the y and appears
fn on_respawn(
    mut reapers: Query<
        (&mut Dyno, &mut Pos, &mut AnimMan<ReaperAnim>),
        (
            With<ReaperAttack>,
            Without<Player>,
            Without<SpawnPointActive>,
        ),
    >,
    spawn_q: Query<&Pos, With<SpawnPointActive>>,
) {
    let Ok(spawn_pos) = spawn_q.get_single() else {
        return;
    };
    for (mut dyno, mut pos, mut anim) in &mut reapers {
        dyno.vel = Vec2::ZERO;
        pos.y = spawn_pos.y;
        anim.set_state(ReaperAnim::Appear);
    }
}

pub(super) fn register_reaper_attack(app: &mut App) {
    app.insert_resource(ReaperAttackConsts::default());
    debug_resource!(app, ReaperAttackConsts);

    app.add_plugins(MyLdtkEntityPlugin::<ReaperAttackBundle>::new(
        "Entities",
        "ReaperAttack",
    ));

    app.observe(appear_on_spawned_lid_active);

    app.add_systems(
        Update,
        (update_onscreen_dynos, update_offscreen_dynos)
            .run_if(in_state(MetaStateKind::World))
            .after(PhysicsSet)
            .after(CameraSet),
    );
    app.add_systems(OnEnter(PlayerMetaState::Spawning), on_respawn);
}
