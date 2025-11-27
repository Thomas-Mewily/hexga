struct CameraUniform {
    view_proj: mat4x4f,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var sample: sampler;

struct VertexInput {
    @location(0) pos: vec3f,
    @location(1) uv: vec2f,
    @location(2) color: vec3f,
};

struct VertexOutput {
   @builtin(position) pos: vec4f,
   @location(1) uv: vec2f,
   @location(2) color: vec3f,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput
{
    var out: VertexOutput;
    out.pos = camera.view_proj * vec4<f32>(model.pos, 1.0);
    out.color = model.color;
    out.uv = model.uv;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(texture, sample, in.uv);
    return tex_color * vec4<f32>(in.color, 1.0);
    //return vec4f(in.color, 1.0);
}