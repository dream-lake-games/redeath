use crate::prelude::*;

#[derive(Bundle)]
struct SpikesBundle {
    name: Name,
    pos: Pos,
    trigger_tx: TriggerTx,
}
impl MyLdtkIntCell for SpikesBundle {
    type Root = PlatformRoot;
    type RenderLayers = StaticLayer;
    type LeftoverRenderLayers = MainAmbienceLayer;
    fn from_ldtk(pos: Pos, _value: i32) -> Self {
        Self {
            name: Name::new("spikes"),
            pos,
            trigger_tx: TriggerTx::single(TriggerTxKind::Spikes, HBox::new(6, 6)),
        }
    }
}

pub(super) fn register_spikes(app: &mut App) {
    app.add_plugins(MyLdtkIntCellPlugin::<SpikesBundle>::single("Spikes", 1));
}
