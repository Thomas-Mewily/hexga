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
    fn begin(&mut self) {
        self.immediate.clear();
    }

    fn end(&mut self) {
        self.flush();
    }
}

#[derive(Default)]
pub struct Drawer
{
    pub(crate) immediate : MeshBuilder,
    pub(crate) call : Vec<GpuDrawCall>
}
impl Drawer
{
    pub fn new() -> Self { ___() }

    /// Transform the immediate [MeshBuilder] in a [GpuDrawCall]
    pub fn flush(&mut self)
    {
        
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

pub struct GpuDrawCall
{
    pass : Vec<GpuRenderPass>
}

pub struct GpuRenderPass
{
    verts: wgpu::Buffer
}


pub mod prelude
{

}
