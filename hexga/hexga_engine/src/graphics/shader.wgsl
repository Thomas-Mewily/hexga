struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3f,
    @location(1) color: vec3f,
};

struct VertexOutput {
   @builtin(position) pos: vec4f,
   @location(0) color: vec3f,
}



@vertex
fn vs_main(model: VertexInput) -> VertexOutput 
{
    var out: VertexOutput;
    out.pos = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.color = model.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}