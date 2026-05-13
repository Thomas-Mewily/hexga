use super::*;


#[repr(C)]
pub struct GpuSlice<'a, T>
    where T: Copy
{
    buffer: WgpuBufferSlice<'a>,
    phantom: PhantomData<T>
}
impl<'a,T> GpuSlice<'a, T> where T: Copy 
{
    pub unsafe fn from_wgpu(buffer: WgpuBufferSlice<'a>) -> Self {
        Self { buffer, phantom: PhantomData }
    }
}

impl<'a, T> Collection for GpuSlice<'a, T> where T: Copy {}
impl<'a, T> Length for GpuSlice<'a, T> where T: Copy
{
    fn len(&self) -> usize { 
        let size : WgpuBufferAddress = self.buffer.size().into();
        size as usize 
    }
}

impl<'a, T> WgpuSliceable<T> for GpuSlice<'a, T>
    where T:Copy
{
    fn wgpu_usage(&self) -> WgpuBufferUsage {
        WgpuSliceable::<T>::wgpu_usage(self.buffer.buffer())
    }

    fn wgpu_slice<S: RangeBounds<WgpuBufferAddress>>(&self, bounds: S) -> WgpuBufferSlice<'_> {
        WgpuSliceable::<T>::wgpu_slice(self.buffer.buffer(), bounds)
    }

    fn wgpu_as_slice(&self) -> WgpuBufferSlice<'_> {
        WgpuSliceable::<T>::wgpu_as_slice(self.buffer.buffer())
    }

    fn wgpu_view(&self) -> WgpuBufferView<'_> {
        WgpuSliceable::<T>::wgpu_view(self.buffer.buffer())
    }

    fn wgpu_view_mut(&self) -> WgpuBufferViewMut<'_> {
        WgpuSliceable::<T>::wgpu_view_mut(self.buffer.buffer())
    }

    fn wgpu_deep_clone_order(&self) -> WgpuBuffer {
        WgpuSliceable::<T>::wgpu_deep_clone_order(self.buffer.buffer())
    }
}