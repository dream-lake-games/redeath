use crate::prelude::*;

#[derive(Component)]
struct OgState(SwitchBlockAnim);

#[derive(Bundle)]
struct SwitchBlockBundle {
    name: Name,
    pos: Pos,
    spatial: SpatialBundle,
    dynamic_anim: AnimMan<SwitchBlockAnim>,
    core_anim: AnimMan<SwitchBlockCoreAnim>,
    og_state: OgState,
}
impl MyLdtkIntCell for SwitchBlockBundle {
    type Root = PlatformRoot;
    type RenderLayers = DummyLayer;
    type LeftoverRenderLayers = DummyLayer;
    fn from_ldtk(pos: Pos, value: i32) -> Self {
        let anim = if value == 4 {
            SwitchBlockAnim::On
        } else {
            SwitchBlockAnim::Off
        };
        Self {
            name: Name::new("switch_block"),
            pos,
            spatial: pos.to_spatial(ZIX_SWITCH_BLOCK),
            dynamic_anim: AnimMan::new(anim),
            core_anim: default(),
            og_state: OgState(anim),
        }
    }
}

fn on_dash(
    _trigger: Trigger<DashEvent>,
    mut switch_blocks: Query<(&Pos, &mut AnimMan<SwitchBlockAnim>), With<SpawnedLidActive>>,
    mut commands: Commands,
) {
    for (pos, mut anim) in &mut switch_blocks {
        let current_state = anim.get_state();
        anim.set_state(match current_state {
            SwitchBlockAnim::On => SwitchBlockAnim::Off,
            SwitchBlockAnim::Off => SwitchBlockAnim::On,
        });
        commands.spawn(EphemeralAnim::new(
            match current_state {
                SwitchBlockAnim::On => SwitchBlockEffectAnim::TurnOff,
                SwitchBlockAnim::Off => SwitchBlockEffectAnim::TurnOn,
            },
            false,
            pos.clone(),
            ZIX_SWITCH_BLOCK + 0.001,
        ));
    }
    if switch_blocks.iter().count() > 0 {
        commands.spawn(SoundEffect::SwitchBlockClick);
    }
}

fn update_physics_to_match_anim(
    switch_blocks: Query<(
        Entity,
        &AnimMan<SwitchBlockAnim>,
        Option<&TriggerTxCtrl>,
        Option<&StaticTxCtrl>,
    )>,
    trigger_colls: Res<TriggerColls>,
    mut commands: Commands,
) {
    for (eid, anim, maybe_trigger, maybe_static) in &switch_blocks {
        match anim.get_state() {
            SwitchBlockAnim::On => match (maybe_trigger, maybe_static) {
                (None, None) => {
                    commands
                        .entity(eid)
                        .insert(TriggerTx::single(TriggerTxKind::Spikes, HBox::new(7, 7)));
                }
                (Some(ttx_ctrl), None) => {
                    if trigger_colls
                        .get_refs(&ttx_ctrl.coll_keys)
                        .iter()
                        .all(|coll| coll.rx_kind != TriggerRxKind::Player)
                    {
                        commands.entity(eid).remove::<TriggerTxCtrl>();
                        commands
                            .entity(eid)
                            .insert(StaticTx::single(StaticTxKind::Solid, HBox::new(8, 8)));
                    }
                }
                (Some(_), Some(_)) => {
                    panic!("ahh wtf");
                }
                (None, Some(_)) => (),
            },
            SwitchBlockAnim::Off => {
                commands.entity(eid).remove::<TriggerTxCtrl>();
                commands.entity(eid).remove::<StaticTxCtrl>();
            }
        }
    }
}

fn reset_to_og_state(
    _trigger: Trigger<EnterOrRespawnLevelEvent>,
    mut switch_blocks: Query<(&OgState, &mut AnimMan<SwitchBlockAnim>), With<SpawnedLidActive>>,
) {
    for (og_state, mut anim) in &mut switch_blocks {
        anim.set_state(og_state.0);
    }
}

pub(super) fn register_switch_block(app: &mut App) {
    app.add_plugins(MyLdtkIntCellPlugin::<SwitchBlockBundle>::multiple(
        "CommonPlatforms",
        vec![4, 5],
    ));

    app.observe(on_dash);
    app.observe(reset_to_og_state);

    app.add_systems(
        Update,
        update_physics_to_match_anim
            .run_if(in_state(PlayerMetaState::Playing))
            .after(PhysicsSet)
            .after(PlayerSet)
            .after(AnimSet),
    );
}
