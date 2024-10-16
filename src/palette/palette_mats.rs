use bevy::{
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::prelude::*;

#[derive(AsBindGroup, Debug, Clone, Asset, Reflect, PartialEq)]
pub struct SimplePaletteMat {
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
impl Material2d for SimplePaletteMat {
    fn fragment_shader() -> ShaderRef {
        "shaders/simple_palette_mat.wgsl".into()
    }
}
impl SimplePaletteMat {
    pub fn new(input: Handle<Image>, palette: Palette) -> Self {
        Self {
            input,
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

#[derive(AsBindGroup, Debug, Clone, Asset, Reflect, PartialEq)]
pub struct ShiftedPaletteMat {
    #[texture(1)]
    #[sampler(2)]
    input: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    shift: Handle<Image>,
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
impl Material2d for ShiftedPaletteMat {
    fn fragment_shader() -> ShaderRef {
        "shaders/shifted_palette_mat.wgsl".into()
    }
}
impl ShiftedPaletteMat {
    pub fn new(input: Handle<Image>, shift: Handle<Image>, palette: Palette) -> Self {
        Self {
            input,
            shift,
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
