use bevy::{
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::prelude::*;

/// This mat is applied to the FINAL image
#[derive(AsBindGroup, Debug, Clone, Asset, Reflect, PartialEq)]
pub struct FinalPostProcessingMat {
    #[texture(1)]
    #[sampler(2)]
    image: Handle<Image>,
    #[uniform(3)]
    time: f32,
    #[uniform(4)]
    pub enabled: f32,
}
impl Material2d for FinalPostProcessingMat {
    fn fragment_shader() -> ShaderRef {
        "shaders/final_post_processing.wgsl".into()
    }
}
impl FinalPostProcessingMat {
    pub fn new(image: Handle<Image>) -> Self {
        Self {
            image,
            time: 0.0,
            enabled: 1.0,
        }
    }
}

pub(super) fn update_final_post_processing_mats(
    hands: Query<&Handle<FinalPostProcessingMat>>,
    mut mat: ResMut<Assets<FinalPostProcessingMat>>,
    time: Res<Time>,
) {
    for hand in &hands {
        let mat = mat.get_mut(hand.id()).unwrap();
        mat.time += time.delta_seconds();
    }
}
