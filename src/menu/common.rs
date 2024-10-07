use crate::prelude::*;

#[derive(Component, Clone, Debug, Reflect)]
pub(super) struct MenuTemp;

#[derive(Clone, Debug, Reflect)]
pub(super) struct MenuImage {
    path: &'static str,
    zix: f32,
}
impl MenuImage {
    pub fn new(path: &'static str) -> Self {
        Self { path, zix: 0.0 }
    }
    impl_with!(zix, f32);
}
impl Component for MenuImage {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let myself = world.get::<Self>(eid).unwrap();
            let ass = world.resource::<AssetServer>();
            let hand = ass.load(myself.path);
            let sprite_bund = SpriteBundle {
                texture: hand,
                transform: Transform::from_translation(Vec3::Z * myself.zix),
                ..default()
            };
            world.commands().entity(eid).insert(sprite_bund);
            world
                .commands()
                .entity(eid)
                .insert(MenuLayer::to_render_layers());
            world.commands().entity(eid).insert(MenuTemp);
        });
    }
}

#[derive(Event)]
pub(super) struct CleanupMenuTemp;

pub(super) fn cleanup_menu_temp(
    _: Trigger<CleanupMenuTemp>,
    mut commands: Commands,
    temp: Query<Entity, With<MenuTemp>>,
) {
    for eid in &temp {
        commands.entity(eid).despawn_recursive();
    }
}
