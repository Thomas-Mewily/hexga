use super::*;

pub mod prelude
{
    pub use super::{UV, Vertex, VertexIndex, traits::*};
}

pub mod traits
{}

pub type VertexIndex = u32;

pub type Vertex2 = VertexOf<2>;
pub type Vertex3 = VertexOf<3>;
pub type Vertex = Vertex3;

pub type UV = GpuVector<2>;

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct VertexOf<const N: usize = 3>
{
    pub position: GpuVector<N>,
    pub color: GpuColor,
    pub uv: UV,
}

pub trait WgpuVertexDesc
{
    fn wgpu_vertex_description() -> wgpu::VertexBufferLayout<'static>;
}
impl<const N: usize> WgpuVertexDesc for VertexOf<N>
where
    GpuVector<N>: WgpuVertex,
{
    fn wgpu_vertex_description() -> wgpu::VertexBufferLayout<'static>
    {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: std::mem::offset_of!(Self, position) as wgpu::BufferAddress,
                    shader_location: 0,
                    format: GpuVector::<N>::GPU_VERTEX_FORMAT,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::offset_of!(Self, color) as wgpu::BufferAddress,
                    shader_location: 1,
                    format: GpuColor::GPU_VERTEX_FORMAT,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::offset_of!(Self, uv) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: UV::GPU_VERTEX_FORMAT,
                },
            ],
        }
    }
}

// + no padding
unsafe impl<const N: usize> BitAllUsed for VertexOf<N>
where
    GpuVector<N>: BitAllUsed,
    GpuColor: BitAllUsed,
    UV: BitAllUsed,
{
}
impl<const N: usize> VertexOf<N>
{
    pub const fn new() -> Self
    {
        Self {
            position: GpuVector::ZERO,
            color: GpuColor::WHITE,
            uv: zero(),
        }
    }
    pub const fn with_position(self, position: GpuVector<N>) -> Self { Self { position: position, ..self } }
    pub const fn with_color(self, color: GpuColor) -> Self { Self { color: color, ..self } }
    pub const fn with_uv(self, uv: UV) -> Self { Self { uv: uv, ..self } }
}
impl From<Vertex2> for Vertex3
{
    fn from(value: Vertex2) -> Self
    {
        Self {
            position: value.position.with_z(zero()),
            color: value.color,
            uv: value.uv,
        }
    }
}
