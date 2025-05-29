use super::*;

/// Not RAII. Manual deletion of pipeline is required using [RenderBackend::delete_pipeline].
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct RawPipelineID { pub index : usize }


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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PipelineParam
{
    pub cull_face: CullFace,
    pub front_face_order: FrontFaceOrder,

    pub depth_test: DepthComparison,
    pub depth_write: bool,
    pub depth_write_offset: Option<Vec2>,

    pub color_blend: Option<BlendState>,
    pub alpha_blend: Option<BlendState>,
    pub stencil_test: Option<StencilState>,

    pub color_mask: ColorMask,
    pub primitive_type: PrimitiveType,
}
impl Default for PipelineParam
{
    fn default() -> Self {
        Self
        {
            cull_face: CullFace::Nothing,
            front_face_order: FrontFaceOrder::CounterClockwise,
            color_mask: ColorMask::new(true, true, true, true),
            primitive_type: PrimitiveType::Triangles,
            depth_test: DepthComparison::Always, // no depth test,
            depth_write: false,
            depth_write_offset: None,
            color_blend: None,
            alpha_blend: None,
            stencil_test: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Triangles,
    Lines,
    Points,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PipelineData
{
    pub buffer_layout: Vec<VertexBufferLayout>,
    pub attributes: Vec<VertexAttribute>,
    pub shader: RawShaderID,
    pub param : PipelineParam,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DepthComparison {
    Never,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Equal,
    NotEqual,
    Always,
}
