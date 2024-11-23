use crate::prelude::*;

#[derive(Bundle)]
struct PassUpBundle {
    name: Name,
    static_tx: StaticTx,
    pos: Pos,
}
impl PassUpBundle {
    fn new(pos: Pos) -> Self {
        let hbox = HBox::new(8, 5).with_offset(0.0, 1.5);
        Self {
            name: Name::new("pass_up"),
            static_tx: StaticTx::single(StaticTxKind::PassUp, hbox),
            pos,
        }
    }
}
impl MyLdtkIntCell for PassUpBundle {
    type Root = PlatformRoot;
    type RenderLayers = StaticLayer;
    type LeftoverRenderLayers = DummyLayer;
    fn from_ldtk(pos: Pos, _value: i32) -> Self {
        Self::new(pos)
    }
}

pub(super) fn register_pass_up(app: &mut App) {
    app.add_plugins(MyLdtkIntCellPlugin::<PassUpBundle>::single(
        "CommonPlatforms",
        3,
    ));
}
