use crate::prelude::*;

#[derive(Bundle)]
struct BackgroundDirtBundle {
    name: Name,
}
impl MyLdtkIntCell for BackgroundDirtBundle {
    type Root = WorldDetailRoot;
    type RenderLayers = MainAmbienceLayer;
    fn from_ldtk(_pos: Pos, _value: i32) -> Self {
        Self {
            name: Name::new("background_dirt"),
        }
    }
}

#[derive(Bundle)]
struct DirtBundle {
    name: Name,
    pos: Pos,
    static_tx: StaticTx,
}
impl MyLdtkIntCell for DirtBundle {
    type Root = PlatformRoot;
    type RenderLayers = StaticLayer;
    fn from_ldtk(pos: Pos, _value: i32) -> Self {
        Self {
            name: Name::new("dirt"),
            pos,
            static_tx: StaticTx::single(StaticTxKind::Solid, HBox::new(8, 8)),
        }
    }
}

pub(super) fn register_dirt(app: &mut App) {
    app.add_plugins(MyLdtkIntCellPlugin::<BackgroundDirtBundle>::new(
        "LakeAmbience",
        1,
    ));
    app.add_plugins(MyLdtkIntCellPlugin::<DirtBundle>::new("LakePlatforms", 1));
}
