use super::*;

mod usage;
pub use usage::*;

mod buffer;
pub use buffer::*;

mod slice;
pub use slice::*;

mod slice_mut;
pub use slice_mut::*;

mod read;
pub use read::*;

mod write;
pub use write::*;

mod vec;
pub use vec::*;

pub mod prelude 
{
    pub use super::traits::*;
}

pub mod traits 
{
    pub use super::{GpuBufferNew,GpuBufferElement,GpuSliceable,GpuSliceableMut};
}

pub trait GpuBufferNew<T> //: WithCapacity<Param=GpuBufferUsageFlags>
    where T: BitAllUsed
{
    fn new(value: &[T], usage: GpuBufferUsageFlags) -> Self;
    // Redirect to the WithCapacity. Can't impl WithCapacity for WgpuBuffer since this lib own neither.
    fn with_capacity(capacity: usize, usage: GpuBufferUsageFlags) -> Self;
}
pub trait GpuSliceable<T> //: WgpuSliceable<T>
    where T: GpuBufferElement
{
    fn usage(&self) -> GpuBufferUsageFlags;

    fn slice<'a,S: RangeBounds<usize>>(&'a self, bounds: S) -> GpuSlice<'a, T>;
    fn as_slice<'a>(&'a self) -> GpuSlice<'a, T> { self.slice(..) }

    fn read<'a,S: RangeBounds<usize>>(&'a self, bounds: S) -> GpuSliceRead<'a, T>;
    fn as_read<'a>(&'a self) -> GpuSliceRead<'a, T> { self.read(..) }
}


pub trait GpuBufferElement : Copy + 'static {}
impl<T> GpuBufferElement for T where T: Copy + 'static {}

pub trait GpuSliceableMut<T> where T: GpuBufferElement
{
    fn slice_mut<S: RangeBounds<usize>>(&mut self, bounds: S) -> GpuSliceMut<'_, T>;
    fn as_mut_slice(&mut self) -> GpuSliceMut<'_, T> { self.slice_mut(..) }

    fn write<S: RangeBounds<usize>>(&mut self, bounds: S) -> GpuSliceMutWrite<'_, T> 
    {
        let slice_mut = self.slice_mut(bounds);
        if Arc::get_mut(&mut slice_mut.buffer.buffer).is_none()
        {
            // COW : Copy on write
            let buff = slice_mut.wgpu_deep_clone();
            slice_mut.buffer.buffer = Arc::new(buff);
        }

        unsafe {
            GpuSliceMutWrite::from_wgpu(WgpuSliceable::<T>::wgpu_view_mut(slice_mut.buffer.buffer.deref())) 
        }
    }
    fn as_write<S: RangeBounds<usize>>(&mut self) -> GpuSliceMutWrite<'_, T> { self.write(..) } 

    /*
    fn write<S: RangeBounds<usize>>(&mut self, bounds: S) -> GpuSliceMutWrite<'_, T>
    {
        unsafe {
            GpuSliceMutWrite::from_wgpu(self.wgpu_view_mut()) 
        }
    }
    fn update(&mut self, src: &mut [T]) { self.write(..).view.copy_from_slice(unsafe { bit::try_transmute_slice_unchecked(src) }.unwrap()); }
    */
    //fn fill(&mut self, value: T) { self.write(..).view.fill(value); }
    // fn update_from_fn ?
}
