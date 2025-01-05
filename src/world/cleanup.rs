use crate::prelude::*;

/// This despawns everything in the world, and then triggers an ldtk unload
#[derive(Event)]
pub struct CleanupWorld;

fn handle_cleanup_world(
    _trigger: Trigger<CleanupWorld>,
    mut commands: Commands,
    root: Res<WorldRoot>,
    children: Query<&Children>,
) {
    if let Ok(chiln) = children.get(root.eid()) {
        for child in chiln {
            commands.entity(*child).despawn_descendants();
        }
    }
    commands.trigger(UnloadMyLdtk);
}

pub(super) fn register_cleanup(app: &mut App) {
    app.add_observer(handle_cleanup_world);
}
