use bevy::{
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::prelude::*;

/// The mat that does the multiplying
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
    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
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

/// The mat that turns black to clear after doing dynamic light cutting
#[derive(AsBindGroup, Debug, Clone, Asset, Reflect, PartialEq)]
pub struct LightCutoutMat {
    #[texture(1)]
    #[sampler(2)]
    light: Handle<Image>,
}
impl Material2d for LightCutoutMat {
    fn fragment_shader() -> ShaderRef {
        "shaders/light_cutout_mat.wgsl".into()
    }
    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}
impl LightCutoutMat {
    pub fn new(light: Handle<Image>) -> Self {
        Self { light }
    }
}
