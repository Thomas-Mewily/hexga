use super::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GSlice<'a, T>
where
    T: GBufferElement,
{
    pub(crate) buffer: &'a GBuffer<T>,
    pub(crate) begin: usize,
    pub(crate) len: usize,
}
impl<'a, T> From<&'a GBuffer<T>> for GSlice<'a, T>
where
    T: GBufferElement,
{
    fn from(buffer: &'a GBuffer<T>) -> Self
    {
        Self {
            buffer,
            begin: 0,
            len: buffer.len(),
        }
    }
}
impl<'a, T> GSlice<'a, T>
where
    T: GBufferElement,
{
    pub fn new<S: RangeBounds<usize>>(buffer: &'a GBuffer<T>, bounds: S) -> Self
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

impl<'a, T> Collection<T> for GSlice<'a, T>
where
    T: GBufferElement,
{
    type Guard<'b> = GElementRead<'b, T> where Self: 'b;
}
impl<'a, T> Get<usize> for GSlice<'a, T>
where
    T: GBufferElement,
{
    type Output = T;
    type Guard<'b> = GElementRead<'b, T> where Self: 'b, Self::Output: 'b;
    fn get<'b>(&'b self, index: usize) -> Option<Self::Guard<'b>> where Self::Output: 'b
    {
        if index >= self.len { return None; }
        Some(GElementRead::new(self.buffer.read(..), self.begin + index))
    }
}
impl<'a, T> Length for GSlice<'a, T>
where
    T: GBufferElement,
{
    fn len(&self) -> usize { self.len }
}

impl<'a, T> WgpuSliceable<T> for GSlice<'a, T>
where
    T: GBufferElement,
{
    fn wgpu_buffer_ref(&self) -> &WgpuBuffer { self.buffer.wgpu_buffer_ref() }
}

impl<'a, T> GpuSliceable<T> for GSlice<'a, T>
where
    T: GBufferElement,
{
    fn usage(&self) -> GBufferUsageFlags { self.buffer.usage() }

    fn as_slice(&self) -> GSlice<'_, T> { *self }

    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GSlice<'_, T>
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

        GSlice {
            buffer: self.buffer,
            begin: absolute_begin,
            len: absolute_len,
        }
    }

    fn read<S: RangeBounds<usize>>(&self, bounds: S) -> GSliceRead<'_, T>
    {
        let slice = self.slice(bounds);
        self.buffer.read(slice.begin..slice.begin + slice.len)
    }
}
