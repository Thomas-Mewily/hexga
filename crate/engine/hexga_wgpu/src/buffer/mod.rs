use super::*;

pub mod experimental;
pub(crate) use experimental::*;

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

pub mod prelude {}
pub mod traits {}

pub trait GpuBufferNew<T>
    where T: Copy
{
    fn new(value: &[T], usage: GpuBufferUsageFlags) -> Self;
}
pub trait GpuSliceable<T> : WgpuSliceable<T>
    where T: Copy
{
    fn usage(&self) -> GpuBufferUsageFlags { self.wgpu_usage().into() }

    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSlice<'_, T> { 
        
        let start_byte = match bounds.start_bound() {
            Bound::Included(&i) => Bound::Included(i as WgpuBufferAddress),
            Bound::Excluded(&i) => Bound::Excluded(i as WgpuBufferAddress),
            Bound::Unbounded => Bound::Unbounded,
        };
        
        let end_byte = match bounds.end_bound() {
            Bound::Included(&i) => Bound::Included((i + 1) as WgpuBufferAddress),
            Bound::Excluded(&i) => Bound::Excluded(i as WgpuBufferAddress),
            Bound::Unbounded => Bound::Unbounded,
        };
        
        let byte_bounds = (start_byte, end_byte);
        
        unsafe { 
            GpuSlice::<T>::from_wgpu(self.wgpu_slice(byte_bounds)) 
        }
    }
    fn as_slice(&self) -> GpuSlice<'_, T> { unsafe { GpuSlice::from_wgpu(self.wgpu_as_slice()) } }

    fn read<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSliceRead<'_, T> 
    {
        unsafe { GpuSliceRead::from_wgpu(self.wgpu_view()) }
    }
}
impl<T,S> GpuSliceable<T> for S where S: WgpuSliceable<T>, T: Copy {}

pub trait GpuSliceableMut<T: Copy> : GpuSliceable<T>
{
    fn slice_mut<S: RangeBounds<usize>>(&mut self, bounds: S) -> GpuSliceMut<'_, T>
    {
        let start_byte = match bounds.start_bound() {
            Bound::Included(&i) => Bound::Included(i as WgpuBufferAddress),
            Bound::Excluded(&i) => Bound::Excluded(i as WgpuBufferAddress),
            Bound::Unbounded => Bound::Unbounded,
        };
        
        let end_byte = match bounds.end_bound() {
            Bound::Included(&i) => Bound::Included((i + 1) as WgpuBufferAddress),
            Bound::Excluded(&i) => Bound::Excluded(i as WgpuBufferAddress),
            Bound::Unbounded => Bound::Unbounded,
        };
        
        let byte_bounds = (start_byte, end_byte);
        
        unsafe { 
            GpuSliceMut::<T>::from_wgpu(self.wgpu_slice(byte_bounds)) 
        }
    }
    fn as_mut_slice(&mut self) -> GpuSliceMut<'_, T> { self.slice_mut(..) }

    fn write<S: RangeBounds<usize>>(&mut self, bounds: S) -> GpuSliceMutWrite<'_, T>
    {
        unsafe {
            GpuSliceMutWrite::from_wgpu(self.wgpu_view_mut()) 
        }
    }
    fn update(&mut self, src: &mut [T]) { self.write(..).view.copy_from_slice(unsafe { bit::try_transmute_slice_unchecked(src) }.unwrap()); }
    //fn fill(&mut self, value: T) { self.write(..).view.fill(value); }
    // fn update_from_fn ?
}