

struct VertexInput {
    @location(0) position: vec3f,
    @location(1) color: vec3f,
};

struct VertexOutput {
   @builtin(position) pos: vec4f,
   @location(0) color: vec3f,
}

struct FragmentInput {
   @builtin(position) pos: vec4f,
   @location(0) color: vec3f,
}

@vertex
fn vs_main(vertex_in: VertexInput) -> VertexOutput 
{
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