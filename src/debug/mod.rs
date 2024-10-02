use crate::prelude::*;

fn debug_startup(mut commands: Commands, ass: Res<AssetServer>) {
    commands.spawn((
        Name::new("scene3"),
        SpriteBundle {
            texture: ass.load("play/scene3.png"),
            ..default()
        },
        MainLayer::to_render_layers(),
    ));
}

fn debug_update() {}

pub(super) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, debug_startup);
        app.add_systems(Update, debug_update);
    }
}
