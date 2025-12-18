use super::*;

pub(crate) mod prelude
{
    pub use super::vertex::{UV,Vertex,VertexIndex};
}


pub type VertexIndex = u32;


pub type Vertex2 = VertexOf<2>;
pub type Vertex3 = VertexOf<3>;
pub type Vertex = Vertex3;

pub type UV = GpuVector<2>;

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct VertexOf<const N:usize=3>
{
    pub position: GpuVector<N>,
    pub color: GpuColor,
    pub uv: UV,
}
unsafe impl<const N:usize> BitAllUsed for VertexOf<N> {}
impl<const N:usize> VertexOf<N>
{
    pub const fn new() -> Self { Self { position: GpuVector::ZERO, color: GpuColor::WHITE, uv: zero() } }
    pub const fn with_position(self, position: GpuVector<N>) -> Self { Self { position: position, ..self } }
    pub const fn with_color(self, color: GpuColor) -> Self { Self { color: color, ..self } }
    pub const fn with_uv(self, uv: UV) -> Self { Self { uv: uv, ..self } }
}
impl From<Vertex2> for Vertex3
{
    fn from(value: Vertex2) -> Self {
        Self { position: value.position.with_z(zero()), color: value.color, uv: zero() }
    }
}