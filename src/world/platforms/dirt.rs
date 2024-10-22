use crate::prelude::*;

#[derive(Bundle)]
struct BackgroundDirtBundle {
    name: Name,
}
impl MyLdtkIntCell for BackgroundDirtBundle {
    type Root = WorldDetailRoot;
    type RenderLayers = MainAmbienceLayer;
    type LeftoverRenderLayers = MainAmbienceLayer;
    fn from_ldtk(_pos: Pos, _value: i32) -> Self {
        Self {
            name: Name::new("background_dirt"),
        }
    }
}

#[derive(Bundle)]
struct DirtStaticBundle {
    name: Name,
    pos: Pos,
    static_tx: StaticTx,
    consolidate: MyLdtkConsolidateKind,
}
impl MyLdtkIntCell for DirtStaticBundle {
    type Root = PlatformRoot;
    type RenderLayers = StaticLayer;
    type LeftoverRenderLayers = MainAmbienceLayer;
    fn from_ldtk(pos: Pos, _value: i32) -> Self {
        Self {
            name: Name::new("dirt_static"),
            pos,
            static_tx: StaticTx::single(StaticTxKind::Solid, HBox::new(8, 8)),
            consolidate: MyLdtkConsolidateKind::Solid8x8,
        }
    }
}

pub(super) fn register_dirt(app: &mut App) {
    app.add_plugins(MyLdtkIntCellPlugin::<BackgroundDirtBundle>::single(
        "DirtAmbience",
        1,
    ));
    app.add_plugins(MyLdtkIntCellPlugin::<DirtStaticBundle>::single(
        "DirtStatic",
        1,
    ));
}
