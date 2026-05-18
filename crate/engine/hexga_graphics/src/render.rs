use std::range::Range;

use super::*;

pub mod prelude
{
    pub use super::traits::*;
    pub use super::{DrawCall, DrawGeometry, DrawGeometryImmediate, ImmediateRender, ImmediateRenderBuilder};
}

pub mod traits
{}

#[derive(Debug, Default)]
pub struct ImmediateRenderBuilder
{
    pub(crate) draw_call: NonEmptyStack<DrawCall>,
    pub(crate) big_mesh: MeshBuilder,
    pub(crate) params: NonEmptyStack<DrawParam>,
}

pub trait RenderImmediate: BuilderMesh
{
    fn draw_param(&self) -> DrawParam;
    fn set_draw_param(&mut self, param: DrawParam)
    {
        self.update_draw_param(|p| {
            *p = param;
        });
    }
    fn update_draw_param<F>(&mut self, f: F)
    where
        F: FnOnce(&mut DrawParam);
}

/*
impl Builder for ImmediateRenderBuilder
{
    type Output = ImmediateRender;

    fn build(&self) -> Self::Output {

    }

    fn build_in(&self, dest: &mut Self::Output) {
        dest.draw_call.clear();
        dest.draw_call = self.draw_call
    }
}
*/
/*
impl RenderImmediate for ImmediateRenderBuilder
{

}

impl RenderImmediate for ImmediateRenderBuilder
{

}*/

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
    pub viewport_depth: Range<float>,
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
            viewport_depth: Range { start: 0., end: 1. },
            scissor: ___(),
            texture: ___(),
        }
    }
}
