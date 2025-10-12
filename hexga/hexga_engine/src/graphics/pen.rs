use super::*;


singleton_access!(
    pub Pen,
    GpuPen,
    { Gpu::try_as_ref().map(|gpu| &gpu.pen) },
    { Gpu::try_as_mut().map(|gpu| &mut gpu.pen) }
);

#[derive(Debug)]
pub struct GpuPen
{
    pub(crate) params: NonEmptyStack<DrawParam>,
    pub(crate) default_param : DrawParam,

    pub(crate) big_mesh  : MeshBuilder,
    pub(crate) draw_calls: NonEmptyStack<DrawCall>,
}

impl GpuPen
{
    pub fn new(param : DrawParam) -> Self
    {
        Self { params: NonEmptyStack::new(param), default_param: param, big_mesh: ___(), draw_calls: ___() }
    }
}


impl ScopedFlow for GpuPen
{
    fn begin_flow_draw(&mut self)
    {
        /*
        self.big_mesh.clear();
        self.draw_calls.clear();

        assert_eq!(self.params.len(), 1, "Forget to pop a camera");

        let dcall_param = &mut self.default_param;
        let scissor = param.window_size.to_rect();
        let viewport = scissor.cast_into();
        dcall_param.scissor = scissor;
        dcall_param.viewport = viewport;
        self.params.replace(*dcall_param);
        self.draw_calls.param = *dcall_param;
        */
    }

    fn end_flow_draw(&mut self)
    {
        /*
        self.update_last_draw_call();
        assert_eq!(self.params.len(), 1, "Forget to pop a camera");
        */
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DrawParam
{
    pub camera  : Camera,
    pub viewport: Rect2,
    pub viewport_min_depth: float,
    pub viewport_max_depth: float,
    pub scissor : Rect2P,
}
impl Default for DrawParam
{
    fn default() -> Self {
        Self { camera: ___(), viewport: ___(), viewport_min_depth: 0., viewport_max_depth: 1., scissor: ___() }
    }
}


#[derive(Clone, Debug)]
pub enum DrawGeometry
{
    ImmediateMode(DrawGeometryImmediate),
}
impl Default for DrawGeometry
{
    fn default() -> Self {
        Self::ImmediateMode(___())
    }
}

#[derive(Clone, Debug, Default)]
pub struct DrawGeometryImmediate
{
    pub(crate) vertices_begin: usize,
    pub(crate) vertices_len: usize,

    pub(crate) indices_begin: usize,
    pub(crate) indices_len: usize,
}

#[derive(Clone, Debug, Default)]
pub struct DrawCall
{
    pub(crate) geometry: DrawGeometry,
    pub(crate) param: DrawParam,
    // add texture here
}
impl Deref for DrawCall
{
    type Target=DrawParam;
    fn deref(&self) -> &Self::Target { &self.param }
}
impl DerefMut for DrawCall
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.param }
}

impl DrawCall
{
    pub fn is_geometry_empty(&self) -> bool
    {
        self.geometry.is_empty()
    }
}
impl DrawGeometry
{
    pub fn is_empty(&self) -> bool
    {
        match self
        {
            DrawGeometry::ImmediateMode(immediate) => immediate.is_empty(),
        }
    }
}
impl DrawGeometryImmediate
{
    pub fn is_empty(&self) -> bool
    {
        self.vertices_len == 0 || self.indices_len == 0
    }
}