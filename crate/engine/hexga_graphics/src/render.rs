use std::range::Range;

use super::*;

pub mod prelude
{
    pub use super::traits::*;
    pub use super::{
        DrawCall, DrawGeometry, DrawGeometryImmediate, ImmediateRender, ImmediateRenderBuilder,
    };
}

pub mod traits
{}

#[derive(Debug, Default)]
pub struct ImmediateRenderBuilder
{
    pub(crate) draw_call: NonEmptyStack<DrawCall>,
    pub(crate) params: NonEmptyStack<DrawParam>,
    pub(crate) big_mesh: MeshBuilder,
}



#[derive(Clone, Debug, Default)]
pub struct ImmediateRender
{
    pub draw_call: Vec<DrawCall>,
}

#[derive(Clone, Debug)]
pub enum DrawGeometry
{
    Immediate(DrawGeometryImmediate),
}

#[derive(Clone, Debug, Default)]
pub struct DrawGeometryImmediate
{
    pub vertices: Range<usize>,
    pub indices: Range<usize>,
}

/*
#[derive(Clone, Debug, Default)]
pub struct DrawGeometryImmediate
{
    pub vertices_begin: usize,
    pub vertices_len: usize,

    pub indices_begin: usize,
    pub indices_len: usize,
}
*/
/*
pub struct DrawGeometrySliceIndice
{
    pub begin: usize,
    pub len: usize,
}
*/

impl Default for DrawGeometry
{
    fn default() -> Self { Self::Immediate(___()) }
}

#[derive(Clone, Debug, Default)]
pub struct DrawCall
{
    pub geometry: DrawGeometry,
    pub param: DrawParam,
}
impl Deref for DrawCall
{
    type Target = DrawParam;
    fn deref(&self) -> &Self::Target { &self.param }
}
impl DerefMut for DrawCall
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.param }
}

#[derive(Clone, PartialEq, Debug)]
pub struct DrawParam
{
    pub camera: Camera,
    pub viewport: Rect2,
    pub viewport_min_depth: float,
    pub viewport_max_depth: float,
    pub scissor: Rect2i,
    pub texture: Option<Texture>,
}
impl Default for DrawParam
{
    fn default() -> Self
    {
        Self {
            camera: ___(),
            viewport: ___(),
            viewport_min_depth: 0.,
            viewport_max_depth: 1.,
            scissor: ___(),
            texture: ___(),
        }
    }
}
