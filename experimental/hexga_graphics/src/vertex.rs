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

pub type UV = GVector<2>;

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct VertexOf<const N: usize = 3>
{
    pub position: GVector<N>,
    pub color: GColor,
    pub uv: UV,
}

pub trait WgpuVertexDesc
{
    fn wgpu_vertex_description() -> wgpu::VertexBufferLayout<'static>;
}
impl<const N: usize> WgpuVertexDesc for VertexOf<N>
where
    GVector<N>: WgpuVertex,
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
                    format: GVector::<N>::GPU_VERTEX_FORMAT,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::offset_of!(Self, color) as wgpu::BufferAddress,
                    shader_location: 1,
                    format: GColor::GPU_VERTEX_FORMAT,
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
    GVector<N>: BitAllUsed,
    GColor: BitAllUsed,
    UV: BitAllUsed,
{
}
impl<const N: usize> VertexOf<N>
{
    pub const fn new() -> Self
    {
        Self {
            position: GVector::ZERO,
            color: GColor::WHITE,
            uv: zero(),
        }
    }
    pub const fn with_position(self, position: GVector<N>) -> Self { Self { position: position, ..self } }
    pub const fn with_color(self, color: GColor) -> Self { Self { color: color, ..self } }
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
