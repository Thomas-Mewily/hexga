use super::*;

singleton_access!(
    pub Pen,
    ContextPen,
    { Gpu::try_as_ref().map(|gpu| &gpu.pen) },
    { Gpu::try_as_mut().map(|gpu| &mut gpu.pen) }
);


impl ScopedDraw for ContextPen
{
    fn begin_draw(&mut self, param: ScopedDrawParam) 
    {
        self.big_mesh.clear();
        self.draw_calls.clear();

        assert_eq!(self.params.len(), 1, "Forget to pop a camera");

        let mut draw_call = self.default_param;
        let clip = param.window_size.to_rect();
        let viewport = clip.cast_into();
        draw_call.clip = clip;
        draw_call.viewport = viewport;
        self.params.replace(draw_call);
    }

    fn end_draw(&mut self) 
    {
        self.update_last_draw_call();
        assert_eq!(self.params.len(), 1, "Forget to pop a camera");
    }
}

/* 
pub trait DynCamera : Futurable + ICamera + Debug {}
impl<T> DynCamera for T where T: Futurable + ICamera + Debug  {}
*/

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DrawCallParam
{
    pub camera  : Camera,
    pub viewport: Rect2,
    pub viewport_min_depth: float,
    pub viewport_max_depth: float,
    pub clip    : Rect2P,
}
impl Default for DrawCallParam
{
    fn default() -> Self {
        Self { camera: ___(), viewport: ___(), viewport_min_depth: 0., viewport_max_depth: 1., clip: ___() }
    }
}

#[derive(Debug, Clone)]
pub struct ContextPen
{
    pub(crate) params: NonEmptyStack<DrawCallParam>,
    pub(crate) default_param : DrawCallParam,

    pub(crate) big_mesh  : MeshBuilder,
    pub(crate) draw_calls: NonEmptyStack<DrawCall>
}

impl ContextPen
{
    pub fn new(param : DrawCallParam) -> Self 
    {
        Self { params: NonEmptyStack::new(param), default_param: param, big_mesh: ___(), draw_calls: ___() }
    }
}

impl ICamera for ContextPen
{
    fn have_depth(&self) -> bool { self.params.camera.have_depth() }
}
impl GetPosition for ContextPen
{
    fn pos(&self) -> Vec3 { self.params.camera.pos() }
}
impl SetPosition for ContextPen
{
    fn set_pos(&mut self, pos : Vec3) -> &mut Self { self.params.camera.set_pos(pos); self.apply_cam(); self }
}
impl GetScale for ContextPen
{
    fn scale(&self) -> Vec3 { self.params.camera.scale() }
}
impl SetScale for ContextPen
{
    fn set_scale(&mut self, scale : Vec3) -> &mut Self { self.params.camera.set_scale(scale); self.apply_cam(); self }
}
impl RotateX for ContextPen
{
    fn rotate_x(&mut self, angle : Angle) -> &mut Self { self.params.camera.rotate_x(angle); self.apply_cam(); self }
}
impl RotateY for ContextPen
{
    fn rotate_y(&mut self, angle : Angle) -> &mut Self { self.params.camera.rotate_y(angle); self.apply_cam(); self }
}
impl RotateZ for ContextPen
{
    fn rotate_z(&mut self, angle : Angle) -> &mut Self { self.params.camera.rotate_z(angle); self.apply_cam(); self }
}
impl GetMatrix for ContextPen
{
    fn matrix(&self) -> Mat4 {
        self.params.camera.matrix()
    }
}
impl SetMatrix for ContextPen
{
    fn set_matrix(&mut self, matrix : Mat4) -> &mut Self {
        self.params.camera.set_matrix(matrix); self.apply_cam(); self
    }
}


impl ContextPen
{
    pub(crate) fn update_last_draw_call(&mut self)
    {
        let mesh = &mut self.big_mesh;
        self.draw_calls.indices_len =  mesh.nb_index() - self.draw_calls.indices_begin;
        self.draw_calls.vertices_len =  mesh.nb_vertex() - self.draw_calls.vertices_begin;
    }

    pub fn set_param(&mut self, param: DrawCallParam)
    {
        if self.draw_calls.param == param { return; }

        self.update_last_draw_call();

        if !self.draw_calls.is_geometry_empty()
        {
            let mut draw_call = self.draw_calls.last().clone();
            draw_call.indices_begin = self.big_mesh.nb_index();
            draw_call.indices_len = 0;
            draw_call.vertices_begin = self.big_mesh.nb_vertex();
            draw_call.vertices_len = 0;
            self.draw_calls.push(draw_call);
        }
        self.draw_calls.param = param;
    }

    pub(crate) fn apply_cam(&mut self)
    {
        self.set_param(DrawCallParam { camera: self.params.camera.to_camera(), ..self.draw_calls.last().param });
    }
}
impl IMeshBuilder for ContextPen
{
    fn geometry(&mut self, vertex: impl IntoIterator<Item = Vertex<3>>, index: impl IntoIterator<Item = VertexIndex>) {
        
        self.big_mesh.geometry(vertex, index);
    }
}
/* 
impl Deref for ContextPen
{
    type Target=MeshBuilder;
    fn deref(&self) -> &Self::Target { &self.big_mesh }
}
impl DerefMut for ContextPen
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.big_mesh }
}
*/
/* 
#[derive(Clone, Default)]
pub struct GpuDrawCalls
{
    pub(crate) calls: Vec<GpuDrawCall>
}
impl GpuDrawCalls
{
    pub fn push(&mut self, call: GpuDrawCall) { self.calls.push(call); }
}
*/


#[derive(Clone, Debug, Default)]
pub struct DrawCall
{
    pub(crate) vertices_begin: usize,
    pub(crate) vertices_len: usize,

    pub(crate) indices_begin: usize,
    pub(crate) indices_len: usize,

    pub(crate) param: DrawCallParam,
    // add texture here
}
impl Deref for DrawCall
{
    type Target=DrawCallParam;
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
        self.vertices_len == 0 || self.indices_len == 0
    }
}

