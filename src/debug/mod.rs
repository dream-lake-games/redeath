use crate::prelude::*;

mod draw_hitboxes;

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
        draw_hitboxes::register_draw_hitboxes(app);
    }
}
