use super::*;


#[derive(Clone, Copy)]
#[repr(C)]
pub struct GpuSlice<'a, T>
    where T: GpuBufferElement
{
    buffer: &'a GpuBuffer<T>,
    pub(crate) begin: usize,
    pub(crate) len: usize,
}
impl<'a,T> From<&'a GpuBuffer<T>> for  GpuSlice<'a, T> where T: GpuBufferElement 
{
    fn from(buffer: &'a GpuBuffer<T>) -> Self {
        Self { buffer, begin: 0, len: buffer.len() }
    }
}
impl<'a,T> GpuSlice<'a, T> where T: GpuBufferElement 
{
    pub fn new<S: RangeBounds<usize>>(buffer: &'a GpuBuffer<T>, bounds: S) -> Self
    {
        use std::ops::Bound;
        
        let buffer_len = buffer.len();
        
        let begin = match bounds.start_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
            Bound::Unbounded => 0,
        };
        
        let end = match bounds.end_bound() {
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => buffer_len,
        };
        
        
        debug_assert!(
            begin <= buffer_len,
            "Slice start index {} out of bounds for buffer of length {}",
            begin, buffer_len
        );
        
        debug_assert!(
            end <= buffer_len,
            "Slice end index {} out of bounds for buffer of length {}",
            end, buffer_len
        );
        
        debug_assert!(
            begin <= end,
            "Slice start index {} must be <= end index {}",
            begin, end
        );

        let len = end - begin;
        
        Self 
        {
            buffer,
            begin,
            len,
        }
    }
}


impl<'a, T> Collection for GpuSlice<'a, T> where T: GpuBufferElement {}
impl<'a, T> Length for GpuSlice<'a, T> where T: GpuBufferElement
{
    fn len(&self) -> usize { 
        self.len
    }
}

impl<'a, T> WgpuSliceable<T> for GpuSlice<'a, T>
    where T:GpuBufferElement
{
    fn wgpu_usage(&self) -> WgpuBufferUsage {
        self.buffer.wgpu_usage()
    }

    fn wgpu_slice<S: RangeBounds<WgpuBufferAddress>>(&self, bounds: S) -> WgpuBufferSlice<'_> {
        self.buffer.wgpu_slice(bounds)
    }

    fn wgpu_as_slice(&self) -> WgpuBufferSlice<'_> {
        self.buffer.wgpu_as_slice()
    }

    fn wgpu_view(&self) -> WgpuBufferView<'_> {
        self.buffer.wgpu_view()
    }

    fn wgpu_view_mut(&self) -> WgpuBufferViewMut<'_> {
        self.buffer.wgpu_view_mut()
    }

    fn wgpu_deep_clone_order(&self) -> WgpuBuffer {
        self.buffer.wgpu_deep_clone_order()
    }
}

impl<'a, T> GpuSliceable<T> for GpuSlice<'a, T>
    where T:GpuBufferElement
{
    fn usage(&self) -> GpuBufferUsageFlags {
        self.buffer.usage()
    }

    fn as_slice(&self) -> GpuSlice<'_, T> {
        *self
    }

    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSlice<'_, T> {
        let slice_len = self.len;
        
        let relative_start = match bounds.start_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
            Bound::Unbounded => 0,
        };
        
        let relative_end = match bounds.end_bound() {
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => slice_len,
        };
        
        debug_assert!(
            relative_start <= slice_len,
            "Sub-slice start index {} out of bounds for slice of length {}",
            relative_start, slice_len
        );
        
        debug_assert!(
            relative_end <= slice_len,
            "Sub-slice end index {} out of bounds for slice of length {}",
            relative_end, slice_len
        );
        
        debug_assert!(
            relative_start <= relative_end,
            "Sub-slice start {} must be <= end {}",
            relative_start, relative_end
        );
        
        let absolute_begin = self.begin + relative_start;
        let absolute_len = relative_end - relative_start;
        
        GpuSlice {
            buffer: self.buffer,
            begin: absolute_begin,
            len: absolute_len,
        }
    }

    fn read<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSliceRead<'_, T> {
        let slice = self.slice(bounds);
        self.buffer.read(slice.begin..slice.begin + slice.len)
    }
}