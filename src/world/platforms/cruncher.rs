use crate::prelude::*;

#[derive(Bundle)]
struct CruncherBundle {
    name: Name,
    pos: Pos,
    spatial: SpatialBundle,
    // trigger: TriggerTx,
    anim: AnimMan<CruncherAnim>,
}
impl CruncherBundle {
    fn observe_ttx() -> TriggerTx {
        TriggerTx::single(TriggerTxKind::Observe, HBox::new(16, 24))
    }
    fn hurt_ttx() -> TriggerTx {
        TriggerTx::single(TriggerTxKind::Spikes, HBox::new(14, 24))
    }
}

impl MyLdtkEntity for CruncherBundle {
    type Root = WorldMetaRoot;
    fn from_ldtk(pos: Pos, _fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        Self {
            name: Name::new("cruncher"),
            pos,
            spatial: pos.to_spatial(ZIX_CRUNCHER),
            // trigger: Self::observe_ttx(),
            anim: AnimMan::default().with_observe_ix_changes(),
        }
    }
}

fn start_crunch(
    mut crunchers: Query<(Entity, &mut AnimMan<CruncherAnim>, &TriggerTxCtrl)>,
    trigger_colls: Res<TriggerColls>,
    mut commands: Commands,
) {
    for (eid, mut anim, ttx_ctrl) in &mut crunchers {
        if anim.get_state() != CruncherAnim::Idle {
            continue;
        }
        if trigger_colls
            .get_refs(&ttx_ctrl.coll_keys)
            .iter()
            .any(|coll| coll.rx_kind == TriggerRxKind::Player)
        {
            anim.set_state(CruncherAnim::Crunch);
            commands.entity(eid).remove::<TriggerTxCtrl>();
        }
    }
}

fn handle_anim_changes(
    trigger: Trigger<AnimIxChange<CruncherAnim>>,
    mut commands: Commands,
    mut shake: ResMut<CameraShake>,
) {
    let AnimIxChange { state, ix } = trigger.event().clone();
    match (state, ix) {
        (CruncherAnim::Crunch, 0) => {
            commands.spawn(SoundEffect::CruncherRealShit);
        }
        (CruncherAnim::Crunch, 7) => {
            commands
                .entity(trigger.entity())
                .insert(CruncherBundle::hurt_ttx());
            shake.shake(0.2, 0..=0, -2..=0);
            commands.spawn(SoundEffect::CruncherReverb);
        }
        (CruncherAnim::Crunch, 12) => {
            commands.entity(trigger.entity()).remove::<TriggerTxCtrl>();
        }
        (CruncherAnim::Idle, _) => {
            commands
                .entity(trigger.entity())
                .insert(CruncherBundle::observe_ttx());
        }
        _ => (),
    }
}

pub(super) fn register_cruncher(app: &mut App) {
    app.add_plugins(MyLdtkEntityPlugin::<CruncherBundle>::new(
        "Entities", "Cruncher",
    ));

    app.observe(handle_anim_changes);

    app.add_systems(
        Update,
        (start_crunch,)
            .chain()
            .run_if(in_state(MetaStateKind::World))
            .run_if(in_state(PlayerMetaState::Playing))
            .after(PhysicsSet),
    );
}
