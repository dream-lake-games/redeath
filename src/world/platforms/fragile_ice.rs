use crate::prelude::*;

#[derive(Resource, Clone, Debug, Reflect)]
struct FragileIceConsts {
    respawn_time: f32,
}
impl Default for FragileIceConsts {
    fn default() -> Self {
        Self { respawn_time: 2.25 }
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
            world
                .commands()
                .spawn(FragileIce8Bundle::new(pos, eid))
                .set_parent(parent);
        });
    }
}

#[derive(Component)]
struct RememberParent {
    parent: Entity,
}

#[derive(Bundle)]
struct FragileIce8Bundle {
    name: Name,
    trigger_rx: TriggerRx,
    pos: Pos,
    spatial: SpatialBundle,
    anim: AnimMan<FragileIce8Anim>,
    remember_parent: RememberParent,
}
impl FragileIce8Bundle {
    fn new(pos: Pos, parent: Entity) -> Self {
        let hbox = HBox::new(8, 8);
        Self {
            name: Name::new("fragile_ice8"),
            trigger_rx: TriggerRx::single(TriggerRxKind::WantStatic, hbox),
            pos,
            spatial: pos.to_spatial(ZIX_FRAGILE_ICE),
            anim: default(),
            remember_parent: RememberParent { parent },
        }
    }
    fn static_tx() -> StaticTx {
        let hbox = HBox::new(8, 8);
        StaticTx::single(StaticTxKind::SolidFragile, hbox)
    }
}

#[derive(Bundle)]
struct FragileIce8ParentBundle {
    name: Name,
    stable: ParentStable,
    pos: Pos,
}
impl MyLdtkIntCell for FragileIce8ParentBundle {
    type Root = PlatformRoot;
    type RenderLayers = DummyLayer;
    type LeftoverRenderLayers = DummyLayer;
    fn from_ldtk(pos: Pos, _value: i32) -> Self {
        Self {
            name: Name::new("fragile_ice8_parent"),
            stable: ParentStable,
            pos,
        }
    }
}

fn bless_static_hitboxes(
    mut fragile8s: Query<
        (Entity, &mut AnimMan<FragileIce8Anim>, &TriggerRxCtrl),
        Without<StaticTxCtrl>,
    >,
    trigger_colls: Res<TriggerColls>,
    mut commands: Commands,
) {
    for (eid, mut anim, trx_ctrl) in &mut fragile8s {
        if !matches!(anim.get_state(), FragileIce8Anim::ReadyToRespawn) {
            continue;
        }
        if trigger_colls
            .get_refs(&trx_ctrl.coll_keys)
            .iter()
            .any(|coll| coll.tx_kind == TriggerTxKind::Player)
        {
            continue;
        }
        anim.set_state(FragileIce8Anim::Respawn);
        commands.entity(eid).insert(FragileIce8Bundle::static_tx());
        commands.entity(eid).remove::<TriggerRxCtrl>();
    }
}

fn break_ice(
    mut broken: Query<
        (Entity, &mut AnimMan<FragileIce8Anim>, &RememberParent),
        With<FragileBroken>,
    >,
    mut commands: Commands,
    consts: Res<FragileIceConsts>,
) {
    for (eid, mut anim, remember) in &mut broken {
        anim.set_state(FragileIce8Anim::Shatter);
        commands.entity(eid).remove::<FragileBroken>();
        commands.entity(eid).remove::<StaticTxCtrl>();
        commands.entity(remember.parent).remove::<ParentStable>();
        commands.entity(remember.parent).insert(ParentWaiting {
            time_left: consts.respawn_time,
        });
    }
}

fn update_waiting_parents(
    mut waiting_parents: Query<(Entity, &mut ParentWaiting)>,
    mut commands: Commands,
    bullet_time: Res<BulletTime>,
) {
    for (eid, mut waiting) in &mut waiting_parents {
        waiting.time_left -= bullet_time.delta_seconds();
        if waiting.time_left <= 0.0 {
            commands.entity(eid).remove::<ParentWaiting>();
            commands.entity(eid).insert(ParentStable);
        }
    }
}

pub(super) fn register_fragile_ice(app: &mut App) {
    app.add_plugins(MyLdtkIntCellPlugin::<FragileIce8ParentBundle>::single(
        "CommonPlatforms",
        2,
    ));
    app.insert_resource(FragileIceConsts::default());

    app.add_systems(
        Update,
        (bless_static_hitboxes, break_ice, update_waiting_parents)
            .run_if(in_state(MetaStateKind::World))
            .after(PhysicsSet),
    );
}
