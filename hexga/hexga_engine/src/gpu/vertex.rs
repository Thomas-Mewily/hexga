use super::*;

pub type Vertex2 = Vertex<2>;
pub type Vertex3 = Vertex<3>;

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct Vertex<const N:usize=3>
{
    position: GpuVector<N>,
    color: GpuColor,
}
impl From<Vertex2> for Vertex3
{
    fn from(value: Vertex2) -> Self {
        Self { position: value.position.with_z(zero()), color: value.color }
    }
}



unsafe impl<const N:usize> bytemuck::Zeroable for Vertex<N> {}
unsafe impl<const N:usize> bytemuck::Pod for Vertex<N> {}



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


impl Vertex<3>
{
    pub(crate) fn create_buffer_layout() -> wgpu::VertexBufferLayout<'static> 
    {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: GpuVector::<3>::VERTEX_FORMAT,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<GpuVec3>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: GpuColor::VERTEX_FORMAT,
                },
            ],
        }
    }
}

pub type VertexIndex = u32;

pub mod prelude
{
    pub use super::{Vertex,Vertex2,Vertex3,VertexIndex};
}
