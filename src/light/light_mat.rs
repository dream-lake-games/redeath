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
