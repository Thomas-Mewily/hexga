use super::*;


pub mod prelude
{
    pub use super::{GpuVec,GpuVecDesc,SliceToGpuVec,IteratorToGpuVec};
}


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
        Self { usages: GpuVecUsages::empty(), name: None }
    }
}
impl GpuVecDesc
{
    pub const fn new() -> Self { Self { usages: GpuVecUsages::empty(), name: None }}

    pub const fn with_usages(mut self, usages : GpuVecUsages) -> Self { self.usages = usages; self }
    pub const fn with_label(mut self, label : Option<&'static str>) -> Self { self.name = label; self }

    pub const VERTEX : Self = Self::new().with_usages(GpuVecUsages::VERTEX);
    pub const INDEX : Self = Self::new().with_usages(GpuVecUsages::INDEX);
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
        Self::_with_capacity(&mut Gpu.device, capacity, desc)
    }

    pub(crate) fn _new(device: &mut wgpu::Device, value: &[T], desc: GpuVecDesc) -> Self
    {
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                value.as_ptr() as *const u8,
                value.len() * std::mem::size_of::<T>(),
            )
        };
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytes,
            usage: desc.usages,
        });
        let capacity = value.len();
        let len = value.len();
        Self::new_full(buffer, capacity, len, desc)
    }

    pub fn new(value: &[T], desc: GpuVecDesc) -> Self
    {
        Self::_new(&mut Gpu.device, value, desc)
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
