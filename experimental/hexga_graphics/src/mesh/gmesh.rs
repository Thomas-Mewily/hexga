use super::*;

/// A mesh, stored on the Gpu.
///
/// Can be cheaply [`Clone`]d
#[derive(Clone, Debug)]
pub struct GMesh<const N: usize = 3>
{
    pub vertices: GVec<VertexOf<N>>,
    pub indices: GVec<VertexIndex>,
}
impl<const N: usize> Handle for GMesh<N> {}

impl<const N: usize> Default for GMesh<N>
{
    fn default() -> Self {
        Self::new(&[], &[])
    }
}

impl<const N: usize> Clear for GMesh<N>
{
    fn clear(&mut self)
    {
        self.vertices.clear();
        self.indices.clear();
    }
}

impl<const N: usize> GMesh<N>
{
    pub(crate) fn from_gpu_vec(vertices: GVec<VertexOf<N>>, indices: GVec<VertexIndex>) -> Self { Self { vertices, indices } }

    pub fn new(vertices: &[VertexOf<N>], indices: &[VertexIndex]) -> Self
    {
        let indices = indices.to_gpu_vec(GBufferUsageFlags::Index | GBufferUsageFlags::CopySourceDest);
        let vertices = vertices.to_gpu_vec(GBufferUsageFlags::Vertex | GBufferUsageFlags::CopySourceDest);
        Self { vertices, indices }
    }

    pub fn from_iter(vertices: impl IntoIterator<Item = VertexOf<N>>, indices: impl IntoIterator<Item = VertexIndex>) -> Self
    {
        Self::new(&vertices.into_iter().to_vec(), &indices.into_iter().to_vec())
    }

    pub fn clear_and_push(&mut self, mesh: &MeshBuilder<N>) {
        // Can be opti later if needed, need to impl clear_and_push on GVec too
        self.clear();
        self.push(mesh);
    }
}
impl<const N: usize> Push<&MeshBuilder<N>> for GMesh<N>
{
    type Output = ();

    fn push(&mut self, mesh: &MeshBuilder<N>) -> Self::Output {
        self.vertices.push(mesh.vertices());
        self.indices.push(mesh.indices());
    }
}

impl<const N: usize> From<&MeshBuilder<N>> for GMesh<N>
{
    fn from(mesh: &MeshBuilder<N>) -> Self { Self::new(mesh.vertices(), mesh.indices()) }
}
