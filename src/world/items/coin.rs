use crate::prelude::*;

#[derive(Bundle)]
struct CoinBundle {
    name: Name,
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
    fn from_ldtk(pos: Pos, _fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        Self {
            name: Name::new("coin"),
            pos,
            spatial: pos.to_spatial(ZIX_ITEMS + 0.21),
            trigger_tx: Self::trigger_tx(),
            anim: default(),
            light: default(),
            bob: Bob::vert(pos, 3.0, 1.3),
        }
    }
}

fn maybe_collect_coins(
    mut coins: Query<(
        Entity,
        &TriggerTxCtrl,
        &mut AnimMan<CoinAnim>,
        &mut AnimMan<CoinLightAnim>,
    )>,
    trigger_colls: Res<TriggerColls>,
    mut bullet_time: ResMut<BulletTime>,
    mut commands: Commands,
) {
    for (eid, ttx_ctrl, mut anim, mut light) in &mut coins {
        if trigger_colls
            .get_refs(&ttx_ctrl.coll_keys)
            .iter()
            .any(|coll| coll.rx_kind == TriggerRxKind::Player)
        {
            anim.set_state(CoinAnim::Pop);
            light.set_state(CoinLightAnim::Pop);
            commands.spawn(SoundEffect::CoinBell);
            bullet_time.set_temp(BulletTimeSpeed::Slow, 0.15);
            commands.entity(eid).remove::<TriggerTxCtrl>();
        }
    }
}

pub(super) fn register_coin(app: &mut App) {
    app.add_plugins(MyLdtkEntityPlugin::<CoinBundle>::new("Entities", "Coin"));

    app.add_systems(
        Update,
        maybe_collect_coins
            .chain()
            .after(PlayerSet)
            .run_if(in_state(MetaStateKind::World)),
    );
}
