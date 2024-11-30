#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(1)
var input_texture: texture_2d<f32>;
@group(2) @binding(2)
var input_splr: sampler;
@group(2) @binding(3)
var<uniform> time: f32;
@group(2) @binding(4)
var<uniform> enabled: f32;

// TODO: Maybe spend more time tuning, but also maybe its fine

const canvas_width = 240.0 * 38.0;
const canvas_height = 184.0 * 38.0;

const screen_curvature = 0.03;
const cell_size = 18.0;
const cell_offset = 0.0;
const border_mask = 0.2;

const pulse_intensity = 0.05;
const pulse_width = 100.0;
const pulse_rate = 20.0;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
  if enabled < 0.5 {
    return textureSample(input_texture, input_splr, in.uv);
  }

  var recentered = vec2<f32>(in.uv.x - 0.5, in.uv.y - 0.5) * 2.0;
  var curved = recentered * (1.0 + screen_curvature * dot(recentered, recentered));
  var uv = curved * 0.5 + 0.5;
  if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0) {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0); // Return black for off-screen pixels
  }
  var pixel = uv * vec2<f32>(canvas_width, canvas_height);
  var coord = pixel / cell_size;
  var subcoord = coord * vec2<f32>(select(cell_size, 3.0, cell_size >= 6.0), 1);
  var this_cell_offset = vec2<f32>(0, fract(floor(coord.x) * cell_offset));
  var mask_coord = floor(coord + this_cell_offset) * cell_size;
  var samplePoint = mask_coord / vec2<f32>(canvas_width, canvas_height);
  var abberation = textureSample(
    input_texture,
    input_splr,
    samplePoint
  ).xyz;
  var color = abberation;
  var ind = floor(subcoord.x) % 3;
  var mask_color = vec3<f32>(
    f32(ind == 0.0), 
    f32(ind == 1.0), 
    f32(ind == 2.0)
  ) * 3.0;
  var cell_uv = fract(subcoord + cell_offset) * 2.0 - 1.0;
  var border: vec2<f32> = 1.0 - cell_uv * cell_uv * border_mask;
  mask_color *= vec3f(clamp(border.x, 0.0, 1.0) * clamp(border.y, 0.0, 1.0));
  color *= vec3f(1.0 + (mask_color - 1.0) * 1.0);

  color.r *= 1.0 + pulse_intensity * sin(pixel.y / pulse_width + time * pulse_rate);
  color.b *= 1.0 + pulse_intensity * sin(pixel.y / pulse_width + time * pulse_rate);
  color.g *= 1.0 + pulse_intensity * sin(pixel.y / pulse_width + time * pulse_rate);

  return vec4<f32>(color, 1.0);
}
