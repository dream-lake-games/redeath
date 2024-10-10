use crate::prelude::*;

mod draw_hitboxes;

#[derive(Bundle)]
struct TestEntity {
    name: Name,
    sprite: SpriteBundle,
    render_layers: RenderLayers,
    pos: Pos,
    plid: PhysicalLid,
}
impl MyLdtkEntity for TestEntity {
    type Root = WorldRoot;
    fn from_ldtk(pos: Pos, _fields: &HashMap<String, FieldValue>) -> Self {
        Self {
            name: Name::new("test_entity"),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(8.0, 8.0)),
                    ..default()
                },
                transform: pos.to_spatial(0.0).transform,
                ..default()
            },
            render_layers: MainLayer::to_render_layers(),
            pos,
            plid: default(),
        }
    }
}

fn debug_startup(mut gizmo_config_store: ResMut<GizmoConfigStore>) {
    // Gizmo config
    let (config, _) = gizmo_config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 2.0;
    config.render_layers = MainLayer::to_render_layers();
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
        draw_hitboxes::register_draw_hitboxes(app);
    }
}
