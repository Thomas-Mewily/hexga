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
    pub pos: GpuVector<N>,
    pub color: GColor,
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
                    offset: std::mem::offset_of!(Self, pos) as wgpu::BufferAddress,
                    shader_location: 0,
                    format: GpuVector::<N>::VERTEX_FORMAT,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::offset_of!(Self, color) as wgpu::BufferAddress,
                    shader_location: 1,
                    format: GColor::VERTEX_FORMAT,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::offset_of!(Self, uv) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: UV::VERTEX_FORMAT,
                },
            ],
        }
    }
}

// + no padding
unsafe impl<const N: usize> BitAllUsed for VertexOf<N>
where
    GpuVector<N>: BitAllUsed,
    GColor: BitAllUsed,
    UV: BitAllUsed,
{
}
impl<const N: usize> VertexOf<N>
{
    pub const fn new() -> Self
    {
        Self {
            pos: GpuVector::ZERO,
            color: GColor::WHITE,
            uv: zero(),
        }
    }
    pub const fn with_position(self, position: GpuVector<N>) -> Self { Self { pos: position, ..self } }
    pub const fn with_color(self, color: GColor) -> Self { Self { color, ..self } }
    pub const fn with_uv(self, uv: UV) -> Self { Self { uv, ..self } }
}
impl From<Vertex2> for Vertex3
{
    fn from(value: Vertex2) -> Self
    {
        Self {
            pos: value.pos.with_z(zero()),
            color: value.color,
            uv: value.uv,
        }
    }
}
