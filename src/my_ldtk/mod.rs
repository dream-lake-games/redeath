use std::ops::Deref;

use crate::prelude::*;

pub mod my_ldtk_consolidate;
pub mod my_ldtk_entity;
pub mod my_ldtk_int_cell;
pub mod my_ldtk_level_maint;
pub mod my_ldtk_load;

pub use my_ldtk_consolidate::*;
pub use my_ldtk_entity::*;
pub use my_ldtk_int_cell::*;
pub use my_ldtk_level_maint::*;
pub use my_ldtk_load::*;

/// The set that contains all weird ldtk maintenence
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MyLdtkMaintSet;

/// What leveliid spawned this thing?
#[derive(Component, Clone, Debug, Reflect)]
pub struct SpawnedLid {
    pub iid: String,
}
impl SpawnedLid {
    pub fn new(iid: String) -> Self {
        Self { iid }
    }
}
/// Attached to entities when it is known that the level that spawned them IS active
#[derive(Component, Clone, Debug, Reflect)]
pub struct SpawnedLidActive;
/// Attached to entities when it is known that the level that spawned them IS NOT active
#[derive(Component, Clone, Debug, Reflect)]
pub struct SpawnedLidInactive;

/// What leveliid is this object physically in?
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct PhysicalLid {
    /// Is the entity currently in the last_known_iid?
    pub in_bounds: bool,
    /// What is the last known level iid this entity was in?
    /// NOTE: None on initialization
    pub last_known_iid: Option<String>,
}
impl PhysicalLid {
    pub fn new(iid: String) -> Self {
        Self {
            in_bounds: true,
            last_known_iid: Some(iid),
        }
    }
}
/// Attached to entities when it is known that the level they are phsyically in (pos) IS active
#[derive(Component, Clone, Debug, Reflect)]
pub struct PhysicalLidActive;
/// Attached to entities when it is known that they are in (pos) SOME level, but that that level IS NOT active
#[derive(Component, Clone, Debug, Reflect)]
pub struct PhysicalLidInactive;
/// Attached to entities when it is known that they ARE NOT physically in (pos) ANY level
#[derive(Component, Clone, Debug, Reflect)]
pub struct PhysicalLidOob;

/// Rect bounds of all levels. Grows with time (i.e. only ever includes levels that have been spawned)
/// Never shrinks because it probably is fine
#[derive(Resource, Clone, Debug, Default, Reflect)]
pub struct LevelRects {
    pub map: HashMap<String, Rect>,
    /// Just a convenience since most things just want this
    pub current: Option<Rect>,
}
impl LevelRects {
    pub fn get(&self, key: &str) -> Option<&Rect> {
        self.map.get(key)
    }
    pub fn set(&mut self, key: String, rect: Rect) {
        self.map.insert(key, rect);
    }
}
impl Deref for LevelRects {
    type Target = HashMap<String, Rect>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

/// A "fake" child relationship, so that when ldtk automatically deloads stuff it translates to despawning entities
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
            if let Some(comms) = world.commands().get_entity(my_child) {
                comms.despawn_recursive();
            }
        });
    }
}

/// A "fake" parent relationship, so that children can potentially manipulate their parent's tilemap
#[derive(Component, Reflect)]
pub struct MyLdtkParent {
    parent_eid: Entity,
}
impl MyLdtkParent {
    pub fn get(&self) -> Entity {
        self.parent_eid
    }
}

pub(super) struct MyLdtkPlugin;
impl Plugin for MyLdtkPlugin {
    fn build(&self, app: &mut App) {
        reg_types!(
            app,
            SpawnedLid,
            SpawnedLidActive,
            PhysicalLid,
            PhysicalLidActive,
            PhysicalLidOob,
            MyLdtkChild
        );

        my_ldtk_level_maint::register_my_ldtk_level_maint(app);
        my_ldtk_consolidate::register_my_ldtk_consolidate(app);

        app.add_plugins(LdtkPlugin).insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            ..default()
        });
        register_my_ldtk_load(app);
    }
}

pub trait UnfuckLevelSelection {
    fn to_iid(&self) -> String;
}
impl UnfuckLevelSelection for LevelSelection {
    fn to_iid(&self) -> String {
        let LevelSelection::Iid(iid) = self else {
            panic!("can't unfuck level selection");
        };
        iid.to_string()
    }
}
