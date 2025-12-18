use super::*;

pub struct ImmediateRenderBuilder<P>
{
    pub draw_call : NonEmptyStack<DrawCall<P>>,
    pub params: NonEmptyStack<P>,
    //pub big_mesh
}

#[derive(Clone, Debug, Default)]
pub struct ImmediateRender<P>
{
    pub draw_call: Vec<DrawCall<P>>
}

#[derive(Clone, Debug)]
pub enum DrawGeometry
{
    Immediate(DrawGeometryImmediate),
}
#[derive(Clone, Debug, Default)]
pub struct DrawGeometryImmediate
{
    pub vertices_begin: usize,
    pub vertices_len: usize,

    pub indices_begin: usize,
    pub indices_len: usize,
}

impl Default for DrawGeometry
{
    fn default() -> Self {
        Self::Immediate(___())
    }
}

#[derive(Clone, Debug, Default)]
pub struct DrawCall<P>
{
    pub geometry: DrawGeometry,
    pub param: P,
}
impl<P> Deref for DrawCall<P>
{
    type Target=P;
    fn deref(&self) -> &Self::Target { &self.param }
}
impl<P> DerefMut for DrawCall<P>
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.param }
}


#[derive(Clone, PartialEq, Debug)]
pub struct DrawParam<T>
{
    pub camera  : Camera,
    pub viewport: Rect2,
    pub viewport_min_depth: float,
    pub viewport_max_depth: float,
    pub scissor : Rect2i,
    pub texture: T,
}
impl<T> Default for DrawParam<T> where T: Default
{
    fn default() -> Self {
        Self { camera: ___(), viewport: ___(), viewport_min_depth: 0., viewport_max_depth: 1., scissor: ___(), texture: ___() }
    }
}