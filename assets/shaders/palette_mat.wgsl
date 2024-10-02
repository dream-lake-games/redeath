#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(1)
var input_texture: texture_2d<f32>;
@group(2) @binding(2)
var input_splr: sampler;
@group(2) @binding(3)
var<uniform> zero: vec4<f32>;
@group(2) @binding(4)
var<uniform> one: vec4<f32>;
@group(2) @binding(5)
var<uniform> two: vec4<f32>;
@group(2) @binding(6)
var<uniform> three: vec4<f32>;
@group(2) @binding(7)
var<uniform> four: vec4<f32>;
@group(2) @binding(8)
var<uniform> five: vec4<f32>;
@group(2) @binding(9)
var<uniform> six: vec4<f32>;
@group(2) @binding(10)
var<uniform> seven: vec4<f32>;

// fn to_linear(nonlinear: vec4<f32>) -> vec4<f32> {
//     let cutoff = step(nonlinear, vec4<f32>(0.04045));
//     let higher = pow((nonlinear + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
//     let lower = nonlinear / vec4<f32>(12.92);
//     return mix(higher, lower, cutoff);
// }

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(input_texture, input_splr, in.uv);
}
