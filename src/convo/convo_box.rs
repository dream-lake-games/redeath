use crate::prelude::*;

#[derive(Bundle, Clone, Debug)]
pub struct ConvoBox {
    name: Name,
    root: ConvoBoxRoot,
    spatial: SpatialBundle,
    pub speaker: ConvoSpeaker,
    pub emotion: ConvoEmotion,
    pub text: ConvoText,
}
impl ConvoBox {
    pub fn new(speaker: ConvoSpeaker, emotion: ConvoEmotion, text: ConvoText) -> Self {
        Self {
            name: Name::new("box_root"),
            root: ConvoBoxRoot,
            spatial: SpatialBundle {
                visibility: Visibility::Hidden,
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..default()
            },
            speaker,
            emotion,
            text,
        }
    }
}

#[derive(Clone, Debug, Reflect)]
struct ConvoBoxRoot;

struct ConvoBoxRootLoading {
    frames_left: u32,
}

impl Component for ConvoBoxRoot {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            // Set the right parent, insert the loading waiter, gift hidden spatial
            let convo_root_eid = world.resource::<ConvoRoot>().eid();
            let box_root_eid = world
                .commands()
                .entity(eid)
                .set_parent(convo_root_eid)
                .insert(ConvoBoxRootLoading { frames_left: 6 })
                .id();

            // Spawn the background
            let hand = world.resource::<AssetServer>().load("convo/bg.png");
            world
                .commands()
                .spawn((
                    Name::new("bg"),
                    SpriteBundle {
                        texture: hand,
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, ZIX_CONVO_BG)),
                        ..default()
                    },
                    MenuLayer::to_render_layers(),
                ))
                .set_parent(box_root_eid);
        });
    }
}

impl Component for ConvoBoxRootLoading {
    const STORAGE_TYPE: StorageType = StorageType::Table;
}
