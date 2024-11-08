use crate::prelude::*;

#[derive(Clone, Debug, Reflect)]
pub struct ConvoText {
    pub text: String,
}
impl ConvoText {
    pub fn simple(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

impl Component for ConvoText {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            world
                .commands()
                .spawn((
                    Name::new("text"),
                    Text2dBundle {
                        text: Text::from_section("Hello world hello hello hello hello", default()),
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, ZIX_CONVO_TEXT)),
                        ..default()
                    },
                ))
                .set_parent(eid);
        });
    }
}
