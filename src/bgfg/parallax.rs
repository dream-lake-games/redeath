use crate::prelude::*;

#[derive(Clone, Debug, Reflect)]
pub(super) struct ParallaxX {
    scratch: f32,
    pub(super) mult: f32,
    pub(super) wrap_size: f32,
}
impl ParallaxX {
    pub(super) fn new(mult: f32, wrap_size: f32) -> Self {
        Self {
            scratch: 0.0,
            mult,
            wrap_size,
        }
    }
}
impl Component for ParallaxX {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let x = world.get::<Transform>(eid).unwrap().translation.x;
            let mut myself = world.get_mut::<Self>(eid).unwrap();
            myself.scratch = x;
        });
    }
}

#[derive(Clone, Debug, Reflect)]
struct ParallaxY {
    scratch: f32,
    mult: f32,
    wrap_size: f32,
}
impl ParallaxY {
    fn new(mult: f32, wrap_size: f32) -> Self {
        Self {
            scratch: 0.0,
            mult,
            wrap_size,
        }
    }
}
impl Component for ParallaxY {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let y = world.get::<Transform>(eid).unwrap().translation.y;
            let mut myself = world.get_mut::<Self>(eid).unwrap();
            myself.scratch = y;
        });
    }
}

#[derive(Resource)]
struct LastCameraPos(Pos);

fn update_parallaxes(
    camera: Query<&Pos, (With<DynamicCamera>, Without<ParallaxX>, Without<ParallaxY>)>,
    mut last_camera_tran: ResMut<LastCameraPos>,
    mut pxs: Query<(&mut Transform, &mut ParallaxX), (Without<DynamicCamera>, Without<ParallaxY>)>,
    mut pys: Query<(&mut Transform, &mut ParallaxY), (Without<DynamicCamera>, Without<ParallaxY>)>,
) {
    let Ok(cam_pos) = camera.get_single() else {
        return;
    };
    let diff = last_camera_tran.0.as_vec2() - cam_pos.as_vec2();
    last_camera_tran.0 = cam_pos.clone();
    for (mut tran, mut px) in &mut pxs {
        px.scratch = tran.translation.x + diff.x * px.mult;
        while px.scratch > px.wrap_size / 2.0 {
            px.scratch += tran.translation.x.signum() * -1.0 * px.wrap_size;
        }
        tran.translation.x = px.scratch;
    }
    for (mut tran, mut py) in &mut pys {
        py.scratch = tran.translation.y + diff.y * py.mult;
        while tran.translation.y.abs() > py.wrap_size / 2.0 {
            tran.translation.y += tran.translation.y.signum() * -1.0 * py.wrap_size;
        }
        tran.translation.y = py.scratch;
    }
}

pub struct BlackScreenImage;
impl Component for BlackScreenImage {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let root = world.resource::<BgFgRoot>().eid();
            // Spawn the common stuff
            world
                .commands()
                .entity(eid)
                .insert((
                    Name::new(format!("black")),
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(SCREEN_VEC),
                            color: BERRY_NEBULA.zero,
                            ..default()
                        },
                        transform: Transform::from_translation(Vec2::ZERO.extend(-100.0)),
                        ..default()
                    },
                    BgLayer::to_render_layers(),
                ))
                .set_parent(root);
        });
    }
}

pub struct ParallaxScreenImage {
    path: String,
    size: UVec2,
    zix: f32,
    x_mult: Option<f32>,
    y_mult: Option<f32>,
    render_layers: RenderLayers,
}
impl ParallaxScreenImage {
    pub fn new_bg<S: AsRef<str>>(path: S, size_x: u32, size_y: u32, zix: f32) -> Self {
        Self {
            path: path.as_ref().to_string(),
            size: UVec2::new(size_x, size_y),
            zix,
            x_mult: None,
            y_mult: None,
            render_layers: BgLayer::to_render_layers(),
        }
    }
    pub fn new_fg<S: AsRef<str>>(path: S, size_x: u32, size_y: u32, zix: f32) -> Self {
        Self {
            path: path.as_ref().to_string(),
            size: UVec2::new(size_x, size_y),
            zix,
            x_mult: None,
            y_mult: None,
            render_layers: FgLayer::to_render_layers(),
        }
    }
    pub fn with_parallax_x(mut self, mult: f32) -> Self {
        self.x_mult = Some(mult);
        self
    }
    pub fn with_parallax_y(mut self, mult: f32) -> Self {
        self.y_mult = Some(mult);
        self
    }
}
impl Component for ParallaxScreenImage {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            // Get my shit
            let myself = world.get::<Self>(eid).unwrap();
            let path = myself.path.clone();
            let size = myself.size;
            let zix = myself.zix;
            let (x_mult, y_mult) = (myself.x_mult.clone(), myself.y_mult.clone());
            let rl = myself.render_layers.clone();
            let sprite_size = (size
                * UVec2::new(
                    if myself.x_mult.is_some() { 3 } else { 1 },
                    if myself.y_mult.is_some() { 3 } else { 1 },
                ))
            .as_vec2();
            let ass = world.resource::<AssetServer>();
            let root = world.resource::<BgFgRoot>().eid();
            let texture = ass.load(&path).clone();
            // Spawn the common stuff
            world
                .commands()
                .entity(eid)
                .insert((
                    Name::new(format!("bg_fg_{path}")),
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(sprite_size),
                            ..default()
                        },
                        texture,
                        transform: Transform::from_translation(Vec2::ZERO.extend(zix)),
                        ..default()
                    },
                    ImageScaleMode::Tiled {
                        tile_x: true,
                        tile_y: true,
                        stretch_value: 1.0,
                    },
                    rl,
                ))
                .set_parent(root);
            // Maybe add the parallaxes
            if let Some(mult) = x_mult {
                world
                    .commands()
                    .entity(eid)
                    .insert(ParallaxX::new(mult, size.x as f32));
            }
            if let Some(mult) = y_mult {
                world
                    .commands()
                    .entity(eid)
                    .insert(ParallaxY::new(mult, size.y as f32));
            }
        });
    }
}

pub(super) fn register_parallax(app: &mut App) {
    app.insert_resource(LastCameraPos(Pos::new(0.0, 0.0)));
    app.add_systems(PostUpdate, (update_parallaxes).after(CameraSet));
}
