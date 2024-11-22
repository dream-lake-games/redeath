use crate::prelude::*;

struct SkellyDash;
impl Component for SkellyDash {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            world.commands().spawn(ConvoOneoff::medium(
                eid,
                Vec2::new(-5.0, -3.0),
                "Press K to dash in ANY cardinal direction",
            ));
        });
    }
}

#[derive(Bundle)]
struct SkellyDashBundle {
    marker: SkellyDash,
    name: Name,
    pos: Pos,
    spatial: SpatialBundle,
    anim: AnimMan<SkellyDashAnim>,
}
impl MyLdtkEntity for SkellyDashBundle {
    type Root = WorldDetailRoot;
    fn from_ldtk(pos: Pos, _fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        Self {
            marker: SkellyDash,
            name: Name::new("skelly_dash"),
            pos,
            spatial: pos.to_spatial(ZIX_ITEMS + 0.03),
            anim: default(),
        }
    }
}

pub(super) fn register_skelly_dash(app: &mut App) {
    app.add_plugins(MyLdtkEntityPlugin::<SkellyDashBundle>::new(
        "Entities",
        "SkellyDash",
    ));
}
