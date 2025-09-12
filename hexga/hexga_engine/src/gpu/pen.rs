use super::*;

singleton!(
    Pen,
    Drawer,
    { Gpu::try_as_ref().map(|gpu| &gpu.draw) },
    { Gpu::try_as_mut().map(|gpu| &mut gpu.draw) }
);


impl ScopedDraw for Drawer
{
    fn begin_draw(&mut self) 
    {
        self.immediate.clear();
        self.draw_call.clear();
        self.camera.begin_draw();
    }

    fn end_draw(&mut self) 
    {
        self.flush();
        self.camera.end_draw();
    }
}

/* 
pub trait DynCamera : Futurable + ICamera + Debug {}
impl<T> DynCamera for T where T: Futurable + ICamera + Debug  {}
*/

pub struct Drawer
{
    pub(crate) camera   : CameraManager,
    pub(crate) immediate: MeshBuilder,
    pub(crate) draw_call: Vec<GpuDrawCalls>
}

impl ICamera for Drawer
{
    fn have_depth(&self) -> bool { self.camera.have_depth() }
    fn viewport(&self) -> Option<Rect2P> { self.camera.viewport() }
}
impl GetMatrix<float,4,4> for Drawer
{
    fn matrix(&self) -> Matrix<float,4,4> { self.camera.matrix() }
}
impl SetMatrix<float,4,4> for Drawer
{
    fn set_matrix(&mut self, matrix : Matrix<float,4,4>) -> &mut Self {
        self.camera.set_matrix(matrix); self
    }
}


impl Drawer
{
    pub fn camera(&self) -> &CameraManager { &self.camera }
    pub fn camera_mut(&mut self) -> &mut CameraManager { &mut self.camera }
    
    pub fn retrieve_current_mesh(&mut self) -> MeshBuilder
    {
        let mut empty = MeshBuilder::___();
        std::mem::swap(&mut empty, &mut self.immediate);
        empty
    } 

    /// Transform the immediate [MeshBuilder] in a [GpuDrawCall]
    pub fn flush(&mut self)
    {
        if self.immediate.is_empty() { return; }

        if self.draw_call.is_empty()
        {
            self.draw_call.push(___());
        }
        // Todo : Not opti, create a new vertex and index buffer every frame.
        self.draw_call.last_mut().unwrap().push(GpuDrawCall { mesh: self.immediate.build() });
    }
}
impl IMeshBuilder for Drawer
{
    fn geometry(&mut self, vertex: impl IntoIterator<Item = Vertex<3>>, index: impl IntoIterator<Item = VertexIndex>) {
        self.immediate.geometry(vertex, index);
    }
}

impl Deref for Drawer
{
    type Target=MeshBuilder;
    fn deref(&self) -> &Self::Target { &self.immediate }
}
impl DerefMut for Drawer
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.immediate }
}

#[derive(Clone, Default)]
pub struct GpuDrawCalls
{
    pub(crate) calls: Vec<GpuDrawCall>
}
impl GpuDrawCalls
{
    pub fn push(&mut self, call: GpuDrawCall) { self.calls.push(call); }
}

#[derive(Clone)]
pub struct GpuDrawCall
{
    pub(crate) mesh: Mesh
}


