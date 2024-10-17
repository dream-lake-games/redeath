use bevy::render::camera::RenderTarget;

use crate::prelude::*;

#[derive(Resource, Clone, Debug, Reflect)]
struct LightManager {
    unused_render_layers: HashSet<usize>,
}
impl Default for LightManager {
    fn default() -> Self {
        let mut unused_render_layers = HashSet::new();
        for rl in BASE_LIGHT_RENDER_LAYER..(BASE_LIGHT_RENDER_LAYER + MAX_NUM_LIGHTS) {
            unused_render_layers.insert(rl);
        }
        Self {
            unused_render_layers,
        }
    }
}
impl LightManager {
    pub fn alloc(&mut self) -> usize {
        let rl = self
            .unused_render_layers
            .iter()
            .next()
            .expect("Too many lights, ran out")
            .clone();
        self.unused_render_layers.remove(&rl);
        rl
    }
    pub fn free(&mut self, rl: usize) {
        self.unused_render_layers.insert(rl);
    }
}

/// Signals that this component wants to have a light.
/// When added to a component, it will claim a render layer, add the animation, image, and camera
/// It will also add `LightClaimed`. Then it will remove itself. Cleanup is handled by LightClaimed.
#[derive(Debug, Reflect)]
pub struct Light {
    anim: LightAnim,
    singular: bool,
}
impl Light {
    pub fn new(anim: LightAnim) -> Self {
        Self {
            anim,
            singular: false,
        }
    }
    impl_with_on!(singular);
}
impl Component for Light {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            // Get my animation
            let myself = world.get::<Self>(eid).expect("myself");
            let anim = myself.anim.clone();
            let singular = myself.singular;

            // Claim a render layer
            let mut manager = world.resource_mut::<LightManager>();
            let rl_usize = manager.alloc();
            let rl = RenderLayers::from_layers(&[rl_usize]);

            // Add the animation
            let mut anim_man = AnimMan::<LightAnim>::new(anim).with_render_layers(rl.clone());
            if singular {
                anim_man = anim_man.with_singular();
            }
            world.commands().entity(eid).insert(anim_man);

            // Add the image
            let image = blank_screen_image();
            let mut images = world.resource_mut::<Assets<Image>>();
            let image_hand = images.add(image);

            // Spawn the camera
            let light_root_eid = world.resource::<LightRoot>().eid();
            let camera_eid = world
                .commands()
                .spawn((
                    Name::new("light_camera"),
                    Camera2dBundle {
                        camera: Camera {
                            order: LightLayer::to_i32() as isize - 1,
                            target: RenderTarget::Image(image_hand.clone()),
                            clear_color: ClearColorConfig::Custom(COLOR_NONE),
                            ..default()
                        },
                        projection: OrthographicProjection {
                            near: ZIX_MIN,
                            far: ZIX_MAX,
                            scale: 1.0,
                            ..default()
                        },
                        ..default()
                    },
                    rl.clone(),
                    FollowDynamicCamera,
                ))
                .set_parent(light_root_eid)
                .id();

            // Testing
            world.commands().spawn((
                SpriteBundle {
                    texture: image_hand.clone(),
                    ..default()
                },
                LightLayer::to_render_layers(),
            ));

            // Make the claim
            world.commands().entity(eid).insert(LightClaimed {
                rl: rl_usize,
                image: image_hand,
                camera: camera_eid,
            });
            world.commands().entity(eid).remove::<Light>();
        });
    }
}

#[derive(Debug, Reflect)]
pub struct LightClaimed {
    rl: usize,
    image: Handle<Image>,
    camera: Entity,
}
impl Component for LightClaimed {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_remove(|mut world, eid, _| {
            let myself = world.get::<Self>(eid).expect("myself");
            let rl = myself.rl;
            let camera = myself.camera;
            let mut manager = world.resource_mut::<LightManager>();
            manager.free(rl);
            world.commands().entity(camera).despawn_recursive();
        });
    }
}
impl LightClaimed {
    pub fn to_render_layers(&self) -> RenderLayers {
        RenderLayers::from_layers(&[self.rl])
    }
}

pub(super) fn register_light_manager(app: &mut App) {
    app.insert_resource(LightManager::default());
}
