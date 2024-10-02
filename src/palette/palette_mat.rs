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
    #[uniform(3)]
    zero: Vec4,
    #[uniform(4)]
    one: Vec4,
    #[uniform(5)]
    two: Vec4,
    #[uniform(6)]
    three: Vec4,
    #[uniform(7)]
    four: Vec4,
    #[uniform(8)]
    five: Vec4,
    #[uniform(9)]
    six: Vec4,
    #[uniform(10)]
    seven: Vec4,
}
impl Material2d for PaletteMat {
    fn fragment_shader() -> ShaderRef {
        "shaders/palette_mat.wgsl".into()
    }
}
impl PaletteMat {
    pub fn new(hand: Handle<Image>, palette: Palette) -> Self {
        Self {
            input: hand,
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
