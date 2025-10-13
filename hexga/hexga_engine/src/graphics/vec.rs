use std::ops::{Bound, RangeBounds};
use super::*;


pub type GpuVecUsages = wgpu::BufferUsages;

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct GpuVec<T> where T:Copy
{
    pub(crate) buffer   : wgpu::Buffer,
    pub(crate) capacity : usize,
    pub(crate) len      : usize,
    pub(crate) desc     : GpuVecDesc,
    phantom : PhantomData<T>,
}
impl<T> GpuVec<T> where T:Copy
{
    pub fn name(&self) -> Option<&'static str> { self.desc.name }
    pub fn clear(&mut self)
    {
        self.len = 0;
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GpuVecDesc
{
    pub usages: GpuVecUsages,
    pub name: Option<&'static str>
}
impl Default for GpuVecDesc
{
    fn default() -> Self {
        Self::new()
    }
}
impl GpuVecDesc
{
    pub const fn new() -> Self { Self { usages: GpuVecUsages::COPY_DST.union(GpuVecUsages::COPY_SRC), name: None }}

    pub const fn add_usage(mut self, usage : GpuVecUsages) -> Self { self.usages = self.usages.union(usage); self }
    pub const fn with_usages(mut self, usages : GpuVecUsages) -> Self { self.usages = usages; self }
    pub const fn with_label(mut self, label : Option<&'static str>) -> Self { self.name = label; self }

    pub const VERTEX : Self = Self::new().add_usage(GpuVecUsages::VERTEX);
    pub const INDEX : Self = Self::new().add_usage(GpuVecUsages::INDEX);
}




impl<T> GpuVec<T> where T:Copy
{
    pub(crate) fn new_full(buffer: wgpu::Buffer, capacity: usize, len: usize, desc: GpuVecDesc) -> Self
    {
        assert!(capacity >= len);
        Self { buffer, capacity, len, desc, phantom: PhantomData }
    }

    pub(crate) fn _with_capacity(device: &mut wgpu::Device, capacity: usize, desc: GpuVecDesc) -> Self
    {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: desc.usages,
            size: (capacity * std::mem::size_of::<T>()) as _,
            mapped_at_creation: false,
        });

        Self::new_full(buffer, capacity, capacity, desc)
    }

    pub fn with_capacity(capacity: usize, desc: GpuVecDesc) -> Self
    {
        Self::_with_capacity(&mut Pen.base.device, capacity, desc)
    }

    pub(crate) fn _new(device: &mut wgpu::Device, value: &[T], desc: GpuVecDesc) -> Self
    {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: value.as_u8_slice(),
            usage: desc.usages,
        });
        let capacity = value.len();
        let len = value.len();
        Self::new_full(buffer, capacity, len, desc)
    }

    pub fn new(value: &[T], desc: GpuVecDesc) -> Self
    {
        Self::_new(&mut Pen.base.device, value, desc)
    }

    /// Returns a typed slice of the underlying `wgpu::Buffer`.
    ///
    /// The given range is expressed in element indices (`T`), not raw bytes.
    pub(crate) fn wgpu_slice<S: RangeBounds<usize>>(&self, bounds: S) -> wgpu::BufferSlice<'_>
    {
        type WgpuBufIdx = wgpu::BufferAddress;
        let size = std::mem::size_of::<T>() as WgpuBufIdx;
        let start = match bounds.start_bound() {
            Bound::Included(&v) => v as WgpuBufIdx  * size,
            Bound::Excluded(&v) => v as WgpuBufIdx * size + size,
            Bound::Unbounded => 0,
        };
        let end = match bounds.end_bound()
        {
            Bound::Included(&v) => v as WgpuBufIdx * size + size,
            Bound::Excluded(&v) => v as WgpuBufIdx * size,
            Bound::Unbounded => self.len as WgpuBufIdx * size,
        };
        self.buffer.slice(start..end)
    }
}


impl<T> GpuVec<T> where T:Copy
{
    fn update_part(&mut self, offset: usize, data: &[T])
    {
        let required = offset + data.len();
        if required > self.capacity {

            let new_capacity = required.next_power_of_two();
            let mut new_buffer = Pen.base.device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                usage: self.desc.usages,
                size: (new_capacity * std::mem::size_of::<T>()) as _,
                mapped_at_creation: false,
            });

            if self.len > 0 {
                let mut encoder = Pen.base.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                encoder.copy_buffer_to_buffer(
                    &self.buffer,
                    0,
                    &new_buffer,
                    0,
                    (self.len * std::mem::size_of::<T>()) as wgpu::BufferAddress,
                );
                Pen.base.queue.submit(Some(encoder.finish()));
            }

            self.buffer = new_buffer;
            self.capacity = new_capacity;
        }
        assert!(offset + data.len() <= self.capacity);
        Pen.base.queue.write_buffer(&self.buffer, offset as _, data.as_u8_slice());
    }

    pub fn replace(&mut self, data: &[T])
    {
        self.update_part(0, data);
        self.len = data.len();
    }
}

pub trait SliceToGpuVec<T> where T:Copy
{
    fn to_gpu_vec(self, desc: GpuVecDesc) -> GpuVec<T>;
}
impl<T> SliceToGpuVec<T> for &[T] where T:Copy
{
    fn to_gpu_vec(self, desc: GpuVecDesc) -> GpuVec<T> {
        GpuVec::new(self, desc)
    }
}
pub trait IteratorToGpuVec<T> where T:Copy
{
    fn collect_to_gpu_vec(self, desc: GpuVecDesc) -> GpuVec<T>;
}
impl<S,T> IteratorToGpuVec<T> for S where S: IntoIterator<Item = T>, T:Copy
{
    fn collect_to_gpu_vec(self, desc: GpuVecDesc) -> GpuVec<T>
    {
        let value = self.into_iter().to_vec();
        GpuVec::new(&value, desc)
    }
}
