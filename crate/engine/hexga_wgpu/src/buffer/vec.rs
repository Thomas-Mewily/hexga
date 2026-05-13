use super::*;

#[derive(Clone, Debug)]
pub struct GpuVec<T>
    where T: GpuBufferElement
{
    pub(crate) buffer: GpuBuffer<T>,
    pub(crate) len: usize,
}

impl<T> Collection for GpuVec<T> where T: GpuBufferElement {}
impl<T> Length for GpuVec<T> where T: GpuBufferElement
{
    fn len(&self) -> usize { self.len }
}
impl<T> Capacity for GpuVec<T> where T: GpuBufferElement
{
    fn capacity(&self) -> usize {
        self.buffer.len()
    }
}

impl<T> WithCapacity for GpuVec<T> where T: GpuBufferElement
{
    type Param=GpuBufferUsageFlags;
    fn with_capacity_and_param(capacity: usize, param: Self::Param) -> Self {
        
    }
}
impl<T> Reserve for GpuVec<T> where T: GpuBufferElement
{
    fn reserve(&mut self, additional: usize) {
        todo!()
    }

    fn reserve_exact(&mut self, additional: usize) {
        todo!()
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> {
        todo!()
    }

    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> {
        todo!()
    }
}


impl<T> Clear for GpuVec<T>
    where T: GpuBufferElement
{
    fn clear(&mut self) { self.len = 0; }
}