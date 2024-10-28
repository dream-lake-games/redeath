use bevy::{
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::prelude::*;

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
            zero: color_as_vec4(palette[0]),
            one: color_as_vec4(palette[1]),
            two: color_as_vec4(palette[2]),
            three: color_as_vec4(palette[3]),
            four: color_as_vec4(palette[4]),
            five: color_as_vec4(palette[5]),
            six: color_as_vec4(palette[6]),
            seven: color_as_vec4(palette[7]),
        }
    }
    pub fn take_shifted_palette(&mut self, shift_amt: i32, palette: &Palette) {
        self.zero = color_as_vec4(palette[0 + shift_amt]);
        self.one = color_as_vec4(palette[1 + shift_amt]);
        self.two = color_as_vec4(palette[2 + shift_amt]);
        self.three = color_as_vec4(palette[3 + shift_amt]);
        self.four = color_as_vec4(palette[4 + shift_amt]);
        self.five = color_as_vec4(palette[5 + shift_amt]);
        self.six = color_as_vec4(palette[6 + shift_amt]);
        self.seven = color_as_vec4(palette[7 + shift_amt]);
    }
}
