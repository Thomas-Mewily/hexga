//! mainly based on miniquad
use std::{fmt::Debug, marker::PhantomData};

use crate::*;


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BufferType 
{
    VertexBuffer,
    IndexBuffer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BufferUsage {
    Immutable,
    Dynamic,
    Stream,
}

pub type BufferID = usize;



struct UntypedSlice<'a>
{
    data    : *const u8,
    layout  : BufferLayout,
    phantom : PhantomData<&'a ()>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferLayout
{
    pub len          : usize,
    pub element_size : usize,
}

pub struct BufferSource<'a>
{
    inner : BufferSourceInner<'a>,
}

impl<'a> Debug for BufferSourceInner<'a> 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("BufferSource") }
}

enum BufferSourceInner<'a>
{
    UntypedSlice(UntypedSlice<'a>),
    Empty(BufferLayout),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TextureAccess {
    /// Used as read-only from GPU
    Static,
    /// Can be written to from GPU
    RenderTarget,
}


#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum MipmapFilterMode {
    None,
    Linear,
    Nearest,
}

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum FilterMode {
    Linear,
    Nearest,
}

/// Sets the wrap parameter for texture.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TextureWrap {
    /// Samples at coord x + 1 map to coord x.
    Repeat,
    /// Samples at coord x + 1 map to coord 1 - x.
    Mirror,
    /// Samples at coord x + 1 map to coord 1.
    Clamp,
}

pub type TextureWrap2 = Vector2<TextureWrap>;

pub type RenderPass = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexFormat {
    /// One 32-bit wide float (equivalent to `f32`)
    Float1,
    /// Two 32-bit wide floats (equivalent to `[f32; 2]`)
    Float2,
    /// Three 32-bit wide floats (equivalent to `[f32; 3]`)
    Float3,
    /// Four 32-bit wide floats (equivalent to `[f32; 4]`)
    Float4,
    /// One unsigned 8-bit integer (equivalent to `u8`)
    Byte1,
    /// Two unsigned 8-bit integers (equivalent to `[u8; 2]`)
    Byte2,
    /// Three unsigned 8-bit integers (equivalent to `[u8; 3]`)
    Byte3,
    /// Four unsigned 8-bit integers (equivalent to `[u8; 4]`)
    Byte4,
    /// One unsigned 16-bit integer (equivalent to `u16`)
    Short1,
    /// Two unsigned 16-bit integers (equivalent to `[u16; 2]`)
    Short2,
    /// Tree unsigned 16-bit integers (equivalent to `[u16; 3]`)
    Short3,
    /// Four unsigned 16-bit integers (equivalent to `[u16; 4]`)
    Short4,
    /// One unsigned 32-bit integers (equivalent to `[u32; 1]`)
    Int1,
    /// Two unsigned 32-bit integers (equivalent to `[u32; 2]`)
    Int2,
    /// Three unsigned 32-bit integers (equivalent to `[u32; 3]`)
    Int3,
    /// Four unsigned 32-bit integers (equivalent to `[u32; 4]`)
    Int4,
    /// Four by four matrix of 32-bit floats
    Mat4,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VertexAttribute {
    pub name: String,
    pub format: VertexFormat,
    pub buffer_index: usize,
    /// This flag affects integer VertexFormats, Byte*, Short*, Int*
    /// Taking Byte4 as an example:
    /// On Metal, it might be received as either `float4` or `uint4`
    /// On OpenGl and `gl_pass_as_float = true` shaders should receive it as `vec4`
    /// With `gl_pass_as_float = false`, as `uvec4`
    ///
    /// Note that `uvec4` requires at least `150` glsl version
    /// Before setting `gl_pass_as_float` to false, better check `context.info().has_integer_attributes()` and double check that shaders are at least `150`
    pub gl_pass_as_float: bool,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct ShaderID(usize);

/// Define front- and back-facing polygons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrontFaceOrder {
    Clockwise,
    CounterClockwise,
}

/// Specify whether front- or back-facing polygons can be culled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CullFace {
    Nothing,
    Front,
    Back,
}

/* 
/// A pixel-wise comparison function.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Comparison {
    Never,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Equal,
    NotEqual,
    Always,
}
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PipelineParams 
{
    pub cull_face: CullFace,
    pub front_face_order: FrontFaceOrder,

    /* 
    pub depth_test: Comparison,
    pub depth_write: bool,
    pub depth_write_offset: Option<GpuVec2>,

    pub color_blend: Option<BlendState>,
    pub alpha_blend: Option<BlendState>,
    pub stencil_test: Option<StencilState>,
    */

    pub color_mask: ColorMask,
    pub primitive_type: PrimitiveType,
}
impl Default for PipelineParams
{
    fn default() -> Self {
        Self 
        { 
            cull_face: CullFace::Nothing, 
            front_face_order: FrontFaceOrder::CounterClockwise, 
            color_mask: Default::default(), 
            primitive_type: PrimitiveType::Triangles,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Triangles,
    Lines,
    Points,
}

type ColorMask = (bool, bool, bool, bool);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Pipeline(usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UniformDesc {
    pub name         : String,
    pub uniform_type : UniformType,
    pub nb           : usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UniformType {
    /// One 32-bit wide float (equivalent to `f32`)
    Float1,
    /// Two 32-bit wide floats (equivalent to `[f32; 2]`)
    Float2,
    /// Three 32-bit wide floats (equivalent to `[f32; 3]`)
    Float3,
    /// Four 32-bit wide floats (equivalent to `[f32; 4]`)
    Float4,
    /// One unsigned 32-bit integers (equivalent to `[u32; 1]`)
    Int1,
    /// Two unsigned 32-bit integers (equivalent to `[u32; 2]`)
    Int2,
    /// Three unsigned 32-bit integers (equivalent to `[u32; 3]`)
    Int3,
    /// Four unsigned 32-bit integers (equivalent to `[u32; 4]`)
    Int4,
    /// Four by four matrix of 32-bit floats
    Mat4,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UniformBlockLayout {
    pub uniforms: Vec<UniformDesc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShaderMeta {
    pub uniforms: UniformBlockLayout,
    pub images  : Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShaderSource 
{
    GLSL(ShaderSourceGLSL),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShaderSourceGLSL
{
    pub vertex: String,
    pub fragment: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferData
{
    buf_type : BufferType, 
    usage    : BufferUsage, 
}


// #proper_error
// miniquad render wrapper
pub trait Render
{
    fn new_buffer<'a>(&mut self, data: BufferData, source : BufferSource<'a>) -> Option<BufferID>;
    fn buffer_update(&mut self, dest: BufferID, source: &BufferSource) -> Result<(), ()>;
    fn delete_buffer(&mut self, id : BufferID);

    fn new_texture             (&mut self, data : &TextureData) -> Texture;
    fn texture_update          (&mut self, dest : Texture, source : &TextureData);
    fn texture_set_mag_filter  (&mut self, id : Texture, filter: FilterMode);
    fn texture_set_wrap        (&mut self, id : Texture, wrap : TextureWrap2);
    fn texture_generate_mipmaps(&mut self, id : Texture);
    fn texture_read_pixels     (&mut self, id : Texture, source : &mut TextureSource);
    fn texture_update_view     (&mut self, id : Texture, view : &mut TextureView);
    fn delete_texture          (&mut self, id : Texture);

    fn new_render_pass   (&mut self, texture : Texture, depth : Option<Texture>) -> RenderPass;
    fn delete_render_pass(&mut self, id: RenderPass);

    fn new_pipeline   (&mut self, data : &PipelineData) -> Pipeline;
    fn apply_pipeline (&mut self, pipeline: Pipeline);
    fn delete_pipeline(&mut self, pipeline: Pipeline);

    fn new_shader   (&mut self, data : &ShaderData) -> ShaderID;
    fn delete_shader(&mut self, program: ShaderID);

    fn apply_viewport(&mut self, rect : GpuMat2);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShaderData
{
    source: ShaderSource,
    meta  : ShaderMeta,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PipelineData
{
    pub buffer_layout: Vec<BufferLayout>, 
    pub attributes: Vec<VertexAttribute>,
    pub shader: ShaderID,
    pub params: PipelineParams,
}