#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(1)
var input_texture: texture_2d<f32>;
@group(2) @binding(2)
var input_splr: sampler;

@group(2) @binding(3)
var light_texture: texture_2d<f32>;
@group(2) @binding(4)
var light_splr: sampler;

@group(2) @binding(5)
var<uniform> zero: vec4<f32>;
@group(2) @binding(6)
var<uniform> one: vec4<f32>;
@group(2) @binding(7)
var<uniform> two: vec4<f32>;
@group(2) @binding(8)
var<uniform> three: vec4<f32>;
@group(2) @binding(9)
var<uniform> four: vec4<f32>;
@group(2) @binding(10)
var<uniform> five: vec4<f32>;
@group(2) @binding(11)
var<uniform> six: vec4<f32>;
@group(2) @binding(12)
var<uniform> seven: vec4<f32>;

const ref_zero = vec3<f32>(13.0 / 255.0, 0.0 / 255.0, 26.0 / 255.0);
const ref_one = vec3<f32>(46.0 / 255.0, 10.0 / 255.0, 48.0 / 255.0);
const ref_two = vec3<f32>(79.0 / 255.0, 20.0 / 255.0, 70.0 / 255.0);
const ref_three = vec3<f32>(111.0 / 255.0, 29.0 / 255.0, 92.0 / 255.0);
const ref_four = vec3<f32>(110.0 / 255.0, 81.0 / 255.0, 129.0 / 255.0);
const ref_five = vec3<f32>(109.0 / 255.0, 133.0 / 255.0, 165.0 / 255.0);
const ref_six = vec3<f32>(108.0 / 255.0, 185.0 / 255.0, 201.0 / 255.0);
const ref_seven = vec3<f32>(108.0 / 255.0, 237.0 / 255.0, 237.0 / 255.0);

const warning_yellow = vec4<f32>(1.0, 1.0, 0.0, 1.0);

// This is fucky for reasons I don't understand (non-linear colors?)
// _but_ _it_ _works_
fn quantize(color: vec3<f32>) -> f32 {
    if (color.y < 0.1 / 255.0) {
        return 0.0;
    }
    if (color.y < 1.0 / 255.0) {
        return 1.0;
    }
    if (color.y < 3.0 / 255.0) {
        return 2.0;
    }
    if (color.y < 5.0 / 255.0) {
        return 3.0;
    }
    if (color.y < 40.0 / 255.0) {
        return 4.0;
    }
    if (color.y < 80.0 / 255.0) {
        return 5.0;
    }
    if (color.y < 160.0 / 255.0) {
        return 6.0;
    }
    return 7.0;
    // if (color.y < 250.0 / 255.0) {
    //     return 7;
    // }
    // return -1;
}

fn get_light_diff(light: vec4<f32>) -> f32 {
    return light.x;
    // if light.x > 0.0 {
    //     return 1;
    // }
    // return 0;
}

fn as_final_palette(quantized: f32) -> vec4<f32> {
    if (quantized < 0.0) {
        return zero;
    }
    var base = zero;
    var diff = zero;
    var mult = 0.0;
    if (quantized < 1.0) {
        base = zero;
        diff = one - zero;
        mult = quantized;
    } else if (quantized < 2.0) {
        base = one;
        diff = two - one;
        mult = quantized - 1.0;
    } else if (quantized < 3.0) {
        base = two;
        diff = three - two;
        mult = quantized - 2.0;
    } else if (quantized < 4.0) {
        base = three;
        diff = four - three;
        mult = quantized - 3.0;
    } else if (quantized < 5.0) {
        base = four;
        diff = five - four;
        mult = quantized - 4.0;
    } else if (quantized < 6.0) {
        base = five;
        diff = six - five;
        mult = quantized - 5.0;
    } else if (quantized < 7.0) {
        base = six;
        diff = seven - six;
        mult = quantized - 6.0;
    } else {
        return seven;
    }
    return base + diff * mult;
}

// fn as_final_palette(quantized: i32) -> vec4<f32> {
//     if (quantized <= 0) {
//         return zero;
//     }
//     if (quantized == 1) {
//         return one;
//     }
//     if (quantized == 2) {
//         return two;
//     }
//     if (quantized == 3) {
//         return three;
//     }
//     if (quantized == 4) {
//         return four;
//     }
//     if (quantized == 5) {
//         return five;
//     }
//     if (quantized == 6) {
//         return six;
//     }
//     if (quantized >= 7) {
//         return seven;
//     }
//     // NOTE: Taking this (the logic giving this teeth a.k.a min/max checking) out because lighting may push below/beyond max
//     // This is a warning bright yellow. Should not happen
//     return warning_yellow;
// }

fn to_linear(nonlinear: vec4<f32>) -> vec4<f32> {
    let cutoff = step(nonlinear, vec4<f32>(0.04045));
    let higher = pow((nonlinear + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
    let lower = nonlinear / vec4<f32>(12.92);
    return mix(higher, lower, cutoff);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let original = textureSample(input_texture, input_splr, in.uv);
    let light = textureSample(light_texture, light_splr, in.uv);

    let quantized = quantize(vec3<f32>(original.x, original.y, original.z));
    let light_diff = get_light_diff(light);

    let finalized = as_final_palette(quantized + light_diff - 1.0);
    return to_linear(finalized);
}
