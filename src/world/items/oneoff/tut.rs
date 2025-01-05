use crate::prelude::*;

struct SkellyDash;
impl Component for SkellyDash {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            world.commands().spawn(ConvoOneoff::medium(
                eid,
                Vec2::new(-5.0, -3.0),
                "Press K to dash",
            ));
        });
    }
}

#[derive(Bundle)]
struct SkellyDashBundle {
    marker: SkellyDash,
    name: Name,
    pos: Pos,
    transform: Transform,
    visibility: Visibility,
    anim: AnimMan<SkellyDashAnim>,
}
impl MyLdtkEntity for SkellyDashBundle {
    type Root = WorldDetailRoot;
    fn from_ldtk(pos: Pos, _fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        Self {
            marker: SkellyDash,
            name: Name::new("skelly_dash"),
            pos,
            transform: pos.to_transform(ZIX_ITEMS + 0.03),
            visibility: Visibility::Inherited,
            anim: default(),
        }
    }
}

#[derive(Bundle)]
struct DashDrawingBundle {
    name: Name,
    pos: Pos,
    transform: Transform,
    visibility: Visibility,
    anim: AnimMan<DashDrawingAnim>,
}
impl MyLdtkEntity for DashDrawingBundle {
    type Root = WorldDetailRoot;
    fn from_ldtk(pos: Pos, fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        let Some(FieldValue::String(Some(text))) = fields.get("Kind") else {
            panic!("ahh dash drawing");
        };
        let Some(FieldValue::Bool(flipx)) = fields.get("FlipX") else {
            panic!("ahh dash drawing2");
        };
        let anim = match text.as_str() {
            "Up" => AnimMan::new(DashDrawingAnim::Up),
            "Diagonal" => AnimMan::new(DashDrawingAnim::Diagonal),
            _ => AnimMan::new(DashDrawingAnim::Right),
        }
        .with_flip_x(*flipx);
        Self {
            name: Name::new("skelly_dash"),
            pos,
            transform: pos.to_transform(ZIX_ITEMS + 0.03),
            visibility: Visibility::Inherited,
            anim,
        }
    }
}

pub(super) fn register_tut(app: &mut App) {
    app.add_plugins(MyLdtkEntityPlugin::<SkellyDashBundle>::new(
        "Entities",
        "SkellyDash",
    ));
    app.add_plugins(MyLdtkEntityPlugin::<DashDrawingBundle>::new(
        "Entities",
        "DashDrawing",
    ));
}
