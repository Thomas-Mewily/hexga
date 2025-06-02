use crate::*;

/*
pub mod prelude
{
    pub use super::{Pen,PenParam};
    pub(crate) use super::ContextPen;
}

pub struct Pen;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vertex
{
    pub pos   : GpuVec3,
    pub uv    : GpuVec2,
    pub color : GpuColor,
}
impl Default for Vertex
{
    fn default() -> Self {
        Self::new()
    }
}
impl Vertex
{
    pub const fn new() -> Self { Self { pos: GpuVec3::ZERO, uv: GpuVec2::ZERO, color: GpuColor::WHITE } }

    #[must_use]
    pub fn with_pos<P>(mut self, pos : P) -> Self where P : CastIntoComposite<GpuFloat,Output = GpuVec3> { self.set_pos(pos); self }
    #[must_use]
    pub fn with_uv<P>(mut self, uv : P) -> Self where P : CastIntoComposite<GpuFloat,Output = GpuVec2> { self.set_uv(uv); self }
    #[must_use]
    pub fn with_color<T, C>(mut self, color : C) -> Self where C : ToGpuColor<T> { self.set_color(color); self }

    pub fn set_pos<P>(&mut self, pos : P) -> &mut Self where P : CastIntoComposite<GpuFloat,Output = GpuVec3> { self.pos = pos.cast_into_composite(); self }
    pub fn set_uv<P>(&mut self, uv : P) -> &mut Self where P : CastIntoComposite<GpuFloat,Output = GpuVec2> { self.uv = uv.cast_into_composite(); self }
    pub fn set_color<T, C>(&mut self, color : C) -> &mut Self where C : ToGpuColor<T>  { self.color = color.to_gpu_color(); self }
}

pub type VertexIdx = u16;
pub type VertexTriangleIdx = [VertexIdx;3];

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct PenParam
{
    pub max_vertex: usize,
    pub max_index : usize
}
impl Default for PenParam
{
    fn default() -> Self
    {
        Self { max_vertex: Self::DEFAULT_VERTEX_CAPACITY, max_index: Self::DEFAULT_INDEX_CAPACITY }
    }
}
impl PenParam
{
    // arbitrary constant value copied from on macroquad
    const DEFAULT_VERTEX_CAPACITY : usize = 8000;
    const DEFAULT_INDEX_CAPACITY  : usize = 3600;

    pub fn align(&mut self) -> &mut Self { *self = self.aligned(); self }
    pub fn aligned(mut self) -> Self
    {
        self.max_index = self.max_index / 3 * 3;
        self
    }
}

pub struct GpuMesh
{
    pub vertexs: Vec<Vertex>,
    pub indices: Vec<VertexIdx>,
    //pub texture: Option<Texture2D>,
}


#[derive(Debug)]
pub(crate) struct ContextPen
{
    /*
    pipeline: PipelineID,
    bindings: Bindings,

    vertex_buffer_id : BufferId,
    index_buffer_id : BufferId,

    white_pixel : TextureId,

    prev_vertex_index : usize,
    vertex_stack : NonEmptyStack<Vertex>,

    vertex: Vec<Vertex>,
    index : Vec<VertexIdx>,
    */

    param : PenParam,
}

impl Drop for ContextPen
{
    fn drop(&mut self)
    {
        //render().delete_texture(self.white_pixel);
    }
}

/*
impl ContextPen
{
    pub(crate) fn new(render : &mut dyn ContextMultiMedia, mut param : PenParam) -> Self
    {
        param.align();
        let PenParam{ max_vertex, max_index } = param;
        let white_pixel = render.new_texture_from_rgba8(1, 1, &[255, 255, 255, 255]);

        let vertex_buffer = render.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::empty::<Vertex>(max_vertex),
        );

        let index_buffer = render.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::empty::<VertexIdx>(max_index),
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
            PipelineParam::default(),
        );

        Self
        {
            pipeline,
            bindings,
            vertex_buffer_id: vertex_buffer,
            index_buffer_id: index_buffer,
            white_pixel,
            param,
            prev_vertex_index : 0,
            vertex_stack : ___(),
            vertex : Vec::with_capacity(param.max_vertex),
            index  : Vec::with_capacity(param.max_index),
        }
    }
}

// Todo : use this trait on Pen.
// and don't impl Deref/DerefMut on Pen
pub trait IPen
{

}


macro_rules! impl_pen_pos {
    ($float_type: ty) =>
    {
        impl Position<$float_type,3> for ContextPen
        {
            fn pos(&self) -> Vector<$float_type,3>
            {
                let pos = self.last_vertex().pos;
                CastIntoComposite::<$float_type>::cast_into_composite(pos)
            }

            fn set_pos(&mut self, pos : Vector<$float_type,3>) -> &mut Self
            {
                self.last_vertex_mut().pos = CastIntoComposite::<GpuFloat>::cast_into_composite(pos);
                self
            }
        }
        impl Position<$float_type,2> for ContextPen
        {
            fn pos(&self) -> Vector<$float_type,2>
            {
                let pos = self.last_vertex().pos.to_vec2();
                CastIntoComposite::<$float_type>::cast_into_composite(pos)
            }

            fn set_pos(&mut self, pos : Vector<$float_type,2>) -> &mut Self
            {
                self.last_vertex_mut().pos = CastIntoComposite::<GpuFloat>::cast_into_composite(pos).with_z(0.);
                self
            }
        }

    };
}
map_on_float!(impl_pen_pos);


impl ContextPen
{
    pub fn begin_draw(&mut self)
    {

    }
    pub fn begin_pass(&mut self)
    {
        self.vertex.clear();
        self.index.clear();
        let r = render();
        r.begin_default_pass(Default::default());
        r.apply_pipeline(&self.pipeline);
    }

    pub fn end_pass(&mut self)
    {
        let r = render();
        r.apply_bindings(&self.bindings);
        r.buffer_update(self.vertex_buffer_id, BufferSource::slice(&self.vertex));
        r.buffer_update(self.index_buffer_id, BufferSource::slice(&self.index));
        r.draw(0, self.index.len() as _, 1);
        render().end_render_pass();
    }

    pub fn end_draw(&mut self)
    {

    }
}

impl ContextPen
{
    /// Commit the contextual current vertex to the Pen
    pub fn down(&mut self) -> &mut Self { self.commit_vertex(); self }

    /// Commit the contextual current vertex to the Pen and return it's Idx
    pub fn commit_vertex(&mut self) -> VertexIdx { self.push_vertex(*self.vertex_stack) }

    /// Commit the vertex to the Pen
    pub fn push_vertex(&mut self, vertex : Vertex) -> VertexIdx
    {
        let idx = self.vertex.len() as VertexIdx;
        self.vertex.push(vertex);
        idx
    }

    pub fn pos2<P>(&mut self, pos : P) -> &mut Self where P : CastIntoComposite<GpuFloat,Output = GpuVec2>
    {
        self.pos3(pos.cast_into_composite().with_z(0.))
    }
    pub fn pos3<P>(&mut self, pos : P) -> &mut Self where P : CastIntoComposite<GpuFloat,Output = GpuVec3>
    {
        self.last_vertex_mut().set_pos(pos);
        self
    }

    pub fn uv<P>(&mut self, uv : P) -> &mut Self where P : CastIntoComposite<GpuFloat,Output = GpuVec2>
    {
        self.last_vertex_mut().set_uv(uv);
        self
    }

    pub fn color<C>(&mut self, color : C) -> &mut Self where C : IColor
    {
        self.last_vertex_mut().set_color(color);
        self
    }

    pub fn vertex(&mut self, vertex : Vertex) -> &mut Self { *self.last_vertex_mut() = vertex; self }

    pub(crate) fn last_vertex(&self) -> &Vertex { &self.vertex_stack }
    pub(crate) fn last_vertex_mut(&mut self) -> &mut Vertex { &mut self.vertex_stack }








    /// Make a triangle with the last 3 vertex
    pub fn make_triangle(&mut self)
    {
        let len = self.vertex.len();
        self.index_triangle(
    [
            (len - 3) as VertexIdx,
            (len - 2) as VertexIdx,
            (len - 1) as VertexIdx,
          ]
        );
    }

    //pub fn index(&self) -> [GpuVertexIdx] { self.index_batch_buffer }

    pub fn begin_vertex(&mut self) { self.prev_vertex_index = self.vertex.len(); }
    pub(crate) fn vertex_index(&self) -> VertexIdx { (self.vertex.len() - 1) as VertexIdx }
    pub(crate) fn vertex_len(&self) -> VertexIdx { self.vertex.len() as VertexIdx }


    /// Also call `begin_vertexs()`
    pub fn index_triangle(&mut self, index: VertexTriangleIdx)
    {
        self.index_triangle_and(index);
        self.begin_vertex();
    }
    /// Don't call `begin_vertexs()`
    pub fn index_triangle_and(&mut self, index: VertexTriangleIdx) -> &mut Self
    {
        self.index.extend_from_slice(&index);
        self
    }

    /// Also call `begin_vertexs()`
    pub fn index_triangles<I>(&mut self, triangle_index: I) where I : IntoIterator<Item = VertexTriangleIdx>
    {
        self.index_triangles_and(triangle_index);
        self.begin_vertex();
    }
    /// Don't call `begin_vertexs()`
    pub fn index_triangles_and<I>(&mut self, triangle_index: I) -> &mut Self where I : IntoIterator<Item = VertexTriangleIdx>
    {
        for triangle in triangle_index.into_iter()
        {
            self.index_triangle_and(triangle);
        }
        self
    }

    /// Also call `begin_vertexs()`
    pub fn make_convex_poly(&mut self)
    {
        self.make_convex_poly_and();
        self.begin_vertex();
    }
    /// Don't call `begin_vertexs()`
    pub fn make_convex_poly_and(&mut self) -> &mut Self
    {
        let len = self.vertex.len() - self.prev_vertex_index;
        if len <= 2 { return self; }

        for i in 1..(len - 1)
        {
            self.index_triangle_and
            (
            [
                    (self.prev_vertex_index        ) as VertexIdx,
                    (self.prev_vertex_index + i    ) as VertexIdx,
                    (self.prev_vertex_index + i + 1) as VertexIdx
                ]
            );
        }

        self
    }

    pub fn geometry<V,I>(&mut self, vertex : V, index: I)
        where
        V : IntoIterator<Item=Vertex>, V::IntoIter : ExactSizeIterator,
        I : IntoIterator<Item=VertexTriangleIdx>, I::IntoIter : ExactSizeIterator,
    {
        self.geometry_and(vertex, index);
        self.begin_vertex();
    }

    pub fn geometry_and<V,I>(&mut self, vertex : V, index: I) -> &mut Self
        where
        V : IntoIterator<Item=Vertex>, V::IntoIter : ExactSizeIterator,
        I : IntoIterator<Item=VertexTriangleIdx>, I::IntoIter : ExactSizeIterator,
    {
        let vertex = vertex.into_iter();
        let index = index.into_iter();

        /*
        let PenConfig { max_vertex, max_index } = self.param;
        if vertex.len() >= max_vertex || index.len() >= max_index {
            warn!("geometry() exceeded max drawcall size, clamping");
        }
        */
        //let vertex_len = vertex.len().min(self.batch_vertex_buffer.capacity() - self.batch_vertex_buffer.len());
        //let indexs_len = index.len().min(self.batch_index_buffer.capacity() - self.batch_vertex_buffer.len());

        let vertex_offset = self.vertex_len();
        self.vertex.extend(vertex);
        self.index.extend(index.map(|x| x.map(|v| v + vertex_offset)).flatten());
        //self.batch_vertex_buffer.extend(vertex.take(vertex_len));
        //self.batch_index_buffer.extend(index.map(|x| x + vertex_offset as GpuVertexIdx).take(indexs_len));

        self
    }


    /*
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

        /*
        let PenConfig { max_vertex, max_index } = self.param;
        if vertex.len() >= max_vertex || index.len() >= max_index {
            warn!("geometry() exceeded max drawcall size, clamping");
        }
        */
        //let vertex_len = vertex.len().min(self.batch_vertex_buffer.capacity() - self.batch_vertex_buffer.len());
        //let indexs_len = index.len().min(self.batch_index_buffer.capacity() - self.batch_vertex_buffer.len());

        let vertex_offset = self.vertex.len();
        self.index.extend(index.map(|x| x + vertex_offset as GpuVertexIdx));
        self.vertex.extend(vertex);
        //self.batch_vertex_buffer.extend(vertex.take(vertex_len));
        //self.batch_index_buffer.extend(index.map(|x| x + vertex_offset as GpuVertexIdx).take(indexs_len));

        self
    }
    */
}


impl ContextPen
{
    pub fn circle<I>(&mut self, radius : float, nb_point : I)
        where Angle : RangeDefaultSampleExtension<I>,
        Range<Angle> : RangeSampleExtension<I,Item = Angle>
    {
        self.circle_and(radius, nb_point);
        self.begin_vertex();
    }

    pub fn circle_and<I>(&mut self, radius : float, nb_point : I)
        where Angle : RangeDefaultSampleExtension<I>,
        Range<Angle> : RangeSampleExtension<I,Item = Angle>
    {
        let pos : Vec2 = self.pos();
        for angle in Angle::sample(nb_point)
        {

            Pen.set_pos(pos + angle.to_vec2_normalized()  * radius).down();
        }
        Pen.make_convex_poly_and();
    }

    pub fn ellipse<I>(&mut self, radius : Vec2, nb_point : I)
        where Angle : RangeDefaultSampleExtension<I>,
        Range<Angle> : RangeSampleExtension<I,Item = Angle>
    {
        self.ellipse_and(radius, nb_point);
        self.begin_vertex();
    }

    pub fn ellipse_and<I>(&mut self, radius : Vec2, nb_point : I)
        where Angle : RangeDefaultSampleExtension<I>,
        Range<Angle> : RangeSampleExtension<I,Item = Angle>
    {
        let pos : Vec2 = self.pos();
        for angle in Angle::sample(nb_point)
        {

            Pen.set_pos(pos + angle.to_vec2_normalized()  * radius).down();
        }
        Pen.make_convex_poly_and();
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
*/
*/