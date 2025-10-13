use super::*;


#[derive(Debug)]
pub struct GpuRender
{
    pub(crate) params: NonEmptyStack<DrawParam>,
    pub(crate) default_param : DrawParam,

    pub(crate) big_mesh  : MeshBuilder,
    pub(crate) draw_calls: NonEmptyStack<DrawCall>,
}

impl GpuRender
{
    pub fn new(param : DrawParam) -> Self
    {
        Self { params: NonEmptyStack::new(param), default_param: param, big_mesh: ___(), draw_calls: ___() }
    }
}

impl GpuRender
{
        pub(crate) fn update_last_draw_call(&mut self)
    {
        let mesh = &mut self.big_mesh;

        let immediate_mode = match &mut self.draw_calls.last_mut().geometry
        {
            DrawGeometry::Immediate(immediate) =>
            {
                immediate.indices_len =  mesh.nb_index() - immediate.indices_begin;
                immediate.vertices_len =  mesh.nb_vertex() - immediate.vertices_begin;
            }
        };
    }


    pub fn max_viewport(&self) -> Rect2 { self.default_param.viewport }
    pub fn max_scissor(&self) -> Rect2i { self.default_param.scissor }
    pub fn viewport(&self) -> Rect2 { self.params.viewport }
    pub fn set_viewport(&mut self, viewport: Rect2) -> &mut Self
    {
        self.param_map(|p| p.viewport = viewport);
        self
    }


    pub fn push_param(&mut self)
    {
        self.params.push(self.draw_calls.param);
    }
    pub fn pop_param(&mut self)
    {
        let param = self.params.pop().expect("forget to push param");
        self.set_param(param);
    }
    pub fn param_map<F>(&mut self, f: F) where F: FnOnce(&mut DrawParam)
    {
        f(&mut self.params);
        self.set_param(*self.params);
    }
    pub fn param(&self) -> DrawParam { *self.params }

    pub fn set_param(&mut self, param: DrawParam)
    {
        if self.draw_calls.param == param { return; }

        self.update_last_draw_call();

        if !self.draw_calls.is_geometry_empty()
        {
            let mut draw_call = self.draw_calls.last().clone();
            self.draw_calls.geometry = DrawGeometry::Immediate(DrawGeometryImmediate
                {
                    vertices_begin: self.big_mesh.nb_index(),
                    vertices_len: 0,
                    indices_begin: self.big_mesh.nb_vertex(),
                    indices_len: 0
                }
            );
            self.draw_calls.push(draw_call);
        }
        self.draw_calls.param = param;
    }

    pub(crate) fn apply_cam(&mut self)
    {
        let c = self.camera();
        self.param_map(|p| { p.camera = c;});
    }
}
impl IMeshBuilder for GpuRender
{
    fn geometry(&mut self, vertex: impl IntoIterator<Item = Vertex>, index: impl IntoIterator<Item = VertexIndex>) {

        self.big_mesh.geometry(vertex, index);
    }
}

impl ICamera for GpuRender
{
    fn have_depth(&self) -> bool { self.params.camera.have_depth() }
}
impl GetPosition for GpuRender
{
    fn pos(&self) -> Vec3 { self.params.camera.pos() }
}
impl SetPosition for GpuRender
{
    fn set_pos(&mut self, pos : Vec3) -> &mut Self { self.params.camera.set_pos(pos); self.apply_cam(); self }
}
impl GetScale for GpuRender
{
    fn scale(&self) -> Vec3 { self.params.camera.scale() }
}
impl SetScale for GpuRender
{
    fn set_scale(&mut self, scale : Vec3) -> &mut Self { self.params.camera.set_scale(scale); self.apply_cam(); self }
}
impl RotateX for GpuRender
{
    fn rotate_x(&mut self, angle : Angle) -> &mut Self { self.params.camera.rotate_x(angle); self.apply_cam(); self }
}
impl RotateY for GpuRender
{
    fn rotate_y(&mut self, angle : Angle) -> &mut Self { self.params.camera.rotate_y(angle); self.apply_cam(); self }
}
impl RotateZ for GpuRender
{
    fn rotate_z(&mut self, angle : Angle) -> &mut Self { self.params.camera.rotate_z(angle); self.apply_cam(); self }
}
impl GetMatrix for GpuRender
{
    fn matrix(&self) -> Mat4 {
        self.params.camera.matrix()
    }
}
impl SetMatrix for GpuRender
{
    fn set_matrix(&mut self, matrix : Mat4) -> &mut Self {
        self.params.camera.set_matrix(matrix); self.apply_cam(); self
    }
}

impl ScopedFlow for GpuRender
{
    fn begin_flow_draw(&mut self)
    {
        self.big_mesh.clear();
        self.draw_calls.clear();

        assert_eq!(self.params.len(), 1, "Forget to pop a camera");

        let dcall_param = &mut self.default_param;
        let scissor = Window.size().to_rect();
        let viewport = scissor.cast_into();
        dcall_param.scissor = scissor;
        dcall_param.viewport = viewport;
        self.params.replace(*dcall_param);
        self.draw_calls.param = *dcall_param;
    }

    fn end_flow_draw(&mut self)
    {
        self.update_last_draw_call();
        assert_eq!(self.params.len(), 1, "Forget to pop a camera");
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DrawParam
{
    pub camera  : Camera,
    pub viewport: Rect2,
    pub viewport_min_depth: float,
    pub viewport_max_depth: float,
    pub scissor : Rect2i,
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
    Immediate(DrawGeometryImmediate),
}
impl Default for DrawGeometry
{
    fn default() -> Self {
        Self::Immediate(___())
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
            DrawGeometry::Immediate(immediate) => immediate.is_empty(),
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