use bevy::{
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};

use crate::prelude::*;

#[derive(AsBindGroup, Debug, Clone, Asset, Reflect, PartialEq)]
pub struct LightMat {
    #[texture(1)]
    #[sampler(2)]
    image: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    light: Handle<Image>,
    #[uniform(5)]
    base: Vec4,
}
impl Material2d for LightMat {
    fn fragment_shader() -> ShaderRef {
        "shaders/light_mat.wgsl".into()
    }
}
impl LightMat {
    pub fn new(image: Handle<Image>, light: Handle<Image>, base: Color) -> Self {
        Self {
            image,
            light,
            base: color_as_vec4(base),
        }
    }
}

#[derive(Resource, Clone, Debug, Reflect)]
pub struct BaseLights {
    pub ambience: Color,
    pub detail: Color,
    pub main: Color,
}

pub(super) fn register_lights(app: &mut App) {
    app.add_plugins(Material2dPlugin::<LightMat>::default());
    app.insert_resource(BaseLights {
        ambience: Color::srgba(0.4, 0.4, 0.4, 1.0),
        detail: Color::srgba(0.3, 0.3, 0.3, 1.0),
        main: Color::srgba(0.6, 0.6, 0.6, 1.0),
    });
}
