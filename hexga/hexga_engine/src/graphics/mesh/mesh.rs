use super::*;


/// A mesh, stored on the Gpu.
///
/// Can be cheaply [std::clone::Cloned]
#[derive(Clone, PartialEq, Debug)]
pub struct Mesh<const N:usize=3>
{
    pub(crate) vertices: GpuVec<VertexOf<N>>,
    pub(crate) indices: GpuVec<VertexIndex>,
}
impl<const N:usize> Mesh<N>
{
    pub(crate) fn from_gpu_vec(vertices: GpuVec<VertexOf<N>>, indices: GpuVec<VertexIndex>) -> Self
    {
        Self { vertices, indices }
    }

    pub fn new(vertices : &[VertexOf<N>], indices : &[VertexIndex]) -> Self
    {
        let indices = indices.to_gpu_vec(GpuVecDesc::INDEX);
        let vertices = vertices.to_gpu_vec(GpuVecDesc::VERTEX);
        Self { vertices, indices }
    }

    pub fn from_iterator(vertices: impl IntoIterator<Item = VertexOf<N>>, indices: impl IntoIterator<Item = VertexIndex>) -> Self
    {
        Self::new(&vertices.into_iter().to_vec(), &indices.into_iter().to_vec())
    }
}
impl<const N:usize> From<MeshBuilder<N>> for Mesh<N>
{
    fn from(value: MeshBuilder<N>) -> Self
    {
        let (vertices, indices) = value.to_vertices_and_indices();
        Self::from_iterator(vertices, indices)
    }
}