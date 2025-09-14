use super::*;

singleton_access!(
    Pen,
    ContextPen,
    { Gpu::try_as_ref().map(|gpu| &gpu.pen) },
    { Gpu::try_as_mut().map(|gpu| &mut gpu.pen) }
);


impl ScopedDraw for ContextPen
{
    fn begin_draw(&mut self) 
    {
        self.big_mesh.clear();
        self.draw_calls.clear();

        assert_eq!(self.cameras.len(), 1, "Forget to pop a camera");
        self.cameras.replace(self.default_cam);
    }

    fn end_draw(&mut self) 
    {
        self.update_last_draw_call();
        assert_eq!(self.cameras.len(), 1, "Forget to pop a camera");
    }
}

/* 
pub trait DynCamera : Futurable + ICamera + Debug {}
impl<T> DynCamera for T where T: Futurable + ICamera + Debug  {}
*/

#[derive(Debug, Clone)]
pub struct ContextPen
{
    pub(crate) cameras: NonEmptyStack<Camera>,
    pub(crate) default_cam : Camera,

    pub(crate) big_mesh  : MeshBuilder,
    pub(crate) draw_calls: NonEmptyStack<GpuDrawCall>
}

impl ContextPen
{
    pub fn new(default_cam : Camera) -> Self 
    {
        Self { cameras: NonEmptyStack::new(default_cam), default_cam, big_mesh: ___(), draw_calls: ___() }
    }
}

impl ICamera for ContextPen
{
    fn have_depth(&self) -> bool { self.cameras.have_depth() }
    fn viewport(&self) -> Option<Rect2P> { self.cameras.viewport() }
}
impl GetPosition for ContextPen
{
    fn pos(&self) -> Vec3 { self.cameras.pos() }
}
impl SetPosition for ContextPen
{
    fn set_pos(&mut self, pos : Vec3) -> &mut Self { self.cameras.set_pos(pos); self.apply_cam(); self }
}
impl GetScale for ContextPen
{
    fn scale(&self) -> Vec3 { self.cameras.scale() }
}
impl SetScale for ContextPen
{
    fn set_scale(&mut self, scale : Vec3) -> &mut Self { self.cameras.set_scale(scale); self.apply_cam(); self }
}
impl RotateX for ContextPen
{
    fn rotate_x(&mut self, angle : Angle) -> &mut Self { self.cameras.rotate_x(angle); self.apply_cam(); self }
}
impl RotateY for ContextPen
{
    fn rotate_y(&mut self, angle : Angle) -> &mut Self { self.cameras.rotate_y(angle); self.apply_cam(); self }
}
impl RotateZ for ContextPen
{
    fn rotate_z(&mut self, angle : Angle) -> &mut Self { self.cameras.rotate_z(angle); self.apply_cam(); self }
}
impl GetMatrix for ContextPen
{
    fn matrix(&self) -> Mat4 {
        self.cameras.matrix()
    }
}
impl SetMatrix for ContextPen
{
    fn set_matrix(&mut self, matrix : Mat4) -> &mut Self {
        self.cameras.set_matrix(matrix); self
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

    pub(crate) fn new_draw_call_if_needed(&mut self, camera: Camera)
    {
        if self.draw_calls.camera == camera { return; }

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
        self.draw_calls.camera = camera;
    }
    pub(crate) fn apply_cam(&mut self)
    {
        self.new_draw_call_if_needed(self.cameras.to_camera());
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
pub struct GpuDrawCall
{
    pub(crate) vertices_begin: usize,
    pub(crate) vertices_len: usize,

    pub(crate) indices_begin: usize,
    pub(crate) indices_len: usize,

    pub(crate) camera: Camera,
    // add texture here
}
impl GpuDrawCall
{
    pub fn is_geometry_empty(&self) -> bool 
    {
        self.vertices_len == 0 || self.indices_len == 0
    }
}

