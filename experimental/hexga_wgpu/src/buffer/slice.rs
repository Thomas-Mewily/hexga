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
        let range = checked_range(bounds, buffer.len()).expect("GPU slice range out of bounds");
        Self { buffer, begin: range.start, len: range.len() }
    }
}

impl_gpu_index!(['s, T: GBufferElement] GSlice<'s, T>);

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
    fn lib_wgpu_buffer_ref(&self) -> &WgpuBuffer { self.buffer.lib_wgpu_buffer_ref() }
}

impl<'a, T> GpuSliceable<T> for GSlice<'a, T>
where
    T: GBufferElement,
{
    fn usage(&self) -> GBufferUsageFlags { self.buffer.usage() }

    fn as_slice(&self) -> GSlice<'_, T> { *self }

    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GSlice<'_, T>
    {
        let range = checked_range(bounds, self.len).expect("GPU subslice range out of bounds");
        GSlice { buffer: self.buffer, begin: self.begin + range.start, len: range.len() }
    }
}
