use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vertex {
    position: GVec3,
    color: GColor,
}



unsafe impl bytemuck::Zeroable for Vertex {}
unsafe impl bytemuck::Pod for Vertex {}




pub const VERTEX_LIST: &[Vertex] = &[
    Vertex { position: gvec3(0.0, 1.0, 0.0), color: rgb(1.0, 0.0, 0.0) },
    Vertex { position: gvec3(-0.5, -0.5, 0.0), color: rgb(0.0, 1.0, 0.0) },
    Vertex { position: gvec3(0.5, 0.0, 0.0), color: rgb(0.0, 0.0, 1.0) },
];

pub(crate) fn create_vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            },
            wgpu::VertexAttribute {
                offset: size_of::<GVec3>() as wgpu::BufferAddress,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x3,
            },
        ],
    }
}




#[derive(Clone, Debug, Default, PartialEq)]
pub struct Mesh
{
    vertices: Vec<Vertex>,
}
impl Mesh
{
    pub fn new() -> Self { ___() }
    pub fn vertices(&self) -> &[Vertex] { &self.vertices }
    pub fn vertices_mut(&mut self) -> &mut [Vertex] { &mut self.vertices }
}

impl Mesh
{
    pub fn draw(&self)
    {
        
    }
}


pub mod prelude
{
    pub use super::{Vertex,Mesh};
}
