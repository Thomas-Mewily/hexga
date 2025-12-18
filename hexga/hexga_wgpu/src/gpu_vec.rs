use super::*;



#[derive(Clone, Debug, PartialEq, Hash)]
pub struct GpuVec<T>
{
    pub(crate) buffer : GpuBuffer<T>,
    pub(crate) len    : usize,
}
impl<T> Clear for GpuVec<T>
{
    fn clear(&mut self) {
        self.len = 0;
    }
}



pub trait ToGpuVec<T> where T:BitAllUsed
{
    fn to_gpu_vec(self, desc: GpuBufferDesc) -> GpuVec<T>;
}
impl<T> ToGpuVec<T> for &[T] where T:BitAllUsed
{
    fn to_gpu_vec(self, desc: GpuBufferDesc) -> GpuVec<T> {
        GpuVec::new(self, desc)
    }
}

impl<T> GpuBufferNew<T> for GpuVec<T> where GpuBuffer<T>: GpuBufferNew<T>
{
    fn new(value: &[T], desc: GpuBufferDesc) -> Self {
        let len = value.len();
        Self { buffer: GpuBuffer::new(value, desc), len }
    }

    fn with_capacity(capacity: usize, desc: GpuBufferDesc) -> Self {
        Self { buffer: GpuBuffer::with_capacity(capacity, desc), len: capacity }
    }
}
impl<T> GpuByteBuffer for GpuVec<T> where GpuBuffer<T>: GpuByteBuffer
{
    fn wgpu_bytes_len(&self) -> GpuBufferAddress { (self.len * std::mem::size_of::<T>()) as GpuBufferAddress }
    fn wgpu_bytes_capacity(&self) -> GpuBufferAddress { self.buffer.wgpu_bytes_capacity() }
    fn untyped_slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuUntypedSlice<'_> { self.buffer.untyped_slice(bounds) }
}

impl<T> GpuBufferRead<T> for GpuVec<T> where wgpu::Buffer: GpuBufferRead<T>
{
    fn read_in(&self, vec: &mut Vec<T>) -> GpuResult where T: BitZero + BitPattern
    {
        self.buffer.read_in(vec)
    }

    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSlice<'_,T> {
        self.buffer.slice(bounds)
    }
}