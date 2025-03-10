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

const ref_zero = vec3<f32>(13.0 / 255.0, 0.0 / 255.0, 26.0 / 255.0);
const ref_one = vec3<f32>(46.0 / 255.0, 10.0 / 255.0, 48.0 / 255.0);
const ref_two = vec3<f32>(79.0 / 255.0, 20.0 / 255.0, 70.0 / 255.0);
const ref_three = vec3<f32>(111.0 / 255.0, 29.0 / 255.0, 92.0 / 255.0);
const ref_four = vec3<f32>(110.0 / 255.0, 81.0 / 255.0, 129.0 / 255.0);
const ref_five = vec3<f32>(109.0 / 255.0, 133.0 / 255.0, 165.0 / 255.0);
const ref_six = vec3<f32>(108.0 / 255.0, 185.0 / 255.0, 201.0 / 255.0);
const ref_seven = vec3<f32>(108.0 / 255.0, 237.0 / 255.0, 237.0 / 255.0);

const none = vec4<f32>(0.0, 0.0, 0.0, 0.0);

// This is fucky for reasons I don't understand (non-linear colors?)
// _but_ _it_ _works_
fn quantize(color: vec3<f32>) -> i32 {
    if (color.y < 0.1 / 255.0) {
        return 0;
    }
    else if (color.y < 1.0 / 255.0) {
        return 1;
    }
    else if (color.y < 3.0 / 255.0) {
        return 2;
    }
    else if (color.y < 5.0 / 255.0) {
        return 3;
    }
    else if (color.y < 40.0 / 255.0) {
        return 4;
    }
    else if (color.y < 80.0 / 255.0) {
        return 5;
    }
    else if (color.y < 160.0 / 255.0) {
        return 6;
    }
    else {
        return 7;
    }
}

fn as_final_palette(quantized: i32) -> vec4<f32> {
    if (quantized <= 0) {
        return zero;
    } else if (quantized == 1) {
        return one;
    } else if (quantized == 2) {
        return two;
    } else if (quantized == 3) {
        return three;
    } else if (quantized == 4) {
        return four;
    } else if (quantized == 5) {
        return five;
    } else if (quantized == 6) {
        return six;
    } else {
        return seven;
    }
}

fn to_linear(nonlinear: vec4<f32>) -> vec4<f32> {
    let cutoff = step(nonlinear, vec4<f32>(0.04045));
    let higher = pow((nonlinear + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
    let lower = nonlinear / vec4<f32>(12.92);
    return mix(higher, lower, cutoff);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let original = textureSample(input_texture, input_splr, in.uv);
    // If there's no alpha, return blank
    if (original[3] < 0.01) {
        return none;
    }
    // Otherwise quantize it
    let quantized = quantize(vec3<f32>(original.x, original.y, original.z));
    let finalized = as_final_palette(quantized);
    return to_linear(finalized);
}
