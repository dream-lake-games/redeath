use bevy::audio::{PlaybackMode, Volume};

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Reflect, PartialEq)]
pub enum ConvoSpeaker {
    Silence(f32),
    Lenny,
    Friend,
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct ConvoPortrait {
    pub key: String,
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct ConvoSound {
    pub key: String,
}

impl Component for ConvoSpeaker {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let speaker = world.get::<ConvoSpeaker>(eid).expect("myself").clone();
            let portrait = world.get::<ConvoPortrait>(eid).expect("myself").clone();
            let sound = world.get::<ConvoSound>(eid).expect("myself").clone();

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

            match (speaker, portrait.key.as_str()) {
                (ConvoSpeaker::Silence(_), _) => {}
                (ConvoSpeaker::Lenny, _) => {
                    static_portrait!(world, "convo/lenny/portrait/default.png");
                }
                (ConvoSpeaker::Friend, _) => {
                    static_portrait!(world, "convo/friend/portrait/default.png");
                }
            }

            macro_rules! convo_sound {
                ($world:expr, $path:expr$(, $mult:literal$(,)?)?) => {
                    #[allow(unused_mut)]
                    let mut base_mult = 0.2;
                    $(
                        base_mult = $mult;
                    )?
                    let ass = world.resource::<AssetServer>();
                    let hand = ass.load($path);
                    $world
                        .commands()
                        .spawn(AudioBundle {
                            source: hand,
                            settings: PlaybackSettings {
                                mode: PlaybackMode::Despawn,
                                volume: Volume::new(base_mult),
                                ..default()
                            },
                        })
                        .set_parent(eid);
                };
            }

            match (speaker, sound.key.as_str()) {
                (ConvoSpeaker::Silence(_), _) => {}
                (ConvoSpeaker::Lenny, key) => {
                    convo_sound!(world, format!("convo/lenny/sound/{key}.ogg"));
                }
                (ConvoSpeaker::Friend, key) => {
                    convo_sound!(world, format!("convo/friend/sound/{key}.ogg"));
                }
            }
        });
    }
}
