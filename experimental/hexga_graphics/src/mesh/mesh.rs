use super::*;

/// A mesh, stored on the Cpu and automatically send to the Gpu when needed.
#[derive(Clone, Debug)]
pub struct Mesh<const N: usize = 3> {
    gmesh: GMesh<N>,
    builder: Dirty<MeshBuilder<N>>,
}

impl<const N: usize> Default for Mesh<N> {
    fn default() -> Self {
        Self::from(MeshBuilder::default())
    }
}

impl<const N: usize> BuilderMesh<N> for Mesh<N> {
    fn extend(&mut self, vertex: impl IntoIterator<Item = VertexOf<N>>, index: impl IntoIterator<Item = VertexIndex>) {
        self.builder.extend(vertex, index);
    }

    fn geometry(&mut self, vertex: &[VertexOf<N>], index: &[VertexIndex]) {
        self.builder.geometry(vertex, index);
    }
}

impl<const N: usize> Clear for Mesh<N> {
    fn clear(&mut self) {
        self.builder.clear();
    }
}

impl<const N: usize> Mesh<N> {
    pub fn new() -> Self {
        ___()
    }

    pub fn from_vertices_and_indices(vertices: Vec<VertexOf<N>>, indices: Vec<VertexIndex>) -> Self {
        Self::from(MeshBuilder::from_vertices_and_indices(vertices, indices))
    }

    pub fn gmesh(&mut self) -> &GMesh<N> {
        self.builder.undirty_lazy(|b| self.gmesh.clear_and_push(b));
        &self.gmesh
    }
}
impl<const N: usize> From<MeshBuilder<N>> for Mesh<N> {
    fn from(builder: MeshBuilder<N>) -> Self {
        Self {
            gmesh: ___(),
            builder: Dirty::new_dirty(builder),
        }
    }
}
