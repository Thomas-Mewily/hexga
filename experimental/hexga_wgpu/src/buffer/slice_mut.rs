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
        let range = checked_range(bounds, buffer.len()).expect("GPU slice range out of bounds");
        Self { buffer, begin: range.start, len: range.len() }
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

impl_gpu_index!(['s, T: GBufferElement] GSliceMut<'s, T>);

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
    fn lib_wgpu_buffer_ref(&self) -> &WgpuBuffer { self.buffer.lib_wgpu_buffer_ref() }
}

impl<'a, T> GpuSliceable<T> for GSliceMut<'a, T>
where
    T: GBufferElement,
{
    fn usage(&self) -> GBufferUsageFlags { self.deref().usage() }

    fn as_slice(&self) -> GSlice<'_, T> { *self.deref() }

    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GSlice<'_, T> { self.deref().slice(bounds) }
}

impl<'a, T> GpuSliceableMut<T> for GSliceMut<'a, T>
where
    T: GBufferElement,
{
    fn slice_mut<S: RangeBounds<usize>>(&mut self, bounds: S) -> GSliceMut<'_, T>
    {
        let range = checked_range(bounds, self.len).expect("GPU subslice range out of bounds");
        GSliceMut { buffer: self.buffer, begin: self.begin + range.start, len: range.len() }
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
