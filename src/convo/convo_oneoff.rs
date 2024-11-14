use bevy::text::Text2dBounds;

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
}

#[derive(Clone)]
struct ConvoOneoffText {
    anchor_eid: Entity,
    offset: Vec2,
    content: String,
}
impl ConvoOneoffText {
    fn induced_bg_offset(&self, size: &ConvoOneoffSize) -> Vec2 {
        (self.offset + size.bg_offset()) * TextLayer::growth_factor() as f32
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
            world.commands().entity(eid).insert((
                Name::new("convo_oneoff_parent"),
                SpatialBundle {
                    visibility: Visibility::Hidden,
                    ..default()
                },
            ));
            world
                .commands()
                .spawn((
                    Name::new("bg"),
                    SpriteBundle {
                        texture: bg_hand,
                        transform: Transform {
                            translation: text.induced_bg_offset(&size).extend(ZIX_CONVO_BG),
                            scale: (Vec2::ONE * TextLayer::growth_factor() as f32).extend(1.0),
                            ..default()
                        },
                        ..default()
                    },
                    TextLayer::to_render_layers(),
                ))
                .set_parent(eid);
            world
                .commands()
                .spawn((
                    Name::new("text"),
                    Text2dBundle {
                        text: Text::from_section(
                            text.content.clone(),
                            TextStyle {
                                font_size: 36.0,
                                ..default()
                            },
                        )
                        .with_justify(JustifyText::Center),
                        transform: Transform::from_translation(
                            (Vec2::new(18.0, 30.0) * TextLayer::growth_factor() as f32)
                                .extend(ZIX_CONVO_TEXT),
                        ),
                        text_2d_bounds: Text2dBounds {
                            size: Vec2::new(38.0, 28.0) * TextLayer::growth_factor() as f32,
                        },
                        text_anchor: bevy::sprite::Anchor::CenterLeft,
                        ..default()
                    },
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
