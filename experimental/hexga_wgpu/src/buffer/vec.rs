use super::*;

/// A growable, strongly typed vector backed by a GPU buffer.
///
/// Its length counts the elements in the vector. Its capacity counts how many
/// elements fit in the backing allocation, including any whole padding elements.
///
/// # Performance, Cloning and mutation
///
/// - Cloning is O(1). Each clone has its own length and shares the GPU allocation.
/// - Mapped writes and queue updates detach from shared allocations before
///   changing data. Queue updates need [`GBufferUsage::Copy`] to
///   preserve existing elements when detaching.
///
/// - Growing beyond capacity allocates a new buffer and copies the existing
///   elements.
#[derive(Clone, Debug)]
pub struct GVec<T>
where
    T: GBufferElement,
{
    pub(crate) buffer: GBuffer<T>,
    pub(crate) len: usize,
}

impl<T> Handle for GVec<T> where T: GBufferElement {}

impl_gpu_index!([T: GBufferElement] GVec<T>);

impl<T: GBufferElement> CollectionPushStableKey for GVec<T> {}

impl<T> Length for GVec<T>
where
    T: GBufferElement,
{
    fn len(&self) -> usize {
        self.len
    }
}
impl<T> Capacity<T> for GVec<T>
where
    T: GBufferElement,
{
    fn capacity(&self) -> usize {
        self.buffer.len()
    }
}

impl<T> GpuBufferNew<T> for GVec<T>
where
    T: BitAllUsed,
{
    fn new(value: &[T], usage: GBufferUsageFlags) -> Self {
        Self {
            buffer: GBuffer::new(value, usage),
            len: value.len(),
        }
    }

    fn with_capacity(capacity: usize, usage: GBufferUsageFlags) -> Self {
        WithCapacity::with_capacity_and_param(capacity, usage)
    }
}

impl<T> WithCapacity for GVec<T>
where
    T: BitAllUsed,
{
    type Param = GBufferUsageFlags;

    fn with_capacity_and_param(capacity: usize, param: Self::Param) -> Self {
        Self {
            buffer: GBuffer::with_capacity_and_param(capacity, param),
            len: 0,
        }
    }
}

impl<T: GBufferElement> Truncate<T> for GVec<T> {
    fn truncate(&mut self, len: usize) {
        self.len = Ord::min(self.len, len);
    }
}

impl<T> Clear for GVec<T>
where
    T: GBufferElement,
{
    fn clear(&mut self) {
        self.len = 0;
    }
}

impl<T> Reserve<T> for GVec<T>
where
    T: GBufferElement + BitAllUsed,
{
    type Error = ();

    fn try_reserve(&mut self, additional: usize) -> Result<(), ()> {
        let new_capacity = self.len.checked_add(additional).ok_or(())?;

        if new_capacity > self.capacity() {
            let new_capacity_pow2 = new_capacity.next_power_of_two();
            self.force_reallocate(new_capacity_pow2)?;
        }
        Ok(())
    }

    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), ()> {
        let new_capacity = self.len.checked_add(additional).ok_or(())?;

        if new_capacity > self.capacity() {
            self.force_reallocate(new_capacity)?;
        }
        Ok(())
    }
}

impl<T> WgpuSliceable<T> for GVec<T>
where
    T: GBufferElement,
{
    fn lib_wgpu_slice<S: RangeBounds<usize>>(&self, bounds: S) -> WgpuBufferSlice<'_> {
        let range = checked_range(bounds, self.len).expect("GPU vector range out of bounds");
        WgpuSliceable::<T>::lib_wgpu_slice(self.lib_wgpu_buffer_ref(), range)
    }

    /*
    fn lib_wgpu_deep_clone_order(&self) -> WgpuBuffer
    {
        let size = self.size();
        let usage = WgpuSliceable::<T>::lib_wgpu_usage(self);

        let new_buffer = Gpu.device().create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage,
            mapped_at_creation: false,
        });

        let mut encoder = Gpu.device().create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        encoder.copy_buffer_to_buffer(self., 0, &new_buffer, 0, size);
        Gpu.queue().submit(Some(encoder.finish()));

        new_buffer
    }
    */

    fn lib_wgpu_buffer_ref(&self) -> &WgpuBuffer {
        self.buffer.lib_wgpu_buffer_ref()
    }
}

impl<T> GpuSliceable<T> for GVec<T>
where
    T: GBufferElement,
{
    fn usage(&self) -> GBufferUsageFlags {
        self.buffer.usage()
    }
    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GSlice<'_, T> {
        let range = checked_range(bounds, self.len).expect("GPU vector range out of bounds");
        self.buffer.slice(range)
    }
}

impl<T> GpuSliceableMut<T> for GVec<T>
where
    T: GBufferElement,
{
    fn slice_mut<S: RangeBounds<usize>>(&mut self, bounds: S) -> GSliceMut<'_, T> {
        let range = checked_range(bounds, self.len).expect("GPU vector range out of bounds");
        self.buffer.slice_mut(range)
    }
}

impl<'a, T> Push<&'a [T]> for GVec<T>
where
    T: BitAllUsed,
{
    type Output = ();
    fn push(&mut self, value: &'a [T]) -> Self::Output {
        self.update_part(self.len(), value);
    }
}
impl<'a, T> TryPush<&'a [T]> for GVec<T>
where
    T: BitAllUsed,
{
    type Error = ();
    fn try_push(&mut self, value: &'a [T]) -> Result<Self::Output, Self::Error> {
        self.try_update_part(self.len(), value)
    }
}

impl<T> GVec<T>
where
    T: BitAllUsed,
{
    pub fn update_part(&mut self, offset: usize, data: &[T]) {
        self.try_update_part(offset, data).expect("GpuVec failed to update");
    }

    pub fn try_update_part(&mut self, offset: usize, data: &[T]) -> Result<(), ()> {
        let elem_size = size_of::<T>();
        if elem_size == 0 || data.is_empty() {
            return Ok(());
        }

        let required_len = offset.checked_add(data.len()).ok_or(())?;
        let write_byte_offset = offset
            .checked_mul(elem_size)
            .and_then(|bytes| WgpuBufferAddress::try_from(bytes).ok())
            .ok_or(())?;
        let write_bytes = bit::try_transmute_slice(data).ok_or_void()?;
        if !self.buffer.usage().contains(GBufferUsageFlags::CopyDest)
            || !write_byte_offset.is_multiple_of(wgpu::COPY_BUFFER_ALIGNMENT)
            || !write_bytes.len().is_multiple_of(wgpu::COPY_BUFFER_ALIGNMENT as usize)
        {
            return Err(());
        }

        let capacity = self.capacity();
        if required_len > capacity {
            self.force_reallocate(required_len.checked_next_power_of_two().ok_or(())?)?;
        } else if Arc::get_mut(&mut self.buffer.buffer).is_none() {
            self.force_reallocate(capacity)?;
        }

        Gpu.queue().write_buffer(&self.buffer.buffer, write_byte_offset, write_bytes);

        if required_len > self.len {
            self.len = required_len;
        }

        Ok(())
    }

    fn force_reallocate(&mut self, new_capacity: usize) -> Result<(), ()> {
        let elem_size = std::mem::size_of::<T>();

        let new_byte_size = new_capacity
            .checked_mul(elem_size)
            .and_then(|b| WgpuBufferAddress::try_from(b).ok())
            .and_then(|b| b.checked_next_multiple_of(wgpu::COPY_BUFFER_ALIGNMENT))
            .ok_or(())?;

        let copy_byte_len = (self.len * elem_size) as WgpuBufferAddress;
        let copy_byte_len = copy_byte_len.checked_next_multiple_of(wgpu::COPY_BUFFER_ALIGNMENT).ok_or(())?;
        if copy_byte_len > self.buffer.buffer.size() {
            return Err(());
        }

        let usage = self.buffer.usage();

        if self.len > 0 && (!usage.contains(GBufferUsageFlags::CopySource) || !usage.contains(GBufferUsageFlags::CopyDest)) {
            return Err(());
        }

        let new_wgpu_buffer = Gpu.device().create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: new_byte_size,
            usage: usage.into(),
            mapped_at_creation: false,
        });

        if self.len > 0 {
            let mut encoder = Gpu.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("gpu_vec_reallocate"),
            });

            encoder.copy_buffer_to_buffer(&self.buffer.buffer, 0, &new_wgpu_buffer, 0, copy_byte_len);

            Gpu.queue().submit(Some(encoder.finish()));
            Gpu.wait();
        }

        self.buffer.buffer = Arc::new(new_wgpu_buffer);

        Ok(())
    }

    pub fn reset(&mut self, data: &[T]) {
        self.update_part(0, data);
        self.len = data.len();
    }
}

pub trait ToGpuVec<T>
where
    T: BitAllUsed,
{
    fn to_gpu_vec(self, desc: GBufferUsageFlags) -> GVec<T>;
}
impl<T> ToGpuVec<T> for &[T]
where
    T: BitAllUsed,
{
    fn to_gpu_vec(self, desc: GBufferUsageFlags) -> GVec<T> {
        GVec::new(self, desc)
    }
}

#[cfg(all(test, feature = "gpu_backend_noop"))]
mod tests {
    use super::*;
    use crate::buffer::buffer::tests::init_gpu;

    #[test]
    fn clone_shares_allocation_until_queue_update() {
        init_gpu();
        let original = GVec::new(&[10u32, 20, 30, 40], GBufferUsage::Copy.into());
        let mut clone = original.clone();
        assert!(Arc::ptr_eq(&original.buffer.buffer, &clone.buffer.buffer));

        clone.try_update_part(1, &[99]).unwrap();
        assert!(!Arc::ptr_eq(&original.buffer.buffer, &clone.buffer.buffer));
        assert_eq!(clone.capacity(), original.capacity());
        assert_eq!(clone.len(), original.len());

        let allocation = Arc::as_ptr(&clone.buffer.buffer);
        clone.try_update_part(2, &[100]).unwrap();
        assert_eq!(Arc::as_ptr(&clone.buffer.buffer), allocation);
        Gpu.queue().submit([]);
        Gpu.wait();
    }
    #[test]
    fn shared_queue_update_copies_an_aligned_prefix() {
        init_gpu();
        let original = GVec::new(&[10u16, 20, 30], GBufferUsage::Copy.into());
        let mut clone = original.clone();

        assert!(clone.try_update_part(1, &[99]).is_err());
        assert!(Arc::ptr_eq(&original.buffer.buffer, &clone.buffer.buffer));

        clone.try_update_part(0, &[90, 91]).unwrap();
        assert!(!Arc::ptr_eq(&original.buffer.buffer, &clone.buffer.buffer));
        assert_eq!(clone.len(), 3);
        assert_eq!(clone.capacity(), 4);
        Gpu.queue().submit([]);
        Gpu.wait();
    }
}
