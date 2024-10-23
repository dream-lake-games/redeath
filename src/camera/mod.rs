use crate::prelude::*;

mod camera_movement;
pub mod camera_shake;

pub use camera_movement::camera_clamp_logic;
pub use camera_shake::*;

/// The set that contains all camera related systems
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CameraSet;

#[derive(Component, Clone, Debug, Reflect)]
pub struct DynamicCamera;

/// This is for the layer cameras that need to "follow" the dynamic camera. Don't get confused
#[derive(Component, Clone, Debug, Reflect)]
pub struct FollowDynamicCamera;

#[derive(Resource, Clone, Copy, Debug, Reflect, Default)]
pub enum DynamicCameraMode {
    /// Follow an entity
    Follow(Entity),
    /// Catch-all for now, don't overthink the API until you need to
    #[default]
    Hanging,
}

#[derive(Bundle)]
struct DynamicCameraBundle {
    name: Name,
    pos: Pos,
    marker: DynamicCamera,
}

fn spawn_dynamic_camera(mut commands: Commands, root: Res<LayerRoot>) {
    commands
        .spawn(DynamicCameraBundle {
            name: Name::new("dynamic_camera"),
            pos: Pos::new(0.0, 0.0),
            marker: DynamicCamera,
        })
        .set_parent(root.eid());
}

pub(super) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        reg_types!(app, DynamicCamera, DynamicCameraMode);

        app.insert_resource(DynamicCameraMode::Hanging);
        app.add_systems(Startup, spawn_dynamic_camera);

        camera_movement::register_camera_movement(app);
        camera_shake::register_camera_shake(app);
    }
}
