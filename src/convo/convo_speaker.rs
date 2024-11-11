use crate::prelude::*;

#[derive(Clone, Copy, Debug, Reflect, PartialEq)]
pub enum ConvoSpeaker {
    Silence(f32),
    Lenny,
    Friend,
}

#[derive(Component, Clone, Copy, Debug, Reflect, PartialEq, Eq)]
pub enum ConvoEmotion {
    Default,
}

impl Component for ConvoSpeaker {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let speaker = world.get::<ConvoSpeaker>(eid).expect("myself").clone();
            let emotion = world.get::<ConvoEmotion>(eid).expect("myself").clone();

            macro_rules! static_portrait {
                ($world:expr, $path:literal) => {
                    let hand = world.resource::<AssetServer>().load($path);
                    $world
                        .commands()
                        .spawn((
                            Name::new("portrait"),
                            SpriteBundle {
                                texture: hand,
                                transform: Transform {
                                    translation: Vec3::new(-444.0, 0.0, ZIX_CONVO_PORTRAIT),
                                    scale: (Vec2::ONE * TextLayer::growth_factor() as f32)
                                        .extend(1.0),
                                    ..default()
                                },
                                ..default()
                            },
                            TextLayer::to_render_layers(),
                        ))
                        .set_parent(eid);
                };
            }

            match (speaker, emotion) {
                (ConvoSpeaker::Silence(_), _) => {}
                (ConvoSpeaker::Lenny, _) => {
                    static_portrait!(world, "convo/lenny/default.png");
                }
                (ConvoSpeaker::Friend, _) => {
                    static_portrait!(world, "convo/friend/default.png");
                }
            }
        });
    }
}
