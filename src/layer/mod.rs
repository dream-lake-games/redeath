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
        impl Layer for $name {}
    };
}
decl_layer!(BgLayer, 1);
decl_layer!(MainLayer, 2);
decl_layer!(FgLayer, 3);
decl_layer!(MenuLayer, 4);
decl_layer!(TransitionLayer, 5);

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

/// Configures the clear colors and ambient light of the layers.
#[derive(Debug, Resource, Clone)]
pub struct LayerClearColors {
    bg_clear_color: ClearColorConfig,
    main_clear_color: ClearColorConfig,
    fg_clear_color: ClearColorConfig,
    menu_clear_color: ClearColorConfig,
    transition_clear_color: ClearColorConfig,
}
macro_rules! impl_clear_color_config_field {
    ($name:ident) => {
        impl_get_copy!($name, ClearColorConfig);
        impl_set!($name, ClearColorConfig);
        impl_with!($name, ClearColorConfig);
    };
}
impl LayerClearColors {
    impl_clear_color_config_field!(bg_clear_color);
    impl_clear_color_config_field!(main_clear_color);
    impl_clear_color_config_field!(fg_clear_color);
    impl_clear_color_config_field!(menu_clear_color);
    impl_clear_color_config_field!(transition_clear_color);
}
impl Default for LayerClearColors {
    fn default() -> Self {
        Self {
            bg_clear_color: ClearColorConfig::Custom(COLOR_NONE),
            main_clear_color: ClearColorConfig::Custom(COLOR_NONE),
            fg_clear_color: ClearColorConfig::Custom(COLOR_NONE),
            menu_clear_color: ClearColorConfig::Custom(COLOR_NONE),
            transition_clear_color: ClearColorConfig::Custom(COLOR_NONE),
        }
    }
}

#[derive(Debug, Resource, Clone)]
struct CameraTargets {
    bg_target: Handle<Image>,
    main_target: Handle<Image>,
    palette_target: Handle<Image>,
    fg_target: Handle<Image>,
    menu_target: Handle<Image>,
    transition_target: Handle<Image>,
    final_target: Handle<Image>,
}
impl Default for CameraTargets {
    fn default() -> Self {
        Self {
            bg_target: Handle::weak_from_u128(thread_rng().gen()),
            main_target: Handle::weak_from_u128(thread_rng().gen()),
            palette_target: Handle::weak_from_u128(thread_rng().gen()),
            fg_target: Handle::weak_from_u128(thread_rng().gen()),
            menu_target: Handle::weak_from_u128(thread_rng().gen()),
            transition_target: Handle::weak_from_u128(thread_rng().gen()),
            final_target: Handle::weak_from_u128(thread_rng().gen()),
        }
    }
}
impl CameraTargets {
    /// Creates actually images that the various layers can write to to place on quads.
    pub fn initialize(&self, images: &mut Assets<Image>) {
        macro_rules! make_layer_image {
            ($label:expr, $handle:expr) => {{
                let target_extent = Extent3d {
                    width: SCREEN_WIDTH,
                    height: SCREEN_HEIGHT,
                    ..default()
                };
                // Makes the image
                let mut image = Image {
                    texture_descriptor: TextureDescriptor {
                        label: Some($label),
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
                images.insert($handle.id(), image);
            }};
        }

        make_layer_image!("bg_target", self.bg_target);
        make_layer_image!("main_target", self.main_target);
        make_layer_image!("palette_target", self.palette_target);
        make_layer_image!("fg_target", self.fg_target);
        make_layer_image!("menu_target", self.menu_target);
        make_layer_image!("transition_target", self.transition_target);
        make_layer_image!("final_target", self.final_target);
    }
}

fn setup_layer_materials(
    root: Res<LayerRoot>,
    palette: Res<Palette>,
    mut commands: Commands,
    camera_targets: Res<CameraTargets>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut palette_mats: ResMut<Assets<PaletteMat>>,
) {
    let palette_mesh = Mesh::from(Rectangle::new(SCREEN_WIDTH_f32, SCREEN_HEIGHT_f32));
    let palette_mesh: Mesh2dHandle = meshes.add(palette_mesh).into();
    let palette_mat = PaletteMat::new(camera_targets.main_target.clone(), palette.clone());
    let squash_layer = RenderLayers::from_layers(&[29]);
    let final_layer = RenderLayers::from_layers(&[30]);

    camera_targets.initialize(&mut images);

    macro_rules! setup_layer {
        ($name:literal, $image:expr, $zix:literal) => {
            commands
                .spawn((
                    Name::new($name),
                    SpriteBundle {
                        transform: Transform::from_translation(Vec3::Z * $zix as f32),
                        texture: $image,
                        ..default()
                    },
                    squash_layer.clone(),
                ))
                .set_parent(root.eid());
        };
    }
    setup_layer!("bg_image", camera_targets.bg_target.clone(), 1);
    // NOTE: Don't add because palette mat reads it, edits it, and then renders it
    // setup_layer!("main_image", camera_targets.main_target.clone(), 2);
    commands
        .spawn((
            Name::new("palette_image"),
            palette_mesh,
            palette_mats.add(palette_mat),
            SpatialBundle::from_transform(Transform::from_translation(Vec3::Z * 3.0)),
            squash_layer.clone(),
        ))
        .set_parent(root.eid());
    setup_layer!("fg_image", camera_targets.fg_target.clone(), 4);
    setup_layer!("menu_image", camera_targets.menu_target.clone(), 5);
    setup_layer!(
        "transition_image",
        camera_targets.transition_target.clone(),
        6
    );

    // This is the camera that sees all of the layer quads and squashes them into one image
    commands
        .spawn((
            Name::new("squash_camera"),
            Camera2dBundle {
                camera: Camera {
                    order: 7,
                    clear_color: ClearColorConfig::Custom(COLOR_NONE),
                    target: RenderTarget::Image(camera_targets.final_target.clone()),
                    ..default()
                },
                ..default()
            },
            InheritedVisibility::VISIBLE,
            squash_layer,
        ))
        .set_parent(root.eid());

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

    // This is currently the final camera
    commands
        .spawn((
            Name::new("final_camera"),
            Camera2dBundle {
                camera: Camera {
                    order: 7,
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
    layer_colors: Res<LayerClearColors>,
    layer_root: Res<LayerRoot>,
) {
    macro_rules! spawn_layer_camera {
        ($comp:ty, $name:literal, $order:literal, $image:expr, $clear_color:expr, $follow_dynamic:expr) => {{
            let mut comms = commands.spawn((
                Name::new($name),
                Camera2dBundle {
                    camera: Camera {
                        order: $order,
                        target: RenderTarget::Image($image),
                        clear_color: $clear_color,
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
        1,
        camera_targets.bg_target.clone(),
        layer_colors.bg_clear_color,
        false
    );
    spawn_layer_camera!(
        MainLayer,
        "main_camera",
        2,
        camera_targets.main_target.clone(),
        layer_colors.main_clear_color,
        true
    );
    spawn_layer_camera!(
        FgLayer,
        "fg_camera",
        3,
        camera_targets.fg_target.clone(),
        layer_colors.fg_clear_color,
        false
    );
    spawn_layer_camera!(
        MenuLayer,
        "menu_camera",
        4,
        camera_targets.menu_target.clone(),
        layer_colors.menu_clear_color,
        false
    );
    spawn_layer_camera!(
        TransitionLayer,
        "transition_camera",
        5,
        camera_targets.transition_target.clone(),
        layer_colors.transition_clear_color,
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
        app.insert_resource(LayerClearColors::default());
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
