use crate::prelude::*;

#[derive(Component)]
struct CutsceneTrigger {
    convo: String,
}

#[derive(Bundle)]
struct CutsceneTriggerBundle {
    name: Name,
    pos: Pos,
    trigger_tx: TriggerTx,
    convo_trigger: CutsceneTrigger,
}
impl MyLdtkEntity for CutsceneTriggerBundle {
    type Root = WorldMetaRoot;
    fn from_ldtk(pos: Pos, fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        let Some(FieldValue::String(Some(convo))) = fields.get("Cutscene") else {
            panic!("ahh convo_trigger");
        };
        Self {
            name: Name::new("convo_trigger"),
            pos,
            trigger_tx: TriggerTx::single(TriggerTxKind::Observe, HBox::new(32, 32)),
            convo_trigger: CutsceneTrigger {
                convo: convo.clone(),
            },
        }
    }
}

fn update_convo_triggers(
    trigs: Query<(Entity, &CutsceneTrigger, &TriggerTxCtrl)>,
    trigger_colls: Res<TriggerColls>,
    mut commands: Commands,
    mut cutscene_state: ResMut<NextState<CutsceneState>>,
) {
    for (eid, cut_trig, ttx_ctrl) in &trigs {
        if trigger_colls
            .get_refs(&ttx_ctrl.coll_keys)
            .iter()
            .any(|coll| coll.rx_kind == TriggerRxKind::Player)
        {
            match cut_trig.convo.as_str() {
                "CanyonConfrontReaper" => {
                    cutscene_state.set(CutsceneState::CanyonConfrontReaper);
                }
                c => panic!("bad cutscene to start: {c}"),
            }
            commands.entity(eid).despawn_recursive();
        }
    }
}

pub(super) fn register_cutscene_trigger(app: &mut App) {
    app.add_plugins(MyLdtkEntityPlugin::<CutsceneTriggerBundle>::new(
        "Entities",
        "CutsceneStartTrigger",
    ));

    app.add_systems(
        Update,
        update_convo_triggers
            .after(PhysicsSet)
            .run_if(in_state(MetaStateKind::World)),
    );
}
