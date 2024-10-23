use crate::prelude::*;

#[derive(Bundle)]
struct DirtAmbienceBundle {
    name: Name,
}
impl MyLdtkIntCell for DirtAmbienceBundle {
    type Root = WorldDetailRoot;
    type RenderLayers = MainAmbienceLayer;
    type LeftoverRenderLayers = MainAmbienceLayer;
    fn from_ldtk(_pos: Pos, _value: i32) -> Self {
        Self {
            name: Name::new("dirt_background"),
        }
    }
}

#[derive(Bundle)]
struct DirtDetailBundle {
    name: Name,
}
impl MyLdtkIntCell for DirtDetailBundle {
    type Root = WorldDetailRoot;
    type RenderLayers = MainDetailLayer;
    type LeftoverRenderLayers = MainAmbienceLayer;
    fn from_ldtk(_pos: Pos, _value: i32) -> Self {
        Self {
            name: Name::new("dirt_detail"),
        }
    }
}

#[derive(Bundle)]
struct DirtPlatformsBundle {
    name: Name,
    pos: Pos,
    static_tx: StaticTx,
    consolidate: MyLdtkConsolidateKind,
}
impl MyLdtkIntCell for DirtPlatformsBundle {
    type Root = PlatformRoot;
    type RenderLayers = StaticLayer;
    type LeftoverRenderLayers = MainAmbienceLayer;
    fn from_ldtk(pos: Pos, _value: i32) -> Self {
        Self {
            name: Name::new("dirt_platform"),
            pos,
            static_tx: StaticTx::single(StaticTxKind::Solid, HBox::new(8, 8)),
            consolidate: MyLdtkConsolidateKind::Solid8x8,
        }
    }
}

pub(super) fn register_dirt(app: &mut App) {
    app.add_plugins(MyLdtkIntCellPlugin::<DirtAmbienceBundle>::single(
        "DirtAmbience",
        1,
    ));
    app.add_plugins(MyLdtkIntCellPlugin::<DirtDetailBundle>::single(
        "DirtDetail",
        1,
    ));
    app.add_plugins(MyLdtkIntCellPlugin::<DirtPlatformsBundle>::single(
        "DirtPlatforms",
        1,
    ));
}
