use super::*;

/// A strongly typed buffer that live on the GPU.
/// This have regular value semantics with Copy-on-Write (CoW) optimization:
/// 
/// - Cloning performs a deep copy semantically, but internally shares the underlying GPU buffer
///   until a modification is required (copy-on-write).
/// 
/// # Performance Characteristics
/// 
/// - `Clone` is O(1) and cheap: it only increments a reference count internally.
/// 
/// - Mutation is cheap when the buffer is uniquely owned; a deep clone of the GPU buffer
///   occurs only when the buffer is shared.
#[derive(Clone)]
pub struct GpuBuffer<T>
    where T: GpuBufferElement
{
    pub(crate) buffer: Arc<WgpuBuffer>,
    typed: PhantomData<T>,
}

impl<T> Debug for GpuBuffer<T>
    where T: GpuBufferElement
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result 
    {
        f.debug_struct(&format!("GpuBuffer<{}>", std::any::type_name::<T>())).field("buffer", &self.buffer).finish()
    }
}



impl<T> Collection for GpuBuffer<T> where T: GpuBufferElement {}
impl<T> Length for GpuBuffer<T> where T: GpuBufferElement
{
    fn len(&self) -> usize { self.buffer.size() as _ }
}

impl<T> GpuBufferNew<T> for GpuBuffer<T>
    where T: BitAllUsed
{
    fn new(value: &[T], usage: GpuBufferUsageFlags) -> Self {
        Self { buffer: Arc::new(<WgpuBuffer as GpuBufferNew<T>>::new(value, usage)), typed: PhantomData }
    }
    
    fn with_capacity(capacity: usize, usage: GpuBufferUsageFlags) -> Self {
        WithCapacity::with_capacity_and_param(capacity, usage)
    }
}

impl<T> WithCapacity for GpuBuffer<T>
    where T: BitAllUsed
{
    type Param=GpuBufferUsageFlags;
    fn with_capacity_and_param(capacity: usize, usage: Self::Param) -> Self {
        Self { buffer: Arc::new(<WgpuBuffer as GpuBufferNew<T>>::with_capacity(capacity, usage)), typed: PhantomData }
    }
}


impl<T> WgpuSliceable<T> for GpuBuffer<T>
    where T: GpuBufferElement
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


impl<T> GpuSliceable<T> for GpuBuffer<T> 
    where T: GpuBufferElement
{
    fn usage(&self) -> GpuBufferUsageFlags { self.wgpu_usage().into() }
    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSlice<'_, T> { GpuSlice::new(self, bounds) }

    fn read<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSliceRead<'_, T> {
        unsafe { GpuSliceRead::from_wgpu(WgpuSliceable::<T>::wgpu_view(self.buffer.deref())) }
    }
}

impl<T> GpuSliceableMut<T> for GpuBuffer<T> 
    where T: GpuBufferElement
{
    fn slice_mut<S: RangeBounds<usize>>(&mut self, bounds: S) -> GpuSliceMut<'_, T> { GpuSliceMut::new(self, bounds) }
}
