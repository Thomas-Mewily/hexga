use super::*;
use std::ops::Range as StdRange;

// Concrete range types coexist with `Get<usize>`. Custom bounds use `try_read`.
macro_rules! impl_gpu_index
{
    (@ranges $params:tt $buffer:ty; $($index:ty),+ $(,)?) => {
        $(impl_gpu_index!(@one $params $buffer; $index);)+
    };
    (@one [$($params:tt)*] $buffer:ty; $index:ty) => {
        impl<$($params)*> CollectionIndex<$index> for $buffer
        {
            type Output = [T];
            type Ref<'a> = <Self as Collection<[T]>>::Ref<'a> where Self: 'a, [T]: 'a;
        }
        impl<$($params)*> Get<$index> for $buffer
        {
            fn get<'a>(&'a self, index: $index) -> Option<Self::Ref<'a>> where Self::Output: 'a
            {
                self.try_read(index)
            }
        }
        impl<$($params)*> TryGet<$index> for $buffer
        {
            type Error = IndexOutOfBounds<$index, StdRange<usize>>;

            fn try_get<'a>(&'a self, index: $index) -> Result<Self::Ref<'a>, Self::Error> where [T]: 'a
            {
                Get::get(self, index.clone()).ok_or_else(|| IndexOutOfBounds::new(index, 0..self.len()))
            }
        }
    };
    ([$($params:tt)*] $buffer:ty) => {
        impl<$($params)*> Collection<T> for $buffer
        {
            type Ref<'a> = GElementRead<'a, T> where Self: 'a, T: 'a;
        }
        impl<$($params)*> CollectionIndex<usize> for $buffer
        {
            type Output = T;
            type Ref<'a> = <Self as Collection<T>>::Ref<'a> where Self: 'a, T: 'a;
        }
        impl<$($params)*> Get<usize> for $buffer
        {
            fn get<'a>(&'a self, index: usize) -> Option<Self::Ref<'a>> where T: 'a
            {
                let end = index.checked_add(1)?;
                Some(GElementRead::new(self.try_read(index..end)?))
            }
        }

        impl<$($params)*> First<usize> for $buffer
        {
            fn first<'a>(&'a self) -> Option<Self::Ref<'a>> where T: 'a { Get::get(self, 0usize) }
        }
        impl<$($params)*> Last<usize> for $buffer
        {
            fn last<'a>(&'a self) -> Option<Self::Ref<'a>> where T: 'a
            {
                Get::get(self, self.len().checked_sub(1)?)
            }
        }
        impl<$($params)*> TryGet<usize> for $buffer
        {
            type Error = IndexOutOfBounds<usize, StdRange<usize>>;

            fn try_get<'a>(&'a self, index: usize) -> Result<Self::Ref<'a>, Self::Error> where T: 'a
            {
                Get::get(self, index).ok_or_else(|| IndexOutOfBounds::new(index, 0..self.len()))
            }
        }

        impl<$($params)*> Collection<[T]> for $buffer
        {
            type Ref<'a> = GSliceRead<'a, T> where Self: 'a, T: 'a;
        }
        impl_gpu_index!(
            @ranges [$($params)*] $buffer;
            StdRange<usize>,
            std::ops::RangeFrom<usize>,
            std::ops::RangeTo<usize>,
            std::ops::RangeInclusive<usize>,
            std::ops::RangeToInclusive<usize>,
            std::ops::RangeFull,
            (Bound<usize>, Bound<usize>),
            hexga::range::Range<usize>,
        );
    };
}

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
    pub use super::buffer::GBuffer;
    pub use super::read::{GElementRead, GSliceRead};
    pub use super::slice::GSlice;
    pub use super::slice_mut::GSliceMut;
    pub use super::traits::*;
    pub use super::vec::GVec;
}

pub mod traits
{
    pub use super::buffer::ToGpuBuffer;
    pub use super::vec::ToGpuVec;
    pub use crate::buffer::{GBufferElement, GpuBufferNew, GpuSliceable, GpuSliceableMut};
}

pub trait GpuBufferNew<T>
where
    T: BitAllUsed,
{
    fn new(value: &[T], usage: GBufferUsageFlags) -> Self;
    // Redirect to the WithCapacity. Can't impl WithCapacity for WgpuBuffer since this lib own neither.
    fn with_capacity(capacity: usize, usage: GBufferUsageFlags) -> Self;
}
pub trait GpuSliceable<T>
where
    T: GBufferElement,
{
    fn usage(&self) -> GBufferUsageFlags;

    fn slice<'a, S: RangeBounds<usize>>(&'a self, bounds: S) -> GSlice<'a, T>;
    fn as_slice<'a>(&'a self) -> GSlice<'a, T> { self.slice(..) }

    fn read<S: RangeBounds<usize>>(&self, bounds: S) -> GSliceRead<'_, T>
    {
        self.try_read(bounds).expect("GPU read range out of bounds")
    }

    /// Returns a mapped view, or `None` if the element range is invalid.
    ///
    /// Nonempty ranges require a mapped buffer. wgpu may reject overlapping
    /// views, including ranges that overlap after alignment.
    fn try_read<S: RangeBounds<usize>>(&self, bounds: S) -> Option<GSliceRead<'_, T>>
    {
        let slice = self.as_slice();
        let range = checked_range(bounds, slice.len)?;
        Some(slice.buffer.read_elements(slice.begin + range.start..slice.begin + range.end))
    }

    fn as_read<'a>(&'a self) -> GSliceRead<'a, T> { self.read(..) }
}

pub trait GBufferElement: Copy + 'static {}
impl<T> GBufferElement for T where T: Copy + 'static {}

pub trait GpuSliceableMut<T>
where
    T: GBufferElement,
{
    fn slice_mut<S: RangeBounds<usize>>(&mut self, bounds: S) -> GSliceMut<'_, T>;
    fn as_mut_slice(&mut self) -> GSliceMut<'_, T> { self.slice_mut(..) }

    /// Writes to an already mapped buffer, detaching from any clones first.
    fn write<S: RangeBounds<usize>>(&mut self, bounds: S) -> GSliceMutWrite<'_, T>
    {
        let slice = self.slice_mut(bounds);
        let buffer = &mut slice.buffer.buffer;
        if Arc::get_mut(buffer).is_none()
        {
            // A mapped buffer cannot be used in a GPU copy command.
            let replacement = Gpu.device().create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: buffer.size(),
                usage: buffer.usage(),
                mapped_at_creation: true,
            });
            replacement.slice(..).get_mapped_range_mut()
                .copy_from_slice(&buffer.slice(..).get_mapped_range());
            *buffer = Arc::new(replacement);
        }

        unsafe { GSliceMutWrite::from_wgpu(buffer.slice(..).get_mapped_range_mut()) }
    }
    fn as_write<S: RangeBounds<usize>>(&mut self) -> GSliceMutWrite<'_, T> { self.write(..) }

    /*
    fn write<S: RangeBounds<usize>>(&mut self, bounds: S) -> GpuSliceMutWrite<'_, T>
    {
        unsafe {
            GpuSliceMutWrite::from_wgpu(self.lib_wgpu_view_mut())
        }
    }
    fn update(&mut self, src: &mut [T]) { self.write(..).view.copy_from_slice(unsafe { bit::try_transmute_slice_unchecked(src) }.unwrap()); }
    */
    //fn fill(&mut self, value: T) { self.write(..).view.fill(value); }
    // fn update_from_fn ?
}


fn checked_range(bounds: impl RangeBounds<usize>, len: usize) -> Option<StdRange<usize>>
{
    let start = match bounds.start_bound()
    {
        Bound::Included(&start) => start,
        Bound::Excluded(&start) => start.checked_add(1)?,
        Bound::Unbounded => 0,
    };
    let end = match bounds.end_bound()
    {
        Bound::Included(&end) => end.checked_add(1)?,
        Bound::Excluded(&end) => end,
        Bound::Unbounded => len,
    };
    (start <= end && end <= len).then_some(start..end)
}