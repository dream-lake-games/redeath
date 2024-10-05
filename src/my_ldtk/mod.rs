use crate::prelude::*;

pub mod my_ldtk_entity;

pub use my_ldtk_entity::*;

#[derive(Reflect)]
struct MyLdtkChild {
    child_eid: Entity,
}
impl Component for MyLdtkChild {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_remove(|mut world, eid, _| {
            let my_child = world
                .get::<Self>(eid)
                .expect("myself should exist")
                .child_eid;
            world.commands().entity(my_child).despawn_recursive();
        });
    }
}

pub(super) struct MyLdtkPlugin;
impl Plugin for MyLdtkPlugin {
    fn build(&self, app: &mut App) {
        reg_types!(app, MyLdtkChild);

        app.add_plugins(LdtkPlugin)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..default()
            })
            .insert_resource(LevelSelection::iid("8502fb20-73f0-11ef-a7cc-51ce498506c2"));
    }
}
