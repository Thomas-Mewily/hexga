use crate::*;
use miniquad::{Pipeline,Bindings,BufferId,TextureId,BufferType,BufferUsage,BufferSource,Backend,ShaderSource,VertexAttribute,PipelineParams,BufferLayout,VertexFormat,ShaderMeta,UniformBlockLayout};

pub struct Pen;

impl Deref for Pen
{
    type Target=ContextPen;
    fn deref(&self) -> &Self::Target { &ctx().pen }
}
impl DerefMut for Pen
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut ctx().pen }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GpuVertex 
{
    pub color : GpuColor,
    pub pos   : GpuVec3,
    pub uv    : GpuVec2,
}
impl Default for GpuVertex
{
    fn default() -> Self {
        Self::new()
    }
}
impl GpuVertex
{
    pub const fn new() -> Self { Self { pos: GpuVec3::ZERO, uv: GpuVec2::ZERO, color: GpuColor::WHITE } }
    pub const fn with_pos(mut self, pos : GpuVec3) -> Self { self.pos = pos; self }
    pub const fn with_uv(mut self, uv : GpuVec2) -> Self { self.uv = uv; self }
    pub fn with_color(mut self, color : impl IColor) -> Self { self.color = color.to_gpu_color(); self }
}

pub type GpuVertexIdx = u16;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PenConfig
{
    pub max_vertex: usize,
    pub max_index : usize
}
impl Default for PenConfig
{
    fn default() -> Self 
    {
        Self { max_vertex: Self::DEFAULT_VERTEX_CAPACITY, max_index: Self::DEFAULT_INDEX_CAPACITY }
    }
}
impl PenConfig
{
    // arbitrary constant value based on macroquad
    const DEFAULT_VERTEX_CAPACITY : usize = 8000;
    const DEFAULT_INDEX_CAPACITY  : usize = 4000;
}

pub struct GpuMesh 
{
    pub vertexs: Vec<GpuVertex>,
    pub indices: Vec<GpuVertexIdx>,
    //pub texture: Option<Texture2D>,
}


#[derive(Debug)]
pub struct ContextPen
{
    pipeline: Pipeline,
    bindings: Bindings,

    vertex_buffer: BufferId,
    index_buffer : BufferId,

    white_pixel : TextureId,
    
    batch_vertex_buffer: Vec<GpuVertex>,
    batch_index_buffer: Vec<GpuVertexIdx>,
    
    param : PenConfig,
}

impl ContextPen
{
    pub(crate) fn new(render : &mut RenderBackEnd, param : PenConfig) -> Self 
    {
        let PenConfig{ max_vertex, max_index } = param;
        let white_pixel = render.new_texture_from_rgba8(1, 1, &[255, 255, 255, 255]);

        let vertex_buffer = render.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::empty::<GpuVertex>(max_vertex),
        );
        
        let index_buffer = render.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::empty::<GpuVertexIdx>(max_index),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![white_pixel],
        };

        let shader = render
        .new_shader(
            match render.info().backend {
                Backend::OpenGl => ShaderSource::Glsl {
                    vertex: shader::VERTEX,
                    fragment: shader::FRAGMENT,
                },
                Backend::Metal => ShaderSource::Msl {
                    program: shader::METAL,
                },
            },
            shader::meta(),
        )
        .unwrap();

        let pipeline = render.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float3),
                VertexAttribute::new("in_uv", VertexFormat::Float2),
                VertexAttribute {

                    gl_pass_as_float: false,
                    ..VertexAttribute::new("in_color", VertexFormat::Byte4)
                },
            ],
            shader,
            PipelineParams::default(),
        );

        Self 
        { 
            pipeline, 
            bindings,
            vertex_buffer,
            index_buffer, 
            white_pixel,
            param,
            batch_vertex_buffer : Vec::with_capacity(param.max_vertex),
            batch_index_buffer  : Vec::with_capacity(param.max_index),
        } 
    }
}

// Todo : use this trait on Pen.
// and don't impl Deref/DerefMut on Pen
pub trait IPen
{

} 

impl ContextPen
{
    pub fn begin_pass(&mut self)
    {
        let r = render();
        r.begin_default_pass(Default::default());
        r.apply_pipeline(&self.pipeline);
    }

    pub fn end_pass(&mut self)
    {
        render().end_render_pass();
    }

    pub fn commit_frame(&mut self)
    {
        let r = render();
        r.apply_bindings(&self.bindings);
        r.buffer_update(self.vertex_buffer, BufferSource::slice(&self.batch_vertex_buffer));
        r.buffer_update(self.index_buffer, BufferSource::slice(&self.batch_index_buffer));
        r.draw(0, self.batch_index_buffer.len() as _, 1);
    }
}

impl ContextPen
{
    pub fn geometry(&mut self, vertexs : &[GpuVertex], indexs: &[GpuVertexIdx]) -> &mut Self
    {
        let PenConfig { max_vertex, max_index } = self.param;
        if vertexs.len() >= max_vertex || indexs.len() >= max_index {
            warn!("geometry() exceeded max drawcall size, clamping");
        }

        let vertexs = &vertexs[0..max_vertex.min(vertexs.len())];
        let indexs = &indexs[0..max_index.min(indexs.len())];

        let vertex_offset = self.batch_vertex_buffer.len();
        self.batch_vertex_buffer.extend(vertexs);
        self.batch_index_buffer.extend(indexs.iter().map(|x| *x + vertex_offset as GpuVertexIdx));

        let r = render();


        self
    }

    pub fn polygons(&mut self, poly : &[GpuVertex])
    {
        
    }
}





pub(crate) mod shader 
{
    use super::*;
    pub const VERTEX: &str = r#"
    #version 150
    in vec3 in_pos;       // Position (x, y, z)
    in vec2 in_uv;        // Texture coordinates (u, v)
    in lowp uvec4 in_color; // Color (r, g, b, a)

    out vec2 uv;          // Pass UV to fragment shader
    out lowp vec4 color;  // Pass color to fragment shader

    void main() {
        gl_Position = vec4(in_pos, 1.0); // Set vertex position
        uv = in_uv;                      // Pass UV coordinates
        color = vec4(in_color) / 255.0;  // Normalize color to [0, 1]
    }
    "#;

    pub const FRAGMENT: &str = r#"
    #version 150
    in vec2 uv;           // UV coordinates from vertex shader
    in lowp vec4 color;   // Color from vertex shader

    out vec4 frag_color;  // Output color

    void main() {
        frag_color = color; // Use the interpolated color
    }
    "#;

    pub const METAL: &str = r#"
    #include <metal_stdlib>
    using namespace metal;

    struct VertexIn {
        float3 pos [[attribute(0)]];
        float2 uv [[attribute(1)]];
        uchar4 color [[attribute(2)]];
    };

    struct VertexOut {
        float4 position [[position]];
        float2 uv;
        float4 color;
    };

    vertex VertexOut vertex_main(VertexIn in [[stage_in]]) {
        VertexOut out;
        out.position = float4(in.pos, 1.0); // Set vertex position
        out.uv = in.uv;                     // Pass UV coordinates
        out.color = float4(in.color) / 255.0; // Normalize color to [0, 1]
        return out;
    }

    fragment float4 fragment_main(
        VertexOut in [[stage_in]]
    ) {
        return in.color; // Use the interpolated color
    }
    "#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta 
        {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout { 
                uniforms: vec![],
            },
        }
    }
}
