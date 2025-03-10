use crate::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum ConvoOneoffSize {
    Medium,
}
impl ConvoOneoffSize {
    fn bg_path(&self) -> &'static str {
        match self {
            Self::Medium => "convo/oneoff/bg_medium.png",
        }
    }

    fn bg_offset(&self) -> Vec2 {
        match self {
            Self::Medium => Vec2::new(30.0, 25.0),
        }
    }

    fn text_offset(&self) -> Vec2 {
        match self {
            Self::Medium => Vec2::new(16.0, 30.0),
        }
    }
}

#[derive(Clone)]
pub struct ConvoOneoffText {
    anchor_eid: Entity,
    offset: Vec2,
    content: String,
}
impl ConvoOneoffText {
    fn induced_bg_offset(&self, size: &ConvoOneoffSize) -> Vec2 {
        (self.offset + size.bg_offset()) * TextLayer::growth_factor() as f32
    }
    fn induced_text_offset(&self, size: &ConvoOneoffSize) -> Vec2 {
        (self.offset + size.text_offset()) * TextLayer::growth_factor() as f32
    }
}
impl Component for ConvoOneoffText {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let size = world
                .get::<ConvoOneoffSize>(eid)
                .expect("should have size")
                .clone();
            let text = world.get::<Self>(eid).expect("myself").clone();
            let bg_hand = world.resource::<AssetServer>().load(size.bg_path());
            let oneoff_parent = world.resource::<WorldDetailRoot>().eid();
            world
                .commands()
                .entity(eid)
                .insert((
                    Name::new("convo_oneoff_parent"),
                    Transform::default(),
                    Visibility::Hidden,
                ))
                .set_parent(oneoff_parent);
            world
                .commands()
                .spawn((
                    Name::new("bg"),
                    Sprite::from_image(bg_hand),
                    Transform {
                        translation: text.induced_bg_offset(&size).extend(ZIX_CONVO_BG),
                        scale: (Vec2::ONE * TextLayer::growth_factor() as f32).extend(1.0),
                        ..default()
                    },
                    Visibility::Inherited,
                    TextLayer::to_render_layers(),
                ))
                .set_parent(eid);
            let font_hand = world
                .resource::<AssetServer>()
                .load("fonts/KodeMono/KodeMono-Bold.ttf");
            world
                .commands()
                .spawn((
                    Name::new("text"),
                    Text2d::new(text.content.clone()),
                    TextFont::from_font(font_hand).with_font_size(42.0),
                    TextLayout::new_with_justify(JustifyText::Center),
                    TextBounds::new(
                        38.0 * TextLayer::growth_factor() as f32,
                        280.0 * TextLayer::growth_factor() as f32,
                    ),
                    Anchor::CenterLeft,
                    Transform::from_translation(
                        text.induced_text_offset(&size).extend(ZIX_CONVO_TEXT),
                    ),
                    Visibility::Inherited,
                    TextLayer::to_render_layers(),
                ))
                .set_parent(eid);
        });
    }
}

fn position_oneoff_texts(
    pos_q: Query<&Pos>,
    mut oneoffs: Query<(Entity, &mut Transform, &ConvoOneoffText, &mut Visibility)>,
    camera_pos: Query<&Pos, With<DynamicCamera>>,
    mut commands: Commands,
) {
    let camera_pos = camera_pos.single().as_vec2();
    for (eid, mut tran, text, mut viz) in &mut oneoffs {
        match pos_q.get(text.anchor_eid) {
            Ok(pos) => {
                let text_layer_pos = (pos.as_vec2() - camera_pos).round();
                tran.translation.x = text_layer_pos.x * TextLayer::growth_factor() as f32;
                tran.translation.y = text_layer_pos.y * TextLayer::growth_factor() as f32;
                *viz = Visibility::Visible;
            }
            _ => {
                commands.entity(eid).despawn_recursive();
            }
        }
    }
}

#[derive(Bundle)]
pub struct ConvoOneoff {
    size: ConvoOneoffSize,
    text: ConvoOneoffText,
}
impl ConvoOneoff {
    pub fn medium(anchor: Entity, offset: Vec2, content: &str) -> Self {
        Self {
            size: ConvoOneoffSize::Medium,
            text: ConvoOneoffText {
                anchor_eid: anchor,
                offset,
                content: content.to_string(),
            },
        }
    }
}

pub(super) fn register_convo_oneoff(app: &mut App) {
    app.add_systems(
        Update,
        position_oneoff_texts.after(PhysicsSet).after(CameraSet),
    );
}
