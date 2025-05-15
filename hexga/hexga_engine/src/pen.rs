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
#[repr(C)]
pub struct GpuVertex 
{
    pub pos   : GpuVec3,
    pub uv    : GpuVec2,
    pub color : GpuColor,
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
    pub fn with_pos<P>(mut self, pos : P) -> Self where P : CastIntoComposite<GpuFloat,Output = GpuVec3> { self.pos = pos.cast_into_composite(); self }
    pub fn with_uv<P>(mut self, uv : P) -> Self where P : CastIntoComposite<GpuFloat,Output = GpuVec2> { self.uv = uv.cast_into_composite(); self }
    pub fn with_color<C>(mut self, color : C) -> Self where C : IColor { self.color = color.to_gpu_color(); self }
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
    // arbitrary constant value copied from on macroquad
    const DEFAULT_VERTEX_CAPACITY : usize = 8000;
    const DEFAULT_INDEX_CAPACITY  : usize = 4000;

    pub fn round(mut self) -> Self
    {
        self.max_index = self.max_index / 3 * 3;
        self
    }
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

    vertex_buffer_id : BufferId,
    index_buffer_id : BufferId,

    white_pixel : TextureId,
    
    prev_vertex_len : usize,
    vertex : NonEmptyStack<GpuVertex>,

    batch_vertex_buffer: Vec<GpuVertex>,
    batch_index_buffer : Vec<GpuVertexIdx>,
    
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
            vertex_buffer_id: vertex_buffer,
            index_buffer_id: index_buffer, 
            white_pixel,
            param,
            vertex : ___(),
            prev_vertex_len : 0,
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
    pub fn begin_draw(&mut self)
    {

    }
    pub fn begin_pass(&mut self)
    {
        self.batch_vertex_buffer.clear();
        self.batch_index_buffer.clear();
        let r = render();
        r.begin_default_pass(Default::default());
        r.apply_pipeline(&self.pipeline);
    }

    pub fn end_pass(&mut self)
    {
        let r = render();
        r.apply_bindings(&self.bindings);
        r.buffer_update(self.vertex_buffer_id, BufferSource::slice(&self.batch_vertex_buffer));
        r.buffer_update(self.index_buffer_id, BufferSource::slice(&self.batch_index_buffer));
        r.draw(0, self.batch_index_buffer.len() as _, 1);
        render().end_render_pass();
    }

    pub fn end_draw(&mut self)
    {
        
    }
}

impl ContextPen
{
    pub fn down(&mut self, vertex : GpuVertex) -> &mut Self
    {
        self.vertex(vertex);
        self
    }

    
    pub fn pos2<P>(&mut self, pos : P) -> &mut Self where P : CastIntoComposite<GpuFloat,Output = GpuVec2>
    {
        self.pos3(pos.cast_into_composite().with_z(0.))
    }
    pub fn pos3<P>(&mut self, pos : P) -> &mut Self where P : CastIntoComposite<GpuFloat,Output = GpuVec3>
    {
        self.last_vertex_mut().with_pos(pos);
        self
    }

    pub fn uv<P>(&mut self, uv : P) -> &mut Self where P : CastIntoComposite<GpuFloat,Output = GpuVec2>
    { 
        self.last_vertex_mut().with_uv(uv); 
        self
    }

    pub fn color<C>(&mut self, color : C) -> &mut Self where C : IColor
    { 
        self.last_vertex_mut().with_color(color); 
        self
    }

}

impl ContextPen
{
    // Add a new vertex at the current position
    pub fn vertex(&mut self, vertex : GpuVertex) -> GpuVertexIdx
    {
        let idx = self.batch_vertex_buffer.len() as GpuVertexIdx;
        self.batch_vertex_buffer.push(vertex);
        idx
    }

    pub fn last_vertex(&self) -> &GpuVertex { &self.vertex }
    pub fn last_vertex_mut(&mut self) -> &mut GpuVertex { &mut self.vertex }

    /// Make a triangle with last 3 vertex
    pub fn make_triangle(&mut self)
    {
        self.prev_vertex_len = self.batch_vertex_buffer.len();
        self.batch_index_buffer.push((self.prev_vertex_len - 3) as GpuVertexIdx);
        self.batch_index_buffer.push((self.prev_vertex_len - 2) as GpuVertexIdx);
        self.batch_index_buffer.push((self.prev_vertex_len - 1) as GpuVertexIdx);
    }
    /* 
    pub fn add_index(&mut self, index : GpuVertexIdx)
    {
        self.batch_index_buffer.push(index);
    }
    */

    pub fn triangle(&mut self, vertex : [GpuVertex;3]) -> &mut Self
    {
        self.static_geometry(vertex, [0, 1, 2])
    }


    pub fn draw_triangle_test(&mut self) -> &mut Self
    {
        self.static_geometry
        (
    [
                GpuVertex::new().with_pos(gpu_vec3(-0.3, -0.3, 0.)).with_color(Color::RED),
                GpuVertex::new().with_pos(gpu_vec3(0.3, -0.3, 0.)).with_color(Color::BLUE),
                GpuVertex::new().with_pos(gpu_vec3(0.0, 0.3, 0.)).with_color(Color::GREEN),
            ]
            , 
            [0, 1, 2]
        )
    }

    pub fn static_geometry<V,I>(&mut self, vertex : V, index: I) -> &mut Self
        where 
        V : IntoIterator<Item=GpuVertex>, V::IntoIter : ExactSizeIterator,
        I : IntoIterator<Item=GpuVertexIdx>, I::IntoIter : ExactSizeIterator,
    {
        let vertex = vertex.into_iter();
        let index = index.into_iter();

        let PenConfig { max_vertex, max_index } = self.param;
        if vertex.len() >= max_vertex || index.len() >= max_index {
            warn!("geometry() exceeded max drawcall size, clamping");
        }

        //let vertex_len = vertex.len().min(self.batch_vertex_buffer.capacity() - self.batch_vertex_buffer.len());
        //let indexs_len = index.len().min(self.batch_index_buffer.capacity() - self.batch_vertex_buffer.len());

        let vertex_offset = self.batch_vertex_buffer.len();
        self.batch_index_buffer.extend(index.map(|x| x + vertex_offset as GpuVertexIdx));

        self.batch_vertex_buffer.extend(vertex);
        //self.batch_vertex_buffer.extend(vertex.take(vertex_len));
        //self.batch_index_buffer.extend(index.map(|x| x + vertex_offset as GpuVertexIdx).take(indexs_len));

        self
    }

    pub fn polygons(&mut self, vertex : &[GpuVertex]) -> &mut Self
    {
        let index_count = (vertex.len() - 2) * 3;
        let mut indices = Vec::with_capacity(index_count);

        for i in 1..(vertex.len() - 1) {
            indices.push(0);
            indices.push(i as GpuVertexIdx);
            indices.push((i + 1) as GpuVertexIdx);
        }

        self.static_geometry(vertex.iter().copied(), indices)
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
