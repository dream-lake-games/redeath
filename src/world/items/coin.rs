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
    transform: Transform,
    visibility: Visibility,
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
            transform: pos.to_transform(ZIX_ITEMS + 0.21),
            visibility: Visibility::Inherited,
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
    transform: Transform,
    visibility: Visibility,
    anim: AnimMan<CoinSmolAnim>,
}
impl CoinSmolBundle {
    fn new(pos: Pos, iid: String, empty: bool) -> Self {
        Self {
            name: Name::new("coin_smol"),
            marker: CoinSmol { iid },
            pos,
            dyno: default(),
            transform: pos.to_transform(ZIX_ITEMS + 1.2),
            visibility: Visibility::Inherited,
            anim: AnimMan::new(if empty {
                CoinSmolAnim::FollowEmpty
            } else {
                CoinSmolAnim::Follow
            }),
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
    root: Res<WorldMetaRoot>,
) {
    for (eid, pos, ttx_ctrl, coin, mut anim, mut light) in &mut coins {
        if !matches!(anim.get_state(), CoinAnim::Spin | CoinAnim::SpinEmpty) {
            continue;
        }
        if trigger_colls
            .get_refs(&ttx_ctrl.coll_keys)
            .iter()
            .any(|coll| coll.rx_kind == TriggerRxKind::Player)
        {
            let empty = anim.get_state() == CoinAnim::SpinEmpty;
            anim.set_state(if empty {
                CoinAnim::PopEmpty
            } else {
                CoinAnim::Pop
            });
            light.set_state(CoinLightAnim::Pop);
            commands.spawn(SoundEffect::CoinCollect);
            commands.entity(eid).remove::<TriggerTxCtrl>();
            commands
                .spawn(CoinSmolBundle::new(pos.clone(), coin.iid.clone(), empty))
                .set_parent(root.eid());
        }
    }
}

fn position_smol_coins(
    player_q: Query<&Pos, (With<Player>, Without<CoinSmol>)>,
    bank_q: Query<
        &Pos,
        (
            With<AnimMan<BankAnim>>,
            With<SpawnedLidActive>,
            Without<CoinSmol>,
        ),
    >,
    mut smol_q: Query<&mut Pos, With<CoinSmol>>,
) {
    let (Ok(player_pos), Ok(bank_pos)) = (player_q.get_single(), bank_q.get_single()) else {
        return;
    };
    let rad = 12.0;
    let target = bank_pos.as_vec2() + Vec2::new(0.0, 4.0);
    let setting = if player_pos.as_vec2().distance(target) < rad {
        target
    } else {
        let dirred =
            player_pos.as_vec2() + (target - player_pos.as_vec2()).normalize_or_zero() * rad;
        dirred
    };
    for mut pos in &mut smol_q {
        pos.x = setting.x;
        pos.y = setting.y;
    }
}

fn reset_coins_per_level(
    _trigger: Trigger<EnterOrRespawnLevelEvent>,
    mut coins: Query<
        (
            Entity,
            &Coin,
            &mut AnimMan<CoinAnim>,
            &mut AnimMan<CoinLightAnim>,
            Option<&TriggerTxCtrl>,
        ),
        With<SpawnedLidActive>,
    >,
    mut smols: Query<&mut AnimMan<CoinSmolAnim>>,
    mut commands: Commands,
    current_collected: Res<SavefileCurrentCollectedCoins>,
) {
    for (eid, coin, mut anim, mut light, ttx_ctrl) in &mut coins {
        let empty = current_collected.0.contains(&coin.iid);
        if ttx_ctrl.is_none() {
            commands.entity(eid).insert(CoinBundle::trigger_tx());
        }
        anim.set_state(if empty {
            CoinAnim::SpinEmpty
        } else {
            CoinAnim::Spin
        });
        light.set_state(CoinLightAnim::Pulse);
    }
    for mut smol_anim in &mut smols {
        smol_anim.set_state(CoinSmolAnim::Pop);
    }
}

fn coin_smol_death(
    mut smol_coins: Query<(Entity, &mut AnimMan<CoinSmolAnim>, &mut Dyno)>,
    mut commands: Commands,
) {
    for (eid, mut anim, mut dyno) in &mut smol_coins {
        commands.entity(eid).remove::<ChaseEntity>();
        if matches!(
            anim.get_state(),
            CoinSmolAnim::Follow | CoinSmolAnim::FollowEmpty
        ) {
            dyno.vel *= 0.2;
            anim.set_state(CoinSmolAnim::Pop);
        }
    }
}

#[derive(Bundle)]
struct BankBundle {
    name: Name,
    pos: Pos,
    transform: Transform,
    visibility: Visibility,
    trigger: TriggerTx,
    anim: AnimMan<BankAnim>,
}
impl MyLdtkEntity for BankBundle {
    type Root = WorldMetaRoot;

    fn from_ldtk(pos: Pos, _fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        Self {
            name: Name::new("bank"),
            pos,
            transform: pos.to_transform(ZIX_ITEMS - 0.32),
            visibility: Visibility::Inherited,
            trigger: TriggerTx::single(TriggerTxKind::Bank, HBox::new(24, 24)),
            anim: AnimMan::new(BankAnim::Premonition),
        }
    }
}

/// Bank on the current level
fn manage_active_banks(
    mut smols: Query<(Entity, &CoinSmol, &mut AnimMan<CoinSmolAnim>, &mut Dyno)>,
    bigs: Query<&AnimMan<CoinAnim>, (With<Coin>, With<SpawnedLidActive>)>,
    mut banks: Query<(&Pos, &TriggerTxCtrl, &mut AnimMan<BankAnim>), With<SpawnedLidActive>>,
    trigger_colls: Res<TriggerColls>,
    mut commands: Commands,
    mut bullet_time: ResMut<BulletTime>,
    mut camera_shake: ResMut<CameraShake>,
    world_detail_root: Res<WorldDetailRoot>,
) {
    let mut cashed_out = false;
    for (pos, ttx_ctrl, mut anim) in &mut banks {
        // First do any necessary transitioning
        if matches!(anim.get_state(), BankAnim::None | BankAnim::Premonition)
            && smols.iter().any(|smol| {
                matches!(
                    smol.2.get_state(),
                    CoinSmolAnim::Follow | CoinSmolAnim::FollowEmpty
                )
            })
        {
            anim.set_state(BankAnim::Grow);
            camera_shake.shake(0.5, -1..=1, -1..=1);
            commands.spawn((SoundEffect::BankTransition, OneSound::Ignore));
        }
        if anim.get_state() == BankAnim::Idle && smols.is_empty() {
            anim.set_state(BankAnim::Shrink);
            camera_shake.shake(0.36, -1..=1, -1..=1);
            commands.spawn((SoundEffect::BankTransition, OneSound::Ignore));
        }
        if anim.get_state() == BankAnim::None
            && bigs.iter().any(|big| big.get_state() != CoinAnim::None)
        {
            anim.set_state(BankAnim::Premonition);
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
            if matches!(
                anim_smol.get_state(),
                CoinSmolAnim::Follow | CoinSmolAnim::FollowEmpty
            ) {
                cashed_out = true;
                dyno.vel *= 0.2;
                anim_smol.set_state(CoinSmolAnim::Pop);
                commands.entity(eid).remove::<ChaseEntity>();
                commands
                    .spawn(EphemeralAnim::new(
                        BankAnim::Celebrate,
                        false,
                        *pos,
                        ZIX_ITEMS - 0.00123,
                    ))
                    .set_parent(world_detail_root.eid());
                bullet_time.set_temp(BulletTimeSpeed::Slow, 0.16);
                commands.spawn(SoundEffect::CoinCashOut);
                commands.trigger(SavefileCollectCoinEvent {
                    iid: coin.iid.clone(),
                });
            }
        }
    }
}

/// All other banks
fn manage_inactive_banks(mut banks: Query<&mut AnimMan<BankAnim>, With<SpawnedLidInactive>>) {
    for mut bank in &mut banks {
        if !matches!(
            bank.get_state(),
            BankAnim::None | BankAnim::Premonition | BankAnim::Shrink
        ) {
            bank.set_state(BankAnim::Shrink);
        }
    }
}

pub(super) fn register_coin(app: &mut App) {
    app.add_plugins(MyLdtkEntityPlugin::<CoinBundle>::new("Entities", "Coin"));
    app.add_plugins(MyLdtkEntityPlugin::<BankBundle>::new("Entities", "Bank"));

    app.add_observer(reset_coins_per_level);

    app.add_systems(
        Update,
        (
            maybe_collect_coins,
            position_smol_coins,
            manage_active_banks,
            manage_inactive_banks,
        )
            .chain()
            .after(PlayerSet)
            .after(PhysicsSet)
            .run_if(in_state(MetaStateKind::World))
            .run_if(in_state(PauseState::Unpaused)),
    );

    app.add_systems(OnEnter(PlayerMetaState::Dying), coin_smol_death);
}
