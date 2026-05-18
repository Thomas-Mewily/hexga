use super::*;
/// A mesh, stored on the Gpu.
///
/// Can be cheaply [`Clone`]d
#[derive(Clone, Debug)]
pub struct Mesh<const N: usize = 3>
{
    pub vertices: GpuVec<VertexOf<N>>,
    pub indices: GpuVec<VertexIndex>,
}

impl<const N: usize> Clear for Mesh<N>
{
    fn clear(&mut self)
    {
        self.vertices.clear();
        self.indices.clear();
    }
}

impl<const N: usize> Mesh<N>
{
    pub(crate) fn from_gpu_vec(vertices: GpuVec<VertexOf<N>>, indices: GpuVec<VertexIndex>)
    -> Self
    {
        Self { vertices, indices }
    }

    pub fn new(vertices: &[VertexOf<N>], indices: &[VertexIndex]) -> Self
    {
        let indices = indices.to_gpu_vec(GpuBufferUsageFlags::Index);
        let vertices = vertices.to_gpu_vec(GpuBufferUsageFlags::Vertex);
        Self { vertices, indices }
    }

    pub fn from_iterator(
        vertices: impl IntoIterator<Item = VertexOf<N>>,
        indices: impl IntoIterator<Item = VertexIndex>,
    ) -> Self
    {
        Self::new(
            &vertices.into_iter().to_vec(),
            &indices.into_iter().to_vec(),
        )
    }
}
impl<const N: usize> From<&MeshBuilder<N>> for Mesh<N>
{
    fn from(mesh: &MeshBuilder<N>) -> Self { Self::new(mesh.vertices(), mesh.indices()) }
}
