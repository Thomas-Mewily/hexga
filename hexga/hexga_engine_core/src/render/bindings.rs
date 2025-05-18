use super::*;


/// Geometry bindings
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bindings {
    /// Vertex buffers. Data contained in the buffer must match layout
    /// specified in the `Pipeline`.
    ///
    /// Most commonly vertex buffer will contain `(x,y,z,w)` coordinates of the
    /// vertex in 3d space, as well as `(u,v)` coordinates that map the vertex
    /// to some position in the corresponding `Texture`.
    pub vertex_buffers: Vec<Buffer>,
    /// Index buffer which instructs the GPU in which order to draw vertices
    /// from a vertex buffer, with each subsequent 3 indices forming a
    /// triangle.
    pub index_buffer: Buffer,
    /// Textures to be used with when drawing the geometry in the fragment
    /// shader.
    pub images: Vec<Texture>,
}

impl Bindings
{
    pub fn view<'a>(&'a self) -> BindingsView<'a> 
    {
        let Self { vertex_buffers, index_buffer, images } = self;
        BindingsView { vertex_buffers, index_buffer : *index_buffer, images }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BindingsView<'a>
{
    /// Vertex buffers. Data contained in the buffer must match layout
    /// specified in the `Pipeline`.
    ///
    /// Most commonly vertex buffer will contain `(x,y,z,w)` coordinates of the
    /// vertex in 3d space, as well as `(u,v)` coordinates that map the vertex
    /// to some position in the corresponding `Texture`.
    pub vertex_buffers: &'a [Buffer],
    /// Index buffer which instructs the GPU in which order to draw vertices
    /// from a vertex buffer, with each subsequent 3 indices forming a
    /// triangle.
    pub index_buffer: Buffer,
    /// Textures to be used with when drawing the geometry in the fragment
    /// shader.
    pub images: &'a [Texture],
}
