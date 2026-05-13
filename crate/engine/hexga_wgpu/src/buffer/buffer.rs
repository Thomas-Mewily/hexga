use super::*;

/// A strongly typed buffer that live on the GPU.
/// This have value like semantic + Copy on Write:
/// - Clone is always deep (don't worry, clonning the same value multiple time will reuse the same instance)
#[derive(Clone)]
pub struct GpuBuffer<T>
    where T: Copy
{
    buffer: Arc<WgpuBuffer>,
    typed: PhantomData<T>,
}

impl<T> Debug for GpuBuffer<T>
    where T: Copy
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result 
    {
        f.debug_struct(&format!("GpuBuffer<{}>", std::any::type_name::<T>())).field("buffer", &self.buffer).finish()
    }
}



impl<T> Collection for GpuBuffer<T> where T: Copy {}
impl<T> Length for GpuBuffer<T> where T: Copy
{
    fn len(&self) -> usize { self.buffer.size() as _ }
}

impl<T> GpuBufferNew<T> for GpuBuffer<T>
    where T: BitAllUsed
{
    fn new(value: &[T], usage: GpuBufferUsageFlags) -> Self {
        Self { buffer: Arc::new(<WgpuBuffer as GpuBufferNew<T>>::new(value, usage)), typed: PhantomData }
    }
}


impl<T> WgpuSliceable<T> for GpuBuffer<T>
    where T: Copy
{
    fn wgpu_usage(&self) -> WgpuBufferUsage {
        WgpuSliceable::<T>::wgpu_usage(self.buffer.deref())
    }

    fn wgpu_slice<S: RangeBounds<WgpuBufferAddress>>(&self, bounds: S) -> WgpuBufferSlice<'_> {
        WgpuSliceable::<T>::wgpu_slice(self.buffer.deref(), bounds)
    }

    fn wgpu_as_slice(&self) -> WgpuBufferSlice<'_> {
        WgpuSliceable::<T>::wgpu_as_slice(self.buffer.deref())
    }

    fn wgpu_view(&self) -> WgpuBufferView<'_> {
        WgpuSliceable::<T>::wgpu_view(self.buffer.deref())
    }

    fn wgpu_view_mut(&self) -> WgpuBufferViewMut<'_> {
        WgpuSliceable::<T>::wgpu_view_mut(self.buffer.deref())
    }
    
    fn wgpu_deep_clone_order(&self) -> WgpuBuffer {
        WgpuSliceable::<T>::wgpu_deep_clone_order(self.buffer.deref())
    }
}
impl<T> GpuSliceableMut<T> for GpuBuffer<T> 
    where T: Copy
{
    fn write<S: RangeBounds<usize>>(&mut self, bounds: S) -> GpuSliceMutWrite<'_, T> 
    {
        if Arc::get_mut(&mut self.buffer).is_none()
        {
            // COW : Copy on write
            let buff = self.wgpu_deep_clone();
            self.buffer = Arc::new(buff);
        }

        unsafe {
            GpuSliceMutWrite::from_wgpu(self.wgpu_view_mut()) 
        }
    }
}
