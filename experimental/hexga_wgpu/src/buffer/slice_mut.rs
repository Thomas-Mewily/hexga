use super::*;

#[repr(C)]
pub struct GSliceMut<'a, T>
where
    T: GBufferElement,
{
    pub(crate) buffer: &'a mut GBuffer<T>,
    begin: usize,
    len: usize,
}
impl<'a, T> From<&'a mut GBuffer<T>> for GSliceMut<'a, T>
where
    T: GBufferElement,
{
    fn from(buffer: &'a mut GBuffer<T>) -> Self
    {
        let len = buffer.len();
        Self { buffer, begin: 0, len }
    }
}
impl<'a, T> GSliceMut<'a, T>
where
    T: GBufferElement,
{
    pub fn new<S: RangeBounds<usize>>(buffer: &'a mut GBuffer<T>, bounds: S) -> Self
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

impl<'a, T> Deref for GSliceMut<'a, T>
where
    T: GBufferElement,
{
    type Target = GSlice<'a, T>;

    fn deref(&self) -> &Self::Target
    {
        // SAFETY: GpuSliceMut contains the same fields as GpuSlice,
        // but with a mutable reference instead of shared reference.
        // Since we're only exposing read-only operations through GpuSlice,
        // this is safe.
        unsafe { &*(self as *const Self as *const GSlice<'a, T>) }
    }
}

impl<'a, T> Collection<usize> for GSliceMut<'a, T>
where
    T: GBufferElement,
{
    type Output = T;
    type Ref<'b> = GElementRead<'b, T> where Self: 'b, Self::Output: 'b;
}
impl<'a, T> Get<usize> for GSliceMut<'a, T>
where
    T: GBufferElement,
{
    fn get<'b>(&'b self, index: usize) -> Option<Self::Ref<'b>> where Self::Output: 'b
    {
        if index >= self.len { return None; }
        Some(GElementRead::new(self.buffer.read(..), self.begin + index))
    }
}
impl<'a, T> Length for GSliceMut<'a, T>
where
    T: GBufferElement,
{
    fn len(&self) -> usize { self.len }
}

impl<'a, T> WgpuSliceable<T> for GSliceMut<'a, T>
where
    T: GBufferElement,
{
    fn wgpu_buffer_ref(&self) -> &WgpuBuffer { self.buffer.wgpu_buffer_ref() }
}

impl<'a, T> GpuSliceable<T> for GSliceMut<'a, T>
where
    T: GBufferElement,
{
    fn usage(&self) -> GBufferUsageFlags { self.deref().usage() }

    fn as_slice(&self) -> GSlice<'_, T> { *self.deref() }

    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GSlice<'_, T> { self.deref().slice(bounds) }

    fn read<S: RangeBounds<usize>>(&self, bounds: S) -> GSliceRead<'_, T> { self.deref().read(bounds) }
}

impl<'a, T> GpuSliceableMut<T> for GSliceMut<'a, T>
where
    T: GBufferElement,
{
    fn slice_mut<S: RangeBounds<usize>>(&mut self, bounds: S) -> GSliceMut<'_, T>
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

        GSliceMut {
            buffer: self.buffer,
            begin: absolute_begin,
            len: absolute_len,
        }
    }

    fn as_mut_slice(&mut self) -> GSliceMut<'_, T>
    {
        GSliceMut {
            buffer: self.buffer,
            begin: self.begin,
            len: self.len,
        }
    }
}
