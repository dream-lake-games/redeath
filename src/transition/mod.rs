use crate::prelude::*;

fn startup(mut commands: Commands, root: Res<TransitionRoot>) {
    commands
        .spawn((
            Name::new("TransitionAnim"),
            AnimMan::<TransitionAnim>::default(),
            SpatialBundle::default(),
        ))
        .set_parent(root.eid());
}

pub struct ShutupRust;

pub(super) struct TransitionPlugin;
impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup.after(RootInit));
    }
}
