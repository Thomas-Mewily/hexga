use super::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Pipeline(usize);

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

    pub color_mask: RenderColorRGBAMask,
    pub primitive_type: PrimitiveType,
}
impl Default for PipelineParams
{
    fn default() -> Self {
        Self 
        { 
            cull_face: CullFace::Nothing, 
            front_face_order: FrontFaceOrder::CounterClockwise, 
            color_mask: (true, true, true, true), 
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
