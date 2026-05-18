use super::*;

#[repr(C)]
pub struct GpuSliceMut<'a, T>
where
    T: GpuBufferElement,
{
    pub(crate) buffer: &'a mut GpuBuffer<T>,
    begin: usize,
    len: usize,
}
impl<'a, T> From<&'a mut GpuBuffer<T>> for GpuSliceMut<'a, T>
where
    T: GpuBufferElement,
{
    fn from(buffer: &'a mut GpuBuffer<T>) -> Self
    {
        let len = buffer.len();
        Self { buffer, begin: 0, len }
    }
}
impl<'a, T> GpuSliceMut<'a, T>
where
    T: GpuBufferElement,
{
    pub fn new<S: RangeBounds<usize>>(buffer: &'a mut GpuBuffer<T>, bounds: S) -> Self
    {
        let buffer_len = buffer.len();

        let begin = match bounds.start_bound()
        {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
            Bound::Unbounded => 0,
        };

        let end = match bounds.end_bound()
        {
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => buffer_len,
        };

        debug_assert!(
            begin <= buffer_len,
            "Slice start index {} out of bounds for buffer of length {}",
            begin,
            buffer_len
        );

        debug_assert!(end <= buffer_len, "Slice end index {} out of bounds for buffer of length {}", end, buffer_len);

        debug_assert!(begin <= end, "Slice start index {} must be <= end index {}", begin, end);

        let len = end - begin;

        Self { buffer, begin, len }
    }
}

impl<'a, T> Deref for GpuSliceMut<'a, T>
where
    T: GpuBufferElement,
{
    type Target = GpuSlice<'a, T>;

    fn deref(&self) -> &Self::Target
    {
        // SAFETY: GpuSliceMut contains the same fields as GpuSlice,
        // but with a mutable reference instead of shared reference.
        // Since we're only exposing read-only operations through GpuSlice,
        // this is safe.
        unsafe { &*(self as *const Self as *const GpuSlice<'a, T>) }
    }
}

impl<'a, T> Collection for GpuSliceMut<'a, T> where T: GpuBufferElement {}
impl<'a, T> Length for GpuSliceMut<'a, T>
where
    T: GpuBufferElement,
{
    fn len(&self) -> usize { self.len }
}

impl<'a, T> WgpuSliceable<T> for GpuSliceMut<'a, T>
where
    T: GpuBufferElement,
{
    fn wgpu_usage(&self) -> WgpuBufferUsage { self.buffer.wgpu_usage() }

    fn wgpu_slice<S: RangeBounds<WgpuBufferAddress>>(&self, bounds: S) -> WgpuBufferSlice<'_> { self.buffer.wgpu_slice(bounds) }

    fn wgpu_as_slice(&self) -> WgpuBufferSlice<'_> { self.buffer.wgpu_as_slice() }

    fn wgpu_view(&self) -> WgpuBufferView<'_> { self.buffer.wgpu_view() }

    fn wgpu_view_mut(&self) -> WgpuBufferViewMut<'_> { self.buffer.wgpu_view_mut() }

    fn wgpu_deep_clone_order(&self) -> WgpuBuffer { self.buffer.wgpu_deep_clone_order() }
}

impl<'a, T> GpuSliceable<T> for GpuSliceMut<'a, T>
where
    T: GpuBufferElement,
{
    fn usage(&self) -> GpuBufferUsageFlags { self.deref().usage() }

    fn as_slice(&self) -> GpuSlice<'_, T> { *self.deref() }

    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSlice<'_, T> { self.deref().slice(bounds) }

    fn read<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSliceRead<'_, T> { self.deref().read(bounds) }
}

impl<'a, T> GpuSliceableMut<T> for GpuSliceMut<'a, T>
where
    T: GpuBufferElement,
{
    fn slice_mut<S: RangeBounds<usize>>(&mut self, bounds: S) -> GpuSliceMut<'_, T>
    {
        let slice_len = self.len;

        let relative_start = match bounds.start_bound()
        {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
            Bound::Unbounded => 0,
        };

        let relative_end = match bounds.end_bound()
        {
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => slice_len,
        };

        debug_assert!(
            relative_start <= slice_len,
            "Sub-slice start index {} out of bounds for slice of length {}",
            relative_start,
            slice_len
        );

        debug_assert!(
            relative_end <= slice_len,
            "Sub-slice end index {} out of bounds for slice of length {}",
            relative_end,
            slice_len
        );

        debug_assert!(
            relative_start <= relative_end,
            "Sub-slice start {} must be <= end {}",
            relative_start,
            relative_end
        );

        let absolute_begin = self.begin + relative_start;
        let absolute_len = relative_end - relative_start;

        GpuSliceMut {
            buffer: self.buffer,
            begin: absolute_begin,
            len: absolute_len,
        }
    }

    fn as_mut_slice(&mut self) -> GpuSliceMut<'_, T>
    {
        GpuSliceMut {
            buffer: self.buffer,
            begin: self.begin,
            len: self.len,
        }
    }
}
