use super::playerlude::*;
use crate::prelude::*;

#[derive(Bundle)]
pub(super) struct PlayerBundle {
    name: Name,
    player: Player,
    pos: Pos,
    spawned_lid: SpawnedLid,
    physical_lid: PhysicalLid,
    transform: Transform,
    visibility: Visibility,
    dyno: Dyno,
    gravity: Gravity,
    static_rx: StaticRx,
    trigger_rx: TriggerRx,
    trigger_tx: TriggerTx,
    anim: AnimMan<PlayerAnim>,
    light: Light<PlayerLightAnim>,
}
impl PlayerBundle {
    pub(super) fn new(pos: Pos, lid: String) -> Self {
        let size = UVec2::new(8, 11);
        let offset = Vec2::new(0.0, -1.5);
        let fsize = size.as_vec2();

        let main_hbox = HBox::new(size.x, size.y)
            .with_offset(offset.x, offset.y)
            .with_marker(PLAYER_MAIN_HBOX);
        let right_hbox = HBox::new(1, size.y)
            .with_offset(offset.x + fsize.x / 2.0, offset.y)
            .with_marker(PLAYER_RIGHT_HBOX);
        let above_hbox = HBox::new(size.x, 1)
            .with_offset(offset.x, offset.y + fsize.y / 2.0)
            .with_marker(PLAYER_ABOVE_HBOX);
        let left_hbox = HBox::new(1, size.y)
            .with_offset(offset.x - fsize.x / 2.0, offset.y)
            .with_marker(PLAYER_LEFT_HBOX);
        let below_hbox = HBox::new(size.x, 1)
            .with_offset(offset.x, offset.y - fsize.y / 2.0)
            .with_marker(PLAYER_BELOW_HBOX);

        Self {
            name: Name::new("player"),
            player: Player,
            pos,
            spawned_lid: SpawnedLid::new(lid.clone()),
            physical_lid: PhysicalLid::new(lid.clone()),
            transform: pos.to_transform(ZIX_PLAYER),
            visibility: Visibility::Inherited,
            dyno: default(),
            gravity: default(),
            static_rx: StaticRx::new(vec![
                (StaticRxKind::Default, main_hbox.clone()),
                (StaticRxKind::Observe, right_hbox),
                (StaticRxKind::Observe, above_hbox),
                (StaticRxKind::Observe, left_hbox),
                (StaticRxKind::Observe, below_hbox),
            ]),
            trigger_rx: TriggerRx::single(TriggerRxKind::Player, main_hbox.clone()),
            trigger_tx: TriggerTx::single(TriggerTxKind::Player, main_hbox.clone()),
            anim: AnimMan::default()
                .with_observe_state_changes()
                .with_observe_ix_changes(),
            light: default(),
        }
    }
}
