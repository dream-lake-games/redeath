use crate::prelude::*;

#[derive(Component)]
struct Coin {
    iid: String,
}

#[derive(Bundle)]
struct CoinBundle {
    name: Name,
    marker: Coin,
    pos: Pos,
    spatial: SpatialBundle,
    trigger_tx: TriggerTx,
    anim: AnimMan<CoinAnim>,
    light: Light<CoinLightAnim>,
    bob: Bob,
}
impl CoinBundle {
    fn trigger_tx() -> TriggerTx {
        let hbox = HBox::new(16, 20);
        TriggerTx::single(TriggerTxKind::Coin, hbox)
    }
}
impl MyLdtkEntity for CoinBundle {
    type Root = ItemsRoot;
    fn from_ldtk(pos: Pos, _fields: &HashMap<String, FieldValue>, iid: String) -> Self {
        Self {
            name: Name::new("coin"),
            marker: Coin { iid },
            pos,
            spatial: pos.to_spatial(ZIX_ITEMS + 0.21),
            trigger_tx: Self::trigger_tx(),
            anim: default(),
            light: default(),
            bob: Bob::vert(pos, 3.0, 1.3),
        }
    }
}

/// This is the little coin that chases you after you collect and until you get to the bank
#[derive(Component)]
struct CoinSmol {
    iid: String,
}

#[derive(Bundle)]
struct CoinSmolBundle {
    name: Name,
    marker: CoinSmol,
    pos: Pos,
    dyno: Dyno,
    spatial: SpatialBundle,
    anim: AnimMan<CoinSmolAnim>,
    chase: ChaseEntity,
}
impl CoinSmolBundle {
    fn new(pos: Pos, chase: Entity, iid: String) -> Self {
        Self {
            name: Name::new("coin_smol"),
            marker: CoinSmol { iid },
            pos,
            dyno: default(),
            spatial: pos.to_spatial(ZIX_ITEMS + 1.2),
            anim: default(),
            chase: ChaseEntity::new(chase, 420.0, 320.0, 16.0, 400.0),
        }
    }
}

fn maybe_collect_coins(
    mut coins: Query<(
        Entity,
        &Pos,
        &TriggerTxCtrl,
        &Coin,
        &mut AnimMan<CoinAnim>,
        &mut AnimMan<CoinLightAnim>,
    )>,
    trigger_colls: Res<TriggerColls>,
    mut commands: Commands,
    player_q: Query<Entity, With<Player>>,
    root: Res<WorldMetaRoot>,
) {
    let Ok(player_eid) = player_q.get_single() else {
        return;
    };
    for (eid, pos, ttx_ctrl, coin, mut anim, mut light) in &mut coins {
        if !matches!(anim.get_state(), CoinAnim::Spin) {
            continue;
        }
        if trigger_colls
            .get_refs(&ttx_ctrl.coll_keys)
            .iter()
            .any(|coll| coll.rx_kind == TriggerRxKind::Player)
        {
            anim.set_state(CoinAnim::Pop);
            light.set_state(CoinLightAnim::Pop);
            commands.spawn(SoundEffect::CoinCollect);
            commands.entity(eid).remove::<TriggerTxCtrl>();
            commands
                .spawn(CoinSmolBundle::new(
                    pos.clone(),
                    player_eid,
                    coin.iid.clone(),
                ))
                .set_parent(root.eid());
        }
    }
}

fn reset_coins_per_level(
    _trigger: Trigger<EnterOrRespawnLevelEvent>,
    mut coins: Query<
        (
            Entity,
            &mut AnimMan<CoinAnim>,
            &mut AnimMan<CoinLightAnim>,
            Option<&TriggerTxCtrl>,
        ),
        With<SpawnedLidActive>,
    >,
    mut smols: Query<&mut AnimMan<CoinSmolAnim>>,
    mut banks: Query<&mut AnimMan<BankAnim>>,
    mut commands: Commands,
) {
    for (eid, mut anim, mut light, ttx_ctrl) in &mut coins {
        if ttx_ctrl.is_none() {
            commands.entity(eid).insert(CoinBundle::trigger_tx());
        }
        anim.set_state(CoinAnim::Spin);
        light.set_state(CoinLightAnim::Pulse);
    }
    for mut smol_anim in &mut smols {
        smol_anim.set_state(CoinSmolAnim::Pop);
    }
    for mut bank in &mut banks {
        bank.set_state(BankAnim::None);
    }
}

fn coin_smol_death(
    mut smol_coins: Query<(Entity, &mut AnimMan<CoinSmolAnim>, &mut Dyno)>,
    mut commands: Commands,
) {
    for (eid, mut anim, mut dyno) in &mut smol_coins {
        commands.entity(eid).remove::<ChaseEntity>();
        if matches!(anim.get_state(), CoinSmolAnim::Follow) {
            dyno.vel *= 0.2;
            anim.set_state(CoinSmolAnim::Pop);
        }
    }
}

#[derive(Bundle)]
struct BankBundle {
    name: Name,
    pos: Pos,
    spatial: SpatialBundle,
    trigger: TriggerTx,
    anim: AnimMan<BankAnim>,
}
impl MyLdtkEntity for BankBundle {
    type Root = WorldMetaRoot;

    fn from_ldtk(pos: Pos, _fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        Self {
            name: Name::new("bank"),
            pos,
            spatial: pos.to_spatial(ZIX_ITEMS - 0.32),
            trigger: TriggerTx::single(TriggerTxKind::Bank, HBox::new(24, 24)),
            anim: AnimMan::new(BankAnim::None),
        }
    }
}

/// Bank on the current level
fn manage_active_banks(
    mut smols: Query<(Entity, &CoinSmol, &mut AnimMan<CoinSmolAnim>, &mut Dyno)>,
    mut banks: Query<(&Pos, &TriggerTxCtrl, &mut AnimMan<BankAnim>)>,
    trigger_colls: Res<TriggerColls>,
    mut commands: Commands,
    mut bullet_time: ResMut<BulletTime>,
    mut camera_shake: ResMut<CameraShake>,
) {
    let mut cashed_out = false;
    for (pos, ttx_ctrl, mut anim) in &mut banks {
        // First do any necessary transitioning
        if anim.get_state() == BankAnim::None && !smols.is_empty() {
            anim.set_state(BankAnim::Grow);
            camera_shake.shake(0.5, -1..=1, -1..=1);
            commands.spawn((SoundEffect::BankTransition, OneSound::Ignore));
        }
        if anim.get_state() == BankAnim::Idle && smols.is_empty() {
            anim.set_state(BankAnim::Shrink);
            camera_shake.shake(0.36, -1..=1, -1..=1);
            commands.spawn((SoundEffect::BankTransition, OneSound::Ignore));
        }
        // Then maybe cash out
        if cashed_out {
            // Can only cash out once per frame
            continue;
        }
        if anim.get_state() != BankAnim::Idle {
            // Have to be idle to cash out
            continue;
        }
        if !trigger_colls
            .get_refs(&ttx_ctrl.coll_keys)
            .iter()
            .any(|coll| coll.rx_kind == TriggerRxKind::Player)
        {
            // Need to be colliding with player
            continue;
        }
        for (eid, coin, mut anim_smol, mut dyno) in &mut smols {
            if anim_smol.get_state() == CoinSmolAnim::Follow {
                cashed_out = true;
                dyno.vel *= 0.2;
                anim_smol.set_state(CoinSmolAnim::Pop);
                commands.entity(eid).remove::<ChaseEntity>();
                commands.spawn(EphemeralAnim::new(
                    BankAnim::Celebrate,
                    false,
                    *pos,
                    ZIX_ITEMS - 0.00123,
                ));
                bullet_time.set_temp(BulletTimeSpeed::Slow, 0.16);
                commands.spawn(SoundEffect::CoinCashOut);
                println!("would cash out {:?}", coin.iid);
            }
        }
    }
}

/// All other banks
fn manage_inactive_banks(mut banks: Query<&mut AnimMan<BankAnim>, With<SpawnedLidInactive>>) {
    for mut bank in &mut banks {
        if !matches!(bank.get_state(), BankAnim::None | BankAnim::Shrink) {
            bank.set_state(BankAnim::Shrink);
        }
    }
}

pub(super) fn register_coin(app: &mut App) {
    app.add_plugins(MyLdtkEntityPlugin::<CoinBundle>::new("Entities", "Coin"));
    app.add_plugins(MyLdtkEntityPlugin::<BankBundle>::new("Entities", "Bank"));

    app.observe(reset_coins_per_level);

    app.add_systems(
        Update,
        (
            maybe_collect_coins,
            manage_active_banks,
            manage_inactive_banks,
        )
            .chain()
            .after(PlayerSet)
            .after(PhysicsSet)
            .run_if(in_state(MetaStateKind::World)),
    );

    app.add_systems(OnEnter(PlayerMetaState::Dying), coin_smol_death);
}
