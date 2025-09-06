use super::*;

pub mod prelude
{
    pub use super::{IMeshBuilder,MeshBuilder};
}

pub trait IMeshBuilder<const N:usize=3>
{
    /// Index are relative to the vertex passed
    fn extends(&mut self, vertex: impl IntoIterator<Item = Vertex<N>>, index: impl IntoIterator<Item = VertexIndex>);
    fn triangles(&mut self, triangle: impl IntoIterator<Item = MeshTriangle<N>>)
    {
        for triangles in triangle.into_iter()
        {
            self.extends(triangles.points, [0,1,2]);
        }
    }
}



#[derive(Default, Clone, PartialEq, PartialOrd, Debug)]
pub struct MeshBuilder<const N:usize=3>
{
    vertices: Vec<Vertex<N>>,
    // Multiple of 3, because 3 VertexIndex = 1 Triangle
    indices: Vec<VertexIndex>,
}

impl<const N:usize> MeshBuilder<N>
{
    pub const DEFAULT_CAPACITY_VERTEX : usize = 512;
    pub const DEFAULT_CAPACITY_INDEX : usize = Self::DEFAULT_CAPACITY_VERTEX * 2;

    pub fn new() -> Self 
    {
        Self::with_capacity(Self::DEFAULT_CAPACITY_VERTEX, Self::DEFAULT_CAPACITY_INDEX)
    }
    pub fn with_capacity(nb_vertices: usize, nb_indices: usize) -> Self
    {
        Self { vertices: Vec::with_capacity(nb_vertices), indices: Vec::with_capacity(nb_indices) }
    }
    pub fn from_vertices_and_capacity(vertices: Vec<Vertex<N>>, indices: Vec<VertexIndex>) -> Self
    {
        Self { vertices, indices }
    }
    pub fn is_valid(&self) -> bool
    {
        if  self.indices.len() % 3 != 0 { return false; }
        for indice3 in  self.indices.chunks_exact(3)
        {
            for indice in indice3.iter().copied()
            {
                if indice >= self.vertices.len() as VertexIndex { return false; }
            }
        }
        true
    }

    pub fn nb_vertices(&self) -> usize { self.vertices.len() }
    pub fn vertices(&self) -> &[Vertex<N>] { &self.vertices }
    pub fn vertices_mut(&mut self) -> &mut [Vertex<N>] { &mut self.vertices }
    
    pub fn nb_indices(&self) -> usize { self.indices.len() }
    pub fn indices(&self) -> &[VertexIndex] { &self.indices }
    pub fn indices_mut(&mut self) -> &mut [VertexIndex] { &mut self.indices }

    pub fn triangles(&self) -> impl Iterator<Item = MeshTriangle<N>>
    {
        self.indices.chunks_exact(3).map(|v| MeshTriangle::from_array(std::array::from_fn(|i| self.vertices[v[i] as usize])))
    }

    pub fn to_vertices_and_indices(self) -> (Vec<Vertex<N>>, Vec<VertexIndex>) { (self.vertices, self.indices) }

    pub fn build(&self) -> Mesh<N> 
    {
        debug_assert!(self.is_valid());
        Mesh::new(&self.vertices, &self.indices)
    }
}
impl<const N:usize> Clearable for MeshBuilder<N>
{
    fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }
}


impl<const N:usize> IMeshBuilder<N> for MeshBuilder<N>
{
    fn extends(&mut self, vertex: impl IntoIterator<Item = Vertex<N>>, index: impl IntoIterator<Item = VertexIndex>)
    {
        let vertices_prev_len = self.nb_vertices();
        self.vertices.extend(vertex);
        let vertices_new_len = self.nb_vertices();
        let nb_vertices_added = vertices_new_len - vertices_prev_len;

        let index_offset = vertices_prev_len as VertexIndex;

        let indices_prev_len = self.nb_indices();
        self.indices.extend(index.into_iter().map(|idx| idx + index_offset));
        let indices_new_len = self.nb_indices();
        let nb_indices_added = indices_new_len - indices_prev_len;
        assert_eq!(nb_indices_added % 3, 0, "Index need to be a multiple of 3, because they form a triangle");
    }
}
