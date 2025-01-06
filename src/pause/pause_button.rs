use crate::prelude::*;

#[derive(Resource, Default)]
pub struct PauseButtonInput {
    pub block_move: Option<(f32, f32)>,
}

pub struct PauseText(pub String);

#[derive(Bundle)]
pub struct PauseButtonBundle {
    name: Name,
    text: PauseText,
    anim: AnimMan<PauseButtonAnim>,
    transform: Transform,
    visibility: Visibility,
}
impl PauseButtonBundle {
    pub fn new(text: &str, y: f32) -> Self {
        Self {
            name: Name::new(format!("pause_button_{:?}", text)),
            text: PauseText(text.to_string()),
            anim: default(),
            transform: Pos::new(0.0, y).to_transform(1.0),
            visibility: Visibility::Inherited,
        }
    }

    pub fn selected(mut self) -> Self {
        self.anim = AnimMan::new(PauseButtonAnim::Active);
        self
    }
}

impl Component for PauseText {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let parent = world.resource::<PauseRoot>().eid();
            let content = world.get::<Self>(eid).unwrap().0.clone();
            world.commands().entity(eid).set_parent(parent);
            let font_hand = world
                .resource::<AssetServer>()
                .load("fonts/KodeMono/KodeMono-Bold.ttf");
            world
                .commands()
                .spawn((
                    Name::new("bause_button_text"),
                    Text2d::new(content),
                    TextFont::from_font(font_hand).with_font_size(24.0),
                    Transform::from_translation(Vec3::Z),
                    Visibility::Inherited,
                    MenuLayer::to_render_layers(),
                ))
                .set_parent(eid);
        });
    }
}
