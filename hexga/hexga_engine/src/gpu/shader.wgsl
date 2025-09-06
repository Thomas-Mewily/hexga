
struct Camera {
    view_proj: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: Camera;


struct VertexInput {
    @location(0) position: vec3f,
    @location(1) color: vec3f,
};

struct VertexOutput {
   @builtin(position) pos: vec4f,
   @location(0) color: vec3f,
}

struct InstanceInput {
    @location(5) model_matrix_col_0: vec4f,
    @location(6) model_matrix_col_1: vec4f,
    @location(7) model_matrix_col_2: vec4f,
    @location(8) model_matrix_col_3: vec4f,
}

struct FragmentInput {
   @builtin(position) pos: vec4f,
   @location(0) color: vec3f,
}

@vertex
fn vs_main(vertex_in: VertexInput, instance: InstanceInput) -> VertexOutput 
{
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_row_0,
        instance.model_matrix_row_1,
        instance.model_matrix_row_2,
        instance.model_matrix_row_3,
    );
    var out: VertexOutput;
    out.pos = vec4f(vertex_in.position, 1.0);
    out.color = vertex_in.color;
    return out;
}

@fragment
fn fs_main(fragment_in: FragmentInput) -> @location(0) vec4f {
    return vec4f(fragment_in.color, 1.0);
}

//@fragment
//fn fs_main(@location(0) color: vec3f) -> @location(0) vec4f {
//    return vec4f(color, 1.0);
//}