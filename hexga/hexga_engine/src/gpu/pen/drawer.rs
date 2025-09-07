use super::*;


/* 
pub struct MeshBuilder
{
    pub(crate) vertices: GpuVec<Vertex>,
    pub(crate) indices: GpuVec<VertexIndex>,
}

impl IDrawer for MeshBuilder
{

}
*/


impl Scoped<Draw> for Drawer
{
    fn begin(&mut self) 
    {
        self.immediate.clear();
        self.draw_call.clear();
        self.camera.begin_draw();
    }

    fn end(&mut self) 
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
    fn matrix(&self) -> Mat4 { self.camera.matrix() }
    fn have_depth(&self) -> bool { self.camera.have_depth() }
    fn viewport(&self) -> Option<Rect2P> { self.camera.viewport() }
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


pub mod prelude
{

}
