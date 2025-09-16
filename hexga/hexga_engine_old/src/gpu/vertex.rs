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
impl<const N:usize> Vertex<N>
{
    pub const fn new() -> Self { Self { position: GpuVector::ZERO, color: GpuColor::WHITE } }
    pub const fn with_position(self, position: GpuVector<N>) -> Self { Self { position: position, ..self } }
    pub const fn with_color(self, color: GpuColor) -> Self { Self { color: color, ..self } }
}
impl From<Vertex2> for Vertex3
{
    fn from(value: Vertex2) -> Self {
        Self { position: value.position.with_z(zero()), color: value.color }
    }
}


//unsafe impl<const N:usize> bytemuck::Zeroable for Vertex<N> {}
//unsafe impl<const N:usize> bytemuck::Pod for Vertex<N> {}


pub type VertexIndex = u32;
