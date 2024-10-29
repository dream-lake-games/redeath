use crate::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::{
    render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    texture::BevyDefault,
    view::RenderLayers,
};
use bevy::sprite::Mesh2dHandle;
use bevy::window::WindowResized;

pub trait Layer: Into<RenderLayers> + Default {
    fn to_render_layers() -> RenderLayers {
        Self::default().into()
    }
    fn to_i32() -> i32;
}

macro_rules! decl_layer {
    ($name:ident, $order:literal) => {
        #[derive(Component, Debug, Reflect, Default)]
        pub struct $name;
        impl Into<RenderLayers> for $name {
            fn into(self) -> RenderLayers {
                RenderLayers::from_layers(&[$order])
            }
        }
        impl Layer for $name {
            fn to_i32() -> i32 {
                $order
            }
        }
    };
}
// IGNORED!
decl_layer!(DummyLayer, 10);
decl_layer!(BgLayer, 10);
decl_layer!(PaletteLayer, 20);
decl_layer!(LightLayer, 30);
decl_layer!(MainAmbienceLayer, 40);
decl_layer!(MainDetailLayer, 41);
decl_layer!(StaticLayer, 50);
decl_layer!(MainLayer, 55);
decl_layer!(FgLayer, 60);
decl_layer!(MenuLayer, 61);
decl_layer!(TransitionLayer, 62);

/// Grows all of the layers by a given scale.
/// Makes it easy for the game to fill the screen in a satisfying way.
#[derive(Resource, Clone, Copy)]
pub struct LayerGrowth {
    scale: f32,
}
impl LayerGrowth {
    impl_get_copy!(scale, f32);
    impl_set!(scale, f32);
}
impl Default for LayerGrowth {
    fn default() -> Self {
        Self { scale: 1.0 }
    }
}

/// Creates a new blank image the size of the screen
pub fn blank_screen_image() -> Image {
    let target_extent = Extent3d {
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
        ..default()
    };
    // Makes the image
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: target_extent,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    // Fills it with zeros
    image.resize(target_extent);
    image
}

#[derive(Debug, Resource, Clone)]
struct CameraTargets {
    bg_target: Handle<Image>,
    main_ambience_target: Handle<Image>,
    main_ambience_shifted: Handle<Image>,
    main_detail_target: Handle<Image>,
    main_detail_shifted: Handle<Image>,
    main_target: Handle<Image>,
    main_shifted: Handle<Image>,
    palette_target: Handle<Image>,
    light_target: Handle<Image>,
    static_target: Handle<Image>,
    fg_target: Handle<Image>,
    menu_target: Handle<Image>,
    transition_target: Handle<Image>,
    final_target: Handle<Image>,
}
impl Default for CameraTargets {
    fn default() -> Self {
        Self {
            bg_target: Handle::weak_from_u128(thread_rng().gen()),
            main_ambience_target: Handle::weak_from_u128(thread_rng().gen()),
            main_ambience_shifted: Handle::weak_from_u128(thread_rng().gen()),
            main_detail_target: Handle::weak_from_u128(thread_rng().gen()),
            main_detail_shifted: Handle::weak_from_u128(thread_rng().gen()),
            main_target: Handle::weak_from_u128(thread_rng().gen()),
            main_shifted: Handle::weak_from_u128(thread_rng().gen()),
            palette_target: Handle::weak_from_u128(thread_rng().gen()),
            light_target: Handle::weak_from_u128(thread_rng().gen()),
            static_target: Handle::weak_from_u128(thread_rng().gen()),
            fg_target: Handle::weak_from_u128(thread_rng().gen()),
            menu_target: Handle::weak_from_u128(thread_rng().gen()),
            transition_target: Handle::weak_from_u128(thread_rng().gen()),
            final_target: Handle::weak_from_u128(thread_rng().gen()),
        }
    }
}
impl CameraTargets {
    /// Creates actual images that the various layers can write to to place on quads.
    pub fn initialize(&self, images: &mut Assets<Image>) {
        macro_rules! make_layer_image {
            ($handle:expr) => {{
                let image = blank_screen_image();
                images.insert($handle.id(), image);
            }};
        }
        make_layer_image!(self.bg_target);
        make_layer_image!(self.main_ambience_target);
        make_layer_image!(self.main_ambience_shifted);
        make_layer_image!(self.main_detail_target);
        make_layer_image!(self.main_detail_shifted);
        make_layer_image!(self.main_target);
        make_layer_image!(self.main_shifted);
        make_layer_image!(self.palette_target);
        make_layer_image!(self.light_target);
        make_layer_image!(self.static_target);
        make_layer_image!(self.fg_target);
        make_layer_image!(self.menu_target);
        make_layer_image!(self.transition_target);
        make_layer_image!(self.final_target);
    }
}

fn setup_layer_materials(
    root: Res<LayerRoot>,
    palette: Res<Palette>,
    mut commands: Commands,
    camera_targets: Res<CameraTargets>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut shifted_palette_mats: ResMut<Assets<ShiftedPaletteMat>>,
    mut light_mats: ResMut<Assets<LightMat>>,
    base_lights: Res<BaseLights>,
) {
    // For the juicy palette shifting and lighting work
    let main_ambience_layer = RenderLayers::from_layers(&[25]);
    let main_detail_layer = RenderLayers::from_layers(&[26]);
    let main_layer = RenderLayers::from_layers(&[27]);
    let squash_layer = RenderLayers::from_layers(&[28]);
    let final_layer = RenderLayers::from_layers(&[31]);

    camera_targets.initialize(&mut images);

    /// Sets up a layer that applies palette transform and shifting but no lighting
    fn setup_simple_layer(
        name: &str,
        image: Handle<Image>,
        shift: Handle<Image>,
        zix: i32,
        palette: Palette,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        shifted_palette_mats: &mut Assets<ShiftedPaletteMat>,
        squash_layer: RenderLayers,
        root: Entity,
    ) {
        // Apply the palette shift
        let shifted_mesh = Mesh::from(Rectangle::new(SCREEN_WIDTH_f32, SCREEN_HEIGHT_f32));
        let shifted_mesh: Mesh2dHandle = meshes.add(shifted_mesh).into();
        let shifted_mat = shifted_palette_mats.add(ShiftedPaletteMat::new(image, shift, palette));
        // Then draw
        commands
            .spawn((
                Name::new(format!("{name}_intermediate_image")),
                shifted_mesh,
                shifted_mat,
                SpatialBundle::from(Transform::from_translation(Vec3::Z * zix as f32)),
                squash_layer.clone(),
            ))
            .set_parent(root);
    }

    setup_simple_layer(
        "bg_image",
        camera_targets.bg_target.clone(),
        camera_targets.palette_target.clone(),
        BgLayer::to_i32(),
        palette.clone(),
        &mut commands,
        meshes.as_mut(),
        shifted_palette_mats.as_mut(),
        squash_layer.clone(),
        root.eid(),
    );
    setup_simple_layer(
        "static_image",
        camera_targets.static_target.clone(),
        camera_targets.palette_target.clone(),
        StaticLayer::to_i32(),
        palette.clone(),
        &mut commands,
        meshes.as_mut(),
        shifted_palette_mats.as_mut(),
        squash_layer.clone(),
        root.eid(),
    );
    setup_simple_layer(
        "fg_image",
        camera_targets.fg_target.clone(),
        camera_targets.palette_target.clone(),
        FgLayer::to_i32(),
        palette.clone(),
        &mut commands,
        meshes.as_mut(),
        shifted_palette_mats.as_mut(),
        squash_layer.clone(),
        root.eid(),
    );
    setup_simple_layer(
        "menu_image",
        camera_targets.menu_target.clone(),
        camera_targets.palette_target.clone(),
        MenuLayer::to_i32(),
        palette.clone(),
        &mut commands,
        meshes.as_mut(),
        shifted_palette_mats.as_mut(),
        squash_layer.clone(),
        root.eid(),
    );
    setup_simple_layer(
        "transition_image",
        camera_targets.transition_target.clone(),
        camera_targets.palette_target.clone(),
        TransitionLayer::to_i32(),
        palette.clone(),
        &mut commands,
        meshes.as_mut(),
        shifted_palette_mats.as_mut(),
        squash_layer.clone(),
        root.eid(),
    );

    /// Sets up a layer that applies both shifting and lighting
    fn setup_complex_layer(
        name: &str,
        image: Handle<Image>,
        shift: Handle<Image>,
        light: Handle<Image>,
        intermediate_image: Handle<Image>,
        zix: i32,
        palette: Palette,
        base_light: Color,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        shifted_palette_mats: &mut Assets<ShiftedPaletteMat>,
        light_mats: &mut Assets<LightMat>,
        intermediate_layer: RenderLayers,
        squash_layer: RenderLayers,
        root: Entity,
    ) {
        // First apply the palette shift
        let shifted_mesh = Mesh::from(Rectangle::new(SCREEN_WIDTH_f32, SCREEN_HEIGHT_f32));
        let shifted_mesh: Mesh2dHandle = meshes.add(shifted_mesh).into();
        let shifted_mat = shifted_palette_mats.add(ShiftedPaletteMat::new(image, shift, palette));
        // Then draw this
        commands
            .spawn((
                Name::new(format!("{name}_intermediate_image")),
                shifted_mesh,
                shifted_mat,
                SpatialBundle::from(Transform::from_translation(Vec3::Z * zix as f32)),
                intermediate_layer.clone(),
            ))
            .set_parent(root);
        commands
            .spawn((
                Name::new(format!("{name}_intermediate_camera")),
                Camera2dBundle {
                    camera: Camera {
                        order: zix as isize,
                        target: RenderTarget::Image(intermediate_image.clone()),
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
                intermediate_layer.clone(),
            ))
            .set_parent(root);
        // Then apply light
        let lighted_mesh = Mesh::from(Rectangle::new(SCREEN_WIDTH_f32, SCREEN_HEIGHT_f32));
        let lighted_mesh: Mesh2dHandle = meshes.add(lighted_mesh).into();
        let lighted_mat = light_mats.add(LightMat::new(
            intermediate_image.clone(),
            light.clone(),
            base_light,
        ));
        commands
            .spawn((
                Name::new(format!("{name}_image")),
                lighted_mesh,
                lighted_mat,
                SpatialBundle::from(Transform::from_translation(Vec3::Z * zix as f32)),
                squash_layer,
            ))
            .set_parent(root);
    }

    setup_complex_layer(
        "ambience",
        camera_targets.main_ambience_target.clone(),
        camera_targets.palette_target.clone(),
        camera_targets.light_target.clone(),
        camera_targets.main_ambience_shifted.clone(),
        MainAmbienceLayer::to_i32(),
        palette.clone(),
        base_lights.ambience,
        &mut commands,
        meshes.as_mut(),
        shifted_palette_mats.as_mut(),
        light_mats.as_mut(),
        main_ambience_layer.clone(),
        squash_layer.clone(),
        root.eid(),
    );
    setup_complex_layer(
        "detail",
        camera_targets.main_detail_target.clone(),
        camera_targets.palette_target.clone(),
        camera_targets.light_target.clone(),
        camera_targets.main_detail_shifted.clone(),
        MainDetailLayer::to_i32(),
        palette.clone(),
        base_lights.detail,
        &mut commands,
        meshes.as_mut(),
        shifted_palette_mats.as_mut(),
        light_mats.as_mut(),
        main_detail_layer.clone(),
        squash_layer.clone(),
        root.eid(),
    );
    setup_complex_layer(
        "main",
        camera_targets.main_target.clone(),
        camera_targets.palette_target.clone(),
        camera_targets.light_target.clone(),
        camera_targets.main_shifted.clone(),
        MainLayer::to_i32(),
        palette.clone(),
        base_lights.main,
        &mut commands,
        meshes.as_mut(),
        shifted_palette_mats.as_mut(),
        light_mats.as_mut(),
        main_layer.clone(),
        squash_layer.clone(),
        root.eid(),
    );

    // This is the camera that sees all of the layer quads and squashes them into one image
    commands
        .spawn((
            Name::new("squash_camera"),
            Camera2dBundle {
                camera: Camera {
                    order: TransitionLayer::to_i32() as isize + 1,
                    clear_color: ClearColorConfig::Custom(COLOR_NONE),
                    target: RenderTarget::Image(camera_targets.final_target.clone()),
                    ..default()
                },
                ..default()
            },
            InheritedVisibility::VISIBLE,
            squash_layer.clone(),
        ))
        .set_parent(root.eid());

    // This sprite just scales up and down to fill the screen
    commands
        .spawn((
            Name::new("final_sprite"),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(SCREEN_VEC),
                    ..default()
                },
                texture: camera_targets.final_target.clone(),
                ..default()
            },
            ResizeFinalImage,
            final_layer.clone(),
        ))
        .set_parent(root.eid());

    // This is currently the final camera, seeing the scaled output
    commands
        .spawn((
            Name::new("final_camera"),
            Camera2dBundle {
                camera: Camera {
                    order: TransitionLayer::to_i32() as isize + 2,
                    clear_color: ClearColorConfig::Custom(COLOR_NONE),
                    ..default()
                },
                ..default()
            },
            InheritedVisibility::VISIBLE,
            final_layer,
        ))
        .set_parent(root.eid());
}

fn setup_layer_cameras(
    mut commands: Commands,
    camera_targets: Res<CameraTargets>,
    layer_root: Res<LayerRoot>,
) {
    macro_rules! spawn_layer_camera {
        ($comp:ty, $name:literal, $image:expr, $follow_dynamic:expr) => {{
            let mut comms = commands.spawn((
                Name::new($name),
                Camera2dBundle {
                    camera: Camera {
                        order: <$comp>::to_i32() as isize,
                        target: RenderTarget::Image($image),
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
                <$comp>::default(),
                <$comp>::to_render_layers(),
            ));
            comms.set_parent(layer_root.eid());
            if $follow_dynamic {
                comms.insert(FollowDynamicCamera);
            }
        }};
    }
    spawn_layer_camera!(
        BgLayer,
        "bg_camera",
        camera_targets.bg_target.clone(),
        false
    );
    spawn_layer_camera!(
        StaticLayer,
        "static_camera",
        camera_targets.static_target.clone(),
        true
    );
    spawn_layer_camera!(
        MainAmbienceLayer,
        "main_ambience_camera",
        camera_targets.main_ambience_target.clone(),
        true
    );
    spawn_layer_camera!(
        MainDetailLayer,
        "main_detail_camera",
        camera_targets.main_detail_target.clone(),
        true
    );
    spawn_layer_camera!(
        MainLayer,
        "main_camera",
        camera_targets.main_target.clone(),
        true
    );
    spawn_layer_camera!(
        PaletteLayer,
        "palette_camera",
        camera_targets.palette_target.clone(),
        true
    );
    spawn_layer_camera!(
        LightLayer,
        "light_camera",
        camera_targets.light_target.clone(),
        false
    );
    spawn_layer_camera!(
        FgLayer,
        "fg_camera",
        camera_targets.fg_target.clone(),
        false
    );
    spawn_layer_camera!(
        MenuLayer,
        "menu_camera",
        camera_targets.menu_target.clone(),
        false
    );
    spawn_layer_camera!(
        TransitionLayer,
        "transition_camera",
        camera_targets.transition_target.clone(),
        false
    );
}

/// Marks the layer images which should respond to resizing events
#[derive(Component)]
struct ResizeFinalImage;

/// After resizing to fill the screen (as best we can), how many mults over screen size did we go?
#[derive(Resource, Debug, Reflect)]
pub struct OverScreenMult(pub f32);

fn resize_canvases(
    mut events: EventReader<WindowResized>,
    mut quad_trans: Query<&mut Transform, With<ResizeFinalImage>>,
    mut over_screen_mult: ResMut<OverScreenMult>,
) {
    let Some(event) = events.read().last() else {
        return;
    };

    // Mult is smallest to fill either vertically or horizontally
    // A.k.a don't cut anything off
    let width_mult = event.width / SCREEN_WIDTH_f32;
    let height_mult = event.height / SCREEN_HEIGHT_f32;
    let mult = width_mult.min(height_mult);
    over_screen_mult.0 = mult;

    // Then update the layering quads
    for mut tran in &mut quad_trans {
        tran.scale = (Vec2::ONE * mult).extend(1.0);
    }
}

#[derive(Default)]
pub struct LayerPlugin;
impl Plugin for LayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LayerGrowth::default());
        app.insert_resource(CameraTargets::default());
        app.insert_resource(OverScreenMult(1.0));

        app.add_systems(
            Startup,
            (setup_layer_materials, setup_layer_cameras)
                .chain()
                .after(RootInit),
        );
        app.add_systems(Update, resize_canvases);
    }
}
