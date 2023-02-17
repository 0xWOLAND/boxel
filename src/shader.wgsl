// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
    view_position: vec4<f32>,
    view_up: vec4<f32>,
    view_target: vec4<f32>
};
@group(1) @binding(0) // 1.
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

const bin_size: f32 = 1.0;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0); // 2.
    return out;
}


// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

fn index(x: f32, y: f32) -> vec4<f32> {
    var pos = vec2<f32>(x, y);
    return textureSample(t_diffuse, s_diffuse, pos);
}

fn voxel_traversal(ray_start: vec3<f32>, ray_direction: vec3<f32>) -> bool {
    return true;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // var ans = voxel_traversal(vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0));
    // return index(in.tex_coords[0], in.tex_coords[1]);
    return camera.view_target;
}