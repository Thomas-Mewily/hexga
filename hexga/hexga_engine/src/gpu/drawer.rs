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
    }

    fn end(&mut self) {
        self.flush();
    }
}

#[derive(Default)]
pub struct Drawer
{
    pub(crate) immediate : MeshBuilder,
    pub(crate) draw_call : Vec<GpuDrawCalls>
}
impl Drawer
{
    pub fn new() -> Self { ___() }

    /// Transform the immediate [MeshBuilder] in a [GpuDrawCall]
    pub fn flush(&mut self)
    {
        if self.immediate.is_empty() { return; }
        if self.draw_call.is_empty()
        {
            self.draw_call.push(___());
        }
        // Todo : Not opti, create a new buffer every frame.
        // Tmp just to test
        self.draw_call.last_mut().unwrap().push(GpuDrawCall { mesh: self.immediate.build() });
    }

    //pub(crate) begin_draw()
}
impl IMeshBuilder for Drawer
{
    fn extends(&mut self, vertex: impl IntoIterator<Item = Vertex<3>>, index: impl IntoIterator<Item = VertexIndex>) {
        self.immediate.extends(vertex, index);
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
