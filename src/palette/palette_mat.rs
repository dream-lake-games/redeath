use bevy::render::render_resource::AsBindGroup;

use crate::prelude::*;

#[derive(AsBindGroup, Debug, Clone, Asset, Reflect, PartialEq)]
pub(crate) struct PaletteMat {
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
