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
pub struct GBuffer<T>
where
    T: GBufferElement,
{
    pub(crate) buffer: Arc<WgpuBuffer>,
    typed: PhantomData<T>,
}

impl<T> Handle for GBuffer<T> where T: GBufferElement {}

impl<T> Debug for GBuffer<T>
where
    T: GBufferElement,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        f.debug_struct(&format!("GpuBuffer<{}>", std::any::type_name::<T>()))
            .field("buffer", &self.buffer)
            .finish()
    }
}

impl<T: GBufferElement> Collection<T> for GBuffer<T>
{
    type Ref<'a> = GElementRead<'a, T> where Self: 'a, T: 'a;
}

impl<T> CollectionIndex<usize> for GBuffer<T>
where
    T: GBufferElement,
{
    type Output = T;
    type Ref<'a> = GElementRead<'a, T> where Self: 'a, Self::Output: 'a;
}
impl<T> Get<usize> for GBuffer<T>
where
    T: GBufferElement,
{
    fn get<'a>(&'a self, index: usize) -> Option<Self::Ref<'a>> where Self::Output: 'a
    {
        if index >= self.buffer.size() as usize / size_of::<T>() { return None; }
        Some(GElementRead::new(self.read(..), index))
    }
}
impl<T> Length for GBuffer<T>
where
    T: GBufferElement,
{
    fn len(&self) -> usize { self.buffer.size() as _ }
}

impl<T> GpuBufferNew<T> for GBuffer<T>
where
    T: BitAllUsed,
{
    fn new(value: &[T], usage: GBufferUsageFlags) -> Self
    {
        Self {
            buffer: Arc::new(<WgpuBuffer as GpuBufferNew<T>>::new(value, usage)),
            typed: PhantomData,
        }
    }

    fn with_capacity(capacity: usize, usage: GBufferUsageFlags) -> Self { WithCapacity::with_capacity_and_param(capacity, usage) }
}

impl<T> WithCapacity for GBuffer<T>
where
    T: BitAllUsed,
{
    type Param = GBufferUsageFlags;
    fn with_capacity_and_param(capacity: usize, usage: Self::Param) -> Self
    {
        Self {
            buffer: Arc::new(<WgpuBuffer as GpuBufferNew<T>>::with_capacity(capacity, usage)),
            typed: PhantomData,
        }
    }
}

impl<T> WgpuSliceable<T> for GBuffer<T>
where
    T: GBufferElement,
{
    fn wgpu_buffer_ref(&self) -> &WgpuBuffer { &self.buffer }
}

impl<T> GpuSliceable<T> for GBuffer<T>
where
    T: GBufferElement,
{
    fn usage(&self) -> GBufferUsageFlags { self.wgpu_usage().into() }
    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GSlice<'_, T> { GSlice::new(self, bounds) }

    fn read<S: RangeBounds<usize>>(&self, bounds: S) -> GSliceRead<'_, T>
    {
        unsafe { GSliceRead::from_wgpu(WgpuSliceable::<T>::wgpu_view(self.buffer.deref())) }
    }
}

impl<T> GpuSliceableMut<T> for GBuffer<T>
where
    T: GBufferElement,
{
    fn slice_mut<S: RangeBounds<usize>>(&mut self, bounds: S) -> GSliceMut<'_, T> { GSliceMut::new(self, bounds) }
}

pub trait ToGpuBuffer<T>
where
    T: BitAllUsed,
{
    fn to_gpu_buffer(self, desc: GBufferUsageFlags) -> GBuffer<T>;
}
impl<T> ToGpuBuffer<T> for &[T]
where
    T: BitAllUsed,
{
    fn to_gpu_buffer(self, desc: GBufferUsageFlags) -> GBuffer<T> { GBuffer::new(self, desc) }
}
