use super::*;

#[derive(Clone, Debug, Default)]
pub struct ImmediateRender<const N: usize = 3> {
    pub geometry: MeshBuilder<N>,
    pub draw_call: Vec<DrawCall<N>>,
}

impl<const N: usize> ImmediateRender<N> {
    pub fn clear(&mut self) {
        self.geometry.clear();
        self.draw_call.clear();
    }
}

#[derive(Clone, Debug)]
pub enum DrawGeometry<const N: usize = 3> {
    Immediate(DrawGeometryImmediate),
    Retain(GMesh<N>),
}
impl<const N: usize> DrawGeometry<N> {
    pub fn is_empty(&self) -> bool {
        match self {
            DrawGeometry::Immediate(immediate) => immediate.is_empty(),
            DrawGeometry::Retain(_) => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DrawGeometryImmediate {
    pub vertices: Range<usize>,
    pub indices: Range<usize>,
}
impl DrawGeometryImmediate {
    pub fn is_empty(&self) -> bool {
        debug_assert_eq!(self.vertices.is_empty(), self.indices.is_empty());
        self.vertices.is_empty() || self.indices.is_empty()
    }
}

/*
#[derive(Clone, Debug, Default)]
pub struct DrawGeometryImmediate
{
    pub vertices_begin: usize,
    pub vertices_len: usize,

    pub indices_begin: usize,
    pub indices_len: usize,
}
*/
/*
pub struct DrawGeometrySliceIndice
{
    pub begin: usize,
    pub len: usize,
}
*/
