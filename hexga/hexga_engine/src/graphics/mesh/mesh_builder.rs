use super::*;


pub trait IMeshBuilder<const N:usize=3>
{
    /// Index are relative to the vertex passed
    fn geometry(&mut self, vertex: impl IntoIterator<Item = VertexOf<N>>, index: impl IntoIterator<Item = VertexIndex>);


    fn triangles_indexed(&mut self, vertex: impl IntoIterator<Item = VertexOf<N>>, index: impl IntoIterator<Item = TriangleVertexIndex>)
    {
        self.geometry(vertex, index.into_iter().flatten())
    }



    fn triangle(&mut self, triangle: TriangleVertex<N>)
    {
        self.geometry(triangle.points, [0,1,2]);
    }
    fn triangles(&mut self, triangles: impl IntoIterator<Item = TriangleVertex<N>>)
    {
        for triangle in triangles.into_iter()
        {
            self.triangle(triangle)
        }
    }
    // circle, oval...
}



#[derive(Default, Clone, PartialEq, PartialOrd, Debug)]
pub struct MeshBuilder<const N:usize=3>
{
    vertices: Vec<VertexOf<N>>,
    // Generally a multiple of 3, because 3 VertexIndex = 1 Triangle
    // But I don't want to force it with Vec<TriangleVertexIndex> in case I change it later
    indices: Vec<VertexIndex>,
    //indices: Vec<TriangleVertexIndex>,
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
    pub fn from_vertices_and_indices(vertices: Vec<VertexOf<N>>, indices: Vec<VertexIndex>) -> Self
    {
        Self { vertices, indices }
    }

    pub fn is_valid(&self) -> bool
    {
        if  self.indices.len() % 3 != 0 { return false; }
        for indice in  self.indices.iter().copied()
        {
            if indice >= self.vertices.len() as VertexIndex { return false; }
        }
        true
    }

    pub fn is_empty(&self) -> bool { self.indices.is_empty() && self.vertices.is_empty() }

    pub fn nb_vertex(&self) -> usize { self.vertices.len() }
    pub fn vertices(&self) -> &[VertexOf<N>] { &self.vertices }
    pub fn vertices_mut(&mut self) -> &mut [VertexOf<N>] { &mut self.vertices }

    pub fn nb_index(&self) -> usize { self.indices.len() }
    pub fn indices(&self) -> &[VertexIndex] { &self.indices }
    // Todo: Cool effect, glitch/corrut N % of the triangles
    pub fn indices_mut(&mut self) -> &mut [VertexIndex] { &mut self.indices }

    pub fn nb_triangle(&self) -> usize { self.nb_index() / 3 }
    pub fn triangles(&self) -> &[TriangleVertexIndex]
    {
        // Safety: TriangleVertexIndex is repr(C) with the same layout as [VertexIndex; 3].
        let (tris, remainder) = self.indices.as_chunks::<3>();
        assert!(remainder.is_empty(), "indices length must be divisible by 3");
        unsafe { std::slice::from_raw_parts(tris.as_ptr().cast(), tris.len()) }
    }
    pub fn triangles_mut(&mut self) -> &mut [TriangleVertexIndex] {
        // Safety: TriangleVertexIndex is repr(C) with the same layout as [VertexIndex; 3].
        let (tris, remainder) = self.indices.as_chunks_mut::<3>();
        assert!(remainder.is_empty(), "indices length must be divisible by 3");
        unsafe { std::slice::from_raw_parts_mut(tris.as_mut_ptr().cast(), tris.len()) }
    }

    pub fn to_vertices_and_indices(self) -> (Vec<VertexOf<N>>, Vec<VertexIndex>) { (self.vertices, self.indices) }
    pub fn to_vertices_and_triangles_indices(self) -> (Vec<VertexOf<N>>, Vec<TriangleVertexIndex>) { (self.vertices, self.indices.chunks_exact(3).map(|c| TriangleVertexIndex::new(c[0], c[1], c[2])).collect()) }
}

impl<const N:usize> Builder for MeshBuilder<N>
{
    type Output= Mesh<N>;
    fn build(&self) -> Self::Output
    {
        assert!(self.is_valid());
        Mesh::new(&self.vertices, &self.indices)
    }

    fn build_in(&self, dest: &mut Self::Output) {
        dest.indices.replace(&self.indices);
        dest.vertices.replace(&self.vertices);
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
    fn geometry(&mut self, vertex: impl IntoIterator<Item = VertexOf<N>>, index: impl IntoIterator<Item = VertexIndex>)
    {
        let vertices_prev_len = self.nb_vertex();
        self.vertices.extend(vertex);
        let vertices_new_len = self.nb_vertex();
        let nb_vertices_added = vertices_new_len - vertices_prev_len;

        let index_offset = vertices_prev_len as VertexIndex;

        let indices_prev_len = self.nb_index();
        self.indices.extend(index.into_iter().map(|idx| idx + index_offset));
        let indices_new_len = self.nb_index();
        let nb_indices_added = indices_new_len - indices_prev_len;
        assert_eq!(nb_indices_added % 3, 0, "Index need to be a multiple of 3, because they form a triangle");
    }
}
