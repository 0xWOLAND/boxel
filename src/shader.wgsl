// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
    view_position: vec4<f32>
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

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return index(1.0, 1.0);
}