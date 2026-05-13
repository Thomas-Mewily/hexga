use super::*;


#[repr(C)]
pub struct GpuSliceMut<'a, T>
where T: Copy
{
    buffer: WgpuBufferSlice<'a>,
    phantom: PhantomData<T>
}
impl<'a,T> GpuSliceMut<'a, T> where T: Copy 
{
    pub unsafe fn from_wgpu(buffer: WgpuBufferSlice<'a>) -> Self {
        Self { buffer, phantom: PhantomData }
    }
}

impl<'a, T> Deref for GpuSliceMut<'a, T> 
where T: Copy
{
    type Target = GpuSlice<'a, T>;
    
    fn deref(&self) -> &Self::Target {
        // SAFETY: GpuSliceMut contains the same fields as GpuSlice,
        // but with a mutable reference instead of shared reference.
        // Since we're only exposing read-only operations through GpuSlice,
        // this is safe.
        unsafe {
            &*(self as *const Self as *const GpuSlice<'a, T>)
        }
    }
}


impl<'a, T> Collection for GpuSliceMut<'a, T> where T: Copy {}
impl<'a, T> Length for GpuSliceMut<'a, T> where T: Copy
{
    fn len(&self) -> usize { 
        let size : WgpuBufferAddress = self.buffer.size().into();
        size as usize 
    }
}

impl<'a, T> WgpuSliceable<T> for GpuSliceMut<'a, T>
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

impl<'a, T> GpuSliceableMut<T> for GpuSliceMut<'a, T>
    where T: Copy
{
    fn write<S: RangeBounds<usize>>(&mut self, bounds: S) -> GpuSliceMutWrite<'_, T> {
        
    }
}