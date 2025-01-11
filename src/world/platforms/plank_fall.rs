use crate::prelude::*;

#[derive(Resource, Clone, Debug, Reflect)]
struct PlankFallConsts {
    respawn_time: f32,
    plank_gravity: f32,
    max_ver_speed: f32,
}
impl Default for PlankFallConsts {
    fn default() -> Self {
        Self {
            respawn_time: 2.25,
            plank_gravity: 0.33,
            max_ver_speed: 80.0,
        }
    }
}

struct ParentStable;
#[derive(Component)]
struct ParentWaiting {
    time_left: f32,
}

impl Component for ParentStable {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let pos = world.get::<Pos>(eid).unwrap().clone();
            let parent = world.resource::<PlatformRoot>().eid();
            let child = world
                .commands()
                .spawn(PlankFallBundle::new(pos, eid))
                .set_parent(parent)
                .id();
            world.commands().entity(eid).insert(RememberChild { child });
        });
    }
}

struct RememberChild {
    child: Entity,
}
impl Component for RememberChild {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_remove(|mut world, eid, _| {
            let remember_child = world.get::<RememberChild>(eid).expect("myself").child;
            if let Some(comms) = world.commands().get_entity(remember_child) {
                comms.despawn_recursive();
            }
        });
    }
}

#[derive(Component)]
struct RememberParent {
    parent: Entity,
}

#[derive(Bundle)]
struct PlankFallBundle {
    name: Name,
    trigger_rx: TriggerRx,
    pos: Pos,
    transform: Transform,
    visibility: Visibility,
    anim: AnimMan<PlankFallAnim>,
    remember_parent: RememberParent,
}
impl PlankFallBundle {
    fn new(pos: Pos, parent: Entity) -> Self {
        let hbox = HBox::new(8, 8);
        Self {
            name: Name::new("plank_fall"),
            trigger_rx: TriggerRx::single(TriggerRxKind::WantStatic, hbox),
            pos,
            transform: pos.to_transform(ZIX_PLANK_FALL),
            visibility: Visibility::Inherited,
            anim: default(),
            remember_parent: RememberParent { parent },
        }
    }
    fn static_tx() -> StaticTx {
        let hbox = HBox::new(8, 8);
        StaticTx::single(StaticTxKind::PassUp, hbox)
    }
}

#[derive(Component)]
struct PlankFallCanary;
#[derive(Bundle)]
struct PlankFallCanaryBundle {
    name: Name,
    canary: PlankFallCanary,
    pos: Pos,
    static_rx: StaticRx,
}
impl PlankFallCanaryBundle {
    fn new(pos: Pos) -> Self {
        Self {
            name: Name::new("plank_fall_canary"),
            canary: PlankFallCanary,
            pos,
            static_rx: StaticRx::single(StaticRxKind::Observe, HBox::new(8, 8)),
        }
    }
}

#[derive(Bundle)]
struct PlankFallParentBundle {
    name: Name,
    stable: ParentStable,
    pos: Pos,
}
impl MyLdtkIntCell for PlankFallParentBundle {
    type Root = PlatformRoot;
    type RenderLayers = DummyLayer;
    type LeftoverRenderLayers = StaticLayer;
    fn from_ldtk(pos: Pos, _value: i32) -> Self {
        Self {
            name: Name::new("plank_fall_parent"),
            stable: ParentStable,
            pos,
        }
    }
}

fn bless_static_hitboxes(
    planks: Query<(Entity, &AnimMan<PlankFallAnim>, &TriggerRxCtrl), Without<StaticTxCtrl>>,
    trigger_colls: Res<TriggerColls>,
    mut commands: Commands,
) {
    for (eid, anim, trx_ctrl) in &planks {
        if !matches!(anim.get_state(), PlankFallAnim::Stable) {
            continue;
        }
        if trigger_colls
            .get_refs(&trx_ctrl.coll_keys)
            .iter()
            .any(|coll| coll.tx_kind == TriggerTxKind::Player)
        {
            continue;
        }
        commands.entity(eid).insert(PlankFallBundle::static_tx());
        commands.entity(eid).remove::<TriggerRxCtrl>();
    }
}

fn update_shaking(
    mut planks: Query<(
        Entity,
        &mut AnimMan<PlankFallAnim>,
        &RememberParent,
        &StaticTxCtrl,
        &Pos,
    )>,
    static_colls: Res<StaticColls>,
    mut commands: Commands,
    player_q: Query<(&Dyno, &Pos), With<Player>>,
    consts: Res<PlankFallConsts>,
) {
    for (eid, mut anim, remember, stx_ctrl, pos) in &mut planks {
        let player_on_top = static_colls
            .get_refs(&stx_ctrl.coll_keys)
            .iter()
            .any(|coll| {
                if let Ok((player_dyno, player_pos)) = player_q.get(coll.rx_ctrl) {
                    if player_dyno.vel.y < -0.1 && player_pos.y > pos.y + 10.5 {
                        return true;
                    }
                }
                false
            });
        if player_on_top && anim.get_state() == PlankFallAnim::Stable {
            anim.set_state(PlankFallAnim::Shaking);
            anim.set_flip_x(thread_rng().gen_bool(0.5));
        }
        if anim.get_state() == PlankFallAnim::Falling {
            let parent = remember.parent;
            commands.entity(parent).remove::<ParentStable>();
            commands.entity(parent).insert(ParentWaiting {
                time_left: consts.respawn_time,
            });
            commands.entity(eid).remove::<RememberParent>();
            commands
                .entity(eid)
                .insert((Dyno::default(), Gravity::new(consts.plank_gravity)));
            commands
                .spawn(PlankFallCanaryBundle::new(*pos))
                .set_parent(eid);
        }
    }
}

fn update_waiting_parents(
    mut waiting_parents: Query<(Entity, &mut ParentWaiting)>,
    mut commands: Commands,
    bullet_time: Res<BulletTime>,
) {
    for (eid, mut waiting) in &mut waiting_parents {
        waiting.time_left -= bullet_time.delta_secs();
        if waiting.time_left <= 0.0 {
            commands.entity(eid).remove::<ParentWaiting>();
            commands.entity(eid).insert(ParentStable);
        }
    }
}

fn cap_falling_speed(
    mut planks: Query<&mut Dyno, With<AnimMan<PlankFallAnim>>>,
    consts: Res<PlankFallConsts>,
) {
    for mut dyno in &mut planks {
        if dyno.vel.y < -consts.max_ver_speed {
            dyno.vel.y = -consts.max_ver_speed;
        }
    }
}

fn update_canaries(
    mut commands: Commands,
    static_colls: Res<StaticColls>,
    mut canaries: Query<(&mut Pos, &Parent, &StaticRxCtrl), With<PlankFallCanary>>,
    mut planks: Query<(Entity, &Pos, &mut AnimMan<PlankFallAnim>), Without<PlankFallCanary>>,
) {
    for (mut canary_pos, parent, srx_ctrl) in &mut canaries {
        let dad = parent.get();
        let (dad_eid, dad_pos, mut dad_anim) = planks.get_mut(dad).unwrap();
        if static_colls
            .get_refs(&srx_ctrl.coll_keys)
            .iter()
            .any(|coll| coll.tx_ctrl != dad)
        {
            commands.entity(dad_eid).remove::<Dyno>();
            commands.entity(dad_eid).remove::<Gravity>();
            commands.entity(dad_eid).remove::<StaticTxCtrl>();
            dad_anim.set_state(PlankFallAnim::Fade);
        } else {
            *canary_pos = *dad_pos;
        }
    }
}

fn reset_on_spawn(
    currently_falling: Query<Entity, (With<AnimMan<PlankFallAnim>>, With<Gravity>)>,
    waiting_parents: Query<Entity, With<ParentWaiting>>,
    mut commands: Commands,
) {
    for eid in currently_falling.iter() {
        if let Some(comms) = commands.get_entity(eid) {
            comms.despawn_recursive();
        }
    }
    for eid in &waiting_parents {
        if let Some(mut comms) = commands.get_entity(eid) {
            comms.remove::<ParentWaiting>();
            comms.insert(ParentStable);
        }
    }
}

pub(super) fn register_plank_fall(app: &mut App) {
    app.add_plugins(MyLdtkIntCellPlugin::<PlankFallParentBundle>::single(
        "CommonPlatforms",
        1,
    ));
    app.insert_resource(PlankFallConsts::default());

    app.add_systems(
        Update,
        (
            bless_static_hitboxes,
            update_shaking,
            update_waiting_parents,
            cap_falling_speed,
            update_canaries,
        )
            .run_if(in_state(MetaStateKind::World))
            .after(PhysicsSet),
    );

    app.add_systems(OnEnter(PlayerMetaState::Spawning), reset_on_spawn);
}
