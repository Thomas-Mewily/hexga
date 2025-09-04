use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vertex {
    position: GpuVec3,
    color: GpuColor,
}



unsafe impl bytemuck::Zeroable for Vertex {}
unsafe impl bytemuck::Pod for Vertex {}



/* 
pub(crate) const VERTEX_LIST: &[Vertex] = &[
    Vertex { position: gvec3(-0.0868241, 0.49240386, 0.0), color: GColor::RED },
    Vertex { position: gvec3(-0.49513406, 0.06958647, 0.0), color: GColor::GREEN },
    Vertex { position: gvec3(-0.21918549, -0.44939706, 0.0), color: GColor::BLUE },
    Vertex { position: gvec3(0.35966998, -0.3473291, 0.0), color: GColor::BLUE },
    Vertex { position: gvec3(0.44147372, 0.2347359, 0.0), color: GColor::BLUE },
];

pub(crate) const VERTEX_INDICES: &[VertexIndex] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];
*/

pub(crate) const VERTEX_LIST: &[Vertex] = &[
    // Front face
    Vertex { position: gpu_vec3(-0.5, -0.5,  0.5), color: GpuColor::RED   }, // 0
    Vertex { position: gpu_vec3( 0.5, -0.5,  0.5), color: GpuColor::GREEN }, // 1
    Vertex { position: gpu_vec3( 0.5,  0.5,  0.5), color: GpuColor::BLUE  }, // 2
    Vertex { position: gpu_vec3(-0.5,  0.5,  0.5), color: GpuColor::WHITE }, // 3

    // Back face
    Vertex { position: gpu_vec3(-0.5, -0.5, -0.5), color: GpuColor::YELLOW }, // 4
    Vertex { position: gpu_vec3( 0.5, -0.5, -0.5), color: GpuColor::CYAN   }, // 5
    Vertex { position: gpu_vec3( 0.5,  0.5, -0.5), color: GpuColor::MAGENTA}, // 6
    Vertex { position: gpu_vec3(-0.5,  0.5, -0.5), color: GpuColor::BLACK  }, // 7
];

pub(crate) const VERTEX_INDICES: &[VertexIndex] = &[
    // Front
    0, 1, 2,
    2, 3, 0,
    // Right
    1, 5, 6,
    6, 2, 1,
    // Back
    5, 4, 7,
    7, 6, 5,
    // Left
    4, 0, 3,
    3, 7, 4,
    // Top
    3, 2, 6,
    6, 7, 3,
    // Bottom
    4, 5, 1,
    1, 0, 4,
];


impl Vertex
{
    pub(crate) const WGPU_FORMAT_X3 : wgpu::VertexFormat = 
    {
        match std::mem::size_of::<Gpufloat>()
        {
            4 => wgpu::VertexFormat::Float32x3,
            8 => wgpu::VertexFormat::Float64x3,
            _ => unreachable!()
        }
    };

    pub(crate) const WGPU_FORMAT_X4 : wgpu::VertexFormat = 
    {
        match std::mem::size_of::<Gpufloat>()
        {
            4 => wgpu::VertexFormat::Float32x4,
            8 => wgpu::VertexFormat::Float64x4,
            _ => unreachable!()
        }
    };

    pub(crate) fn create_buffer_layout() -> wgpu::VertexBufferLayout<'static> 
    {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: Self::WGPU_FORMAT_X3,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<GpuVec3>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: Self::WGPU_FORMAT_X4,
                },
            ],
        }
    }

    pub(crate) const WGPU_INDEX_FORMAT : wgpu::IndexFormat = 
    {
        match std::mem::size_of::<VertexIndex>()
        {
            2 => wgpu::IndexFormat::Uint16,
            4 => wgpu::IndexFormat::Uint32,
            _ => unreachable!(),
        }
    };
}

pub type VertexIndex = u32;


#[derive(Clone, Debug, Default, PartialEq)]
pub struct Mesh
{
    vertices: Vec<Vertex>,
    indices : Vec<VertexIndex>,
}
impl Mesh
{
    pub fn new() -> Self { ___() }
    pub fn with_vertices_and_indices(vertices: Vec<Vertex>, indices: Vec<VertexIndex>) -> Self { Self { vertices, indices }}

    pub fn vertices(&self) -> &[Vertex] { &self.vertices }
    pub fn vertices_mut(&mut self) -> &mut [Vertex] { &mut self.vertices }

    pub fn indices(&self) -> &[VertexIndex] { &self.indices }
    pub fn indices_mut(&mut self) -> &mut [VertexIndex] { &mut self.indices }
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
