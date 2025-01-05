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
            let sprite_bund = (
                Sprite::from_image(hand),
                Transform::from_translation(Vec3::Z * myself.zix),
                Visibility::Inherited,
            );
            // TODO: We can't do this because this will get triggered from entering the menu state
            //       BEFORE the startup system setting the roots goes so this will still be placeholder
            // let parent = world.resource::<MenuRoot>().eid();
            world.commands().entity(eid).insert(sprite_bund);
            world
                .commands()
                .entity(eid)
                .insert(MenuLayer::to_render_layers());
            world
                .commands()
                .entity(eid)
                .insert((Name::new("menu_temp_image_fix_this"), MenuTemp));
        });
    }
}

#[derive(Event)]
pub(super) struct CleanupMenuTemp;

pub(super) fn cleanup_menu_temp(
    _: Trigger<CleanupMenuTemp>,
    mut commands: Commands,
    temp: Query<Entity, With<MenuTemp>>,
    autos: Query<Entity, With<AutoTransition>>,
) {
    for eid in temp.iter().chain(autos.iter()) {
        commands.entity(eid).despawn_recursive();
    }
}

#[derive(Component)]
pub struct AutoTransition(pub f32, pub MetaState);

pub fn watch_auto_transitions(
    mut commands: Commands,
    mut auto_transitions: Query<(Entity, &mut AutoTransition)>,
    time: Res<Time>,
) {
    let mut autoed = false;
    for (_, mut auto) in &mut auto_transitions {
        auto.0 -= time.delta_secs();
        if auto.0 < 0.0 {
            commands.trigger(StartTransition::to(auto.1.clone()));
            autoed = true;
        }
    }
    if autoed {
        for (eid, _) in &auto_transitions {
            commands.entity(eid).despawn_recursive();
        }
    }
}
