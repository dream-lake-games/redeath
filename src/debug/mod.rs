use final_post_processing::FinalPostProcessingMat;

use crate::prelude::*;

mod draw_hitboxes;
mod reload;
mod view_savefiles;

fn debug_startup(mut gizmo_config_store: ResMut<GizmoConfigStore>, mut _commands: Commands) {
    // Gizmo config
    let (config, _) = gizmo_config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 2.0;
    config.render_layers = StaticLayer::to_render_layers();
}

fn _spawn_light(pos: Pos, commands: &mut Commands) {
    commands.spawn((
        Name::new("debug_light"),
        pos,
        pos.to_spatial(0.0 + thread_rng().gen_range(0.0..1.0)),
        Light::new(PlayerLightAnim::Static128),
    ));
}

fn toggle_crt(
    hands: Query<&Handle<FinalPostProcessingMat>>,
    mut mat: ResMut<Assets<FinalPostProcessingMat>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyC) {
        for hand in &hands {
            let mat = mat.get_mut(hand.id()).unwrap();
            if mat.enabled > 0.0 {
                mat.enabled = 0.0;
            } else {
                mat.enabled = 1.0;
            }
        }
    }
}

fn debug_update() {}

pub(super) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, debug_startup.after(RootInit));
        app.add_systems(Update, debug_update);
        app.add_systems(Update, toggle_crt);
        draw_hitboxes::register_draw_hitboxes(app);
        reload::register_reload(app);
        view_savefiles::register_viewsavefiles(app);
    }
}
