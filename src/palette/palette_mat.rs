use bevy::{
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::prelude::*;

#[derive(AsBindGroup, Debug, Clone, Asset, Reflect, PartialEq)]
pub struct PaletteMat {
    #[texture(1)]
    #[sampler(2)]
    input: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    light: Handle<Image>,
    #[uniform(5)]
    zero: Vec4,
    #[uniform(6)]
    one: Vec4,
    #[uniform(7)]
    two: Vec4,
    #[uniform(8)]
    three: Vec4,
    #[uniform(9)]
    four: Vec4,
    #[uniform(10)]
    five: Vec4,
    #[uniform(11)]
    six: Vec4,
    #[uniform(12)]
    seven: Vec4,
}
impl Material2d for PaletteMat {
    fn fragment_shader() -> ShaderRef {
        "shaders/palette_mat.wgsl".into()
    }
}
impl PaletteMat {
    pub fn new(input: Handle<Image>, light: Handle<Image>, palette: Palette) -> Self {
        Self {
            input,
            light,
            zero: color_as_vec4(palette.zero),
            one: color_as_vec4(palette.one),
            two: color_as_vec4(palette.two),
            three: color_as_vec4(palette.three),
            four: color_as_vec4(palette.four),
            five: color_as_vec4(palette.five),
            six: color_as_vec4(palette.six),
            seven: color_as_vec4(palette.seven),
        }
    }
}
