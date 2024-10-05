use crate::prelude::*;

#[derive(Bundle)]
struct TestEntity {
    name: Name,
    sprite: SpriteBundle,
    render_layers: RenderLayers,
}
impl MyLdtkEntity for TestEntity {
    type Root = WorldRoot;
    fn from_ldtk(gt: &GlobalTransform, _fields: &HashMap<String, FieldValue>) -> Self {
        Self {
            name: Name::new("test_entity"),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(8.0, 8.0)),
                    ..default()
                },
                transform: gt.compute_transform(),
                ..default()
            },
            render_layers: MainLayer::to_render_layers(),
        }
    }
}

fn debug_startup(mut commands: Commands, ass: Res<AssetServer>) {
    // commands.spawn((
    //     Name::new("scene3"),
    //     SpriteBundle {
    //         texture: ass.load("play/scene3.png"),
    //         ..default()
    //     },
    //     MainLayer::to_render_layers(),
    // ));
    commands.spawn((
        Name::new("ldtk_world"),
        LdtkWorldBundle {
            ldtk_handle: ass.load("play/test.ldtk"),
            ..default()
        },
    ));
}

fn debug_update() {}

pub(super) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, debug_startup);
        app.add_systems(Update, debug_update);
        app.add_plugins(MyLdtkEntityPlugin::<TestEntity>::new(
            "Entities",
            "TestEntity",
        ));
    }
}
