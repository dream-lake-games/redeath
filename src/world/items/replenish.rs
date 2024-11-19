use crate::prelude::*;

#[derive(Bundle)]
struct ReplenishBundle {
    name: Name,
    pos: Pos,
    spatial: SpatialBundle,
    trigger_tx: TriggerTx,
    anim: AnimMan<ReplenishAnim>,
    light: Light<ReplenishLightAnim>,
    bob: Bob,
}
impl ReplenishBundle {
    fn trigger_tx() -> TriggerTx {
        let hbox = HBox::new(10, 10);
        TriggerTx::single(TriggerTxKind::Replenish, hbox)
    }

    fn replenish_time() -> f32 {
        3.0
    }
}
impl MyLdtkEntity for ReplenishBundle {
    type Root = PlatformRoot;
    fn from_ldtk(pos: Pos, _fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        Self {
            name: Name::new("replenish"),
            pos,
            spatial: pos.to_spatial(ZIX_ITEMS - 0.2),
            trigger_tx: Self::trigger_tx(),
            anim: AnimMan::default().with_initial_ix(thread_rng().gen_range(0..10)),
            light: default(),
            bob: Bob::vert(pos, 3.0, 1.1),
        }
    }
}

#[derive(Component)]
struct Replenishing(f32);

fn maybe_break(
    mut replenishes: Query<
        (
            Entity,
            &mut AnimMan<ReplenishAnim>,
            &mut AnimMan<ReplenishLightAnim>,
        ),
        Without<TriggerTxCtrl>,
    >,
    mut commands: Commands,
) {
    for (eid, mut anim, mut light) in &mut replenishes {
        if anim.get_state() == ReplenishAnim::Pulse {
            anim.set_state(ReplenishAnim::Break);
            commands
                .entity(eid)
                .insert(Replenishing(ReplenishBundle::replenish_time()));
            light.set_state(ReplenishLightAnim::None);
            commands.spawn(SoundEffect::ReplenishBreak);
        }
    }
}

fn maybe_spawn(
    bullet_time: Res<BulletTime>,
    mut replenishes: Query<(
        Entity,
        &mut Replenishing,
        &mut AnimMan<ReplenishAnim>,
        &mut AnimMan<ReplenishLightAnim>,
    )>,
    mut commands: Commands,
) {
    for (eid, mut replenishing, mut anim, mut light) in &mut replenishes {
        replenishing.0 -= bullet_time.delta_seconds();
        if replenishing.0 <= 0.0 {
            commands.entity(eid).remove::<Replenishing>();
            commands.entity(eid).insert(ReplenishBundle::trigger_tx());
            anim.set_state(ReplenishAnim::Spawn);
            light.set_state(ReplenishLightAnim::Spawn);
            commands.spawn(SoundEffect::ReplenishSpawn);
        }
    }
}

fn reset_replenishes_after_dying(
    mut replenishes: Query<
        (
            Entity,
            &mut AnimMan<ReplenishAnim>,
            &mut AnimMan<ReplenishLightAnim>,
            Option<&TriggerTxCtrl>,
        ),
        Without<TriggerTxCtrl>,
    >,
    mut commands: Commands,
) {
    for (eid, mut anim, mut light, ttx_ctrl) in &mut replenishes {
        commands.entity(eid).remove::<Replenishing>();
        if ttx_ctrl.is_none() {
            commands.entity(eid).insert(ReplenishBundle::trigger_tx());
        }
        anim.set_state(ReplenishAnim::Pulse);
        light.set_state(ReplenishLightAnim::Pulse);
    }
}

pub(super) fn register_replenish(app: &mut App) {
    app.add_plugins(MyLdtkEntityPlugin::<ReplenishBundle>::new(
        "Entities",
        "Replenish",
    ));

    app.add_systems(
        Update,
        (maybe_break, maybe_spawn)
            .chain()
            .after(PlayerSet)
            .run_if(in_state(MetaStateKind::World)),
    );

    app.add_systems(
        OnEnter(PlayerMetaState::Spawning),
        reset_replenishes_after_dying,
    );
}
