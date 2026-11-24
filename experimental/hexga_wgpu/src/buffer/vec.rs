use super::*;

#[derive(Clone, Debug)]
pub struct GVec<T>
where
    T: GBufferElement,
{
    pub(crate) buffer: GBuffer<T>,
    pub(crate) len: usize,
}

impl<T> Handle for GVec<T> where T: GBufferElement {}

impl<T: GBufferElement> Collection<T> for GVec<T>
{
    type Ref<'a> = GElementRead<'a, T> where Self: 'a, T: 'a;
}

impl<T> CollectionIndex<usize> for GVec<T>
where
    T: GBufferElement,
{
    type Output = T;
    type Ref<'a> = GElementRead<'a, T> where Self: 'a, Self::Output: 'a;
}
impl<T> Get<usize> for GVec<T>
where
    T: GBufferElement,
{
    fn get<'a>(&'a self, index: usize) -> Option<Self::Ref<'a>> where Self::Output: 'a
    {
        if index >= self.len { return None; }
        Some(GElementRead::new(self.buffer.read(..), index))
    }
}

impl<T> Length for GVec<T>
where
    T: GBufferElement,
{
    fn len(&self) -> usize { self.len }
}
impl<T> Capacity<T> for GVec<T>
where
    T: GBufferElement,
{
    fn capacity(&self) -> usize { self.buffer.len() }
}

impl<T> GpuBufferNew<T> for GVec<T>
where
    T: BitAllUsed,
{
    fn new(value: &[T], usage: GBufferUsageFlags) -> Self
    {
        Self {
            buffer: GBuffer::new(value, usage),
            len: value.len(),
        }
    }

    fn with_capacity(capacity: usize, usage: GBufferUsageFlags) -> Self { WithCapacity::with_capacity_and_param(capacity, usage) }
}

impl<T> WithCapacity for GVec<T>
where
    T: BitAllUsed,
{
    type Param = GBufferUsageFlags;

    fn with_capacity_and_param(capacity: usize, param: Self::Param) -> Self
    {
        Self {
            buffer: GBuffer::with_capacity_and_param(capacity, param),
            len: 0,
        }
    }
}

impl<T> Clear for GVec<T>
where
    T: GBufferElement,
{
    fn clear(&mut self) { self.len = 0; }
}

impl<T> Reserve<T> for GVec<T>
where
    T: GBufferElement + BitAllUsed,
{
    type Error = ();

    fn try_reserve(&mut self, additional: usize) -> Result<(), ()>
    {
        let new_capacity = self.len.checked_add(additional).ok_or(())?;

        if new_capacity > self.capacity()
        {
            let new_capacity_pow2 = new_capacity.next_power_of_two();
            self.force_reallocate(new_capacity_pow2)?;
        }
        Ok(())
    }

    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), ()>
    {
        let new_capacity = self.len.checked_add(additional).ok_or(())?;

        if new_capacity > self.capacity()
        {
            self.force_reallocate(new_capacity)?;
        }
        Ok(())
    }
}

impl<T> WgpuSliceable<T> for GVec<T>
where
    T: GBufferElement,
{
    fn wgpu_slice<S: RangeBounds<usize>>(&self, bounds: S) -> WgpuBufferSlice<'_>
    {
        let start_byte = match bounds.start_bound()
        {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
            Bound::Unbounded => 0,
        };

        let end_byte = match bounds.end_bound()
        {
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => self.len(),
        };
        WgpuSliceable::<T>::wgpu_slice(self.wgpu_buffer_ref(), start_byte..end_byte)
    }

    /*
    fn wgpu_deep_clone_order(&self) -> WgpuBuffer
    {
        let size = self.size();
        let usage = WgpuSliceable::<T>::wgpu_usage(self);

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

    fn wgpu_buffer_ref(&self) -> &WgpuBuffer { self.buffer.wgpu_buffer_ref() }
}

impl<T> GpuSliceable<T> for GVec<T>
where
    T: GBufferElement,
{
    fn usage(&self) -> GBufferUsageFlags { self.buffer.usage() }
    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GSlice<'_, T>
    {
        let vec_len = self.len;

        let start = match bounds.start_bound()
        {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
            Bound::Unbounded => 0,
        };

        let end = match bounds.end_bound()
        {
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => vec_len,
        };

        debug_assert!(start <= vec_len, "Slice start index {} out of bounds for GpuVec of length {}", start, vec_len);

        debug_assert!(end <= vec_len, "Slice end index {} out of bounds for GpuVec of length {}", end, vec_len);

        debug_assert!(start <= end, "Slice start {} must be <= end {}", start, end);

        self.buffer.slice(start..end)
    }

    fn read<S: RangeBounds<usize>>(&self, bounds: S) -> GSliceRead<'_, T>
    {
        let slice = self.slice(bounds);
        self.buffer.read(slice.begin..slice.begin + slice.len())
    }
}

impl<T> GpuSliceableMut<T> for GVec<T>
where
    T: GBufferElement,
{
    fn slice_mut<S: RangeBounds<usize>>(&mut self, bounds: S) -> GSliceMut<'_, T>
    {
        let vec_len = self.len;

        let start = match bounds.start_bound()
        {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
            Bound::Unbounded => 0,
        };

        let end = match bounds.end_bound()
        {
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => vec_len,
        };

        debug_assert!(start <= vec_len, "Slice start index {} out of bounds for GpuVec of length {}", start, vec_len);

        debug_assert!(end <= vec_len, "Slice end index {} out of bounds for GpuVec of length {}", end, vec_len);

        debug_assert!(start <= end, "Slice start {} must be <= end {}", start, end);

        self.buffer.slice_mut(start..end)
    }
}

impl<'a, T> Push<&'a [T]> for GVec<T>
where
    T: BitAllUsed,
{
    type Output = ();
    fn push(&mut self, value: &'a [T]) -> Self::Output { self.update_part(self.len(), value); }
}
impl<'a, T> TryPush<&'a [T]> for GVec<T>
where
    T: BitAllUsed,
{
    type Error = ();
    fn try_push(&mut self, value: &'a [T]) -> Result<Self::Output, Self::Error> { self.try_update_part(self.len(), value) }
}

impl<T> GVec<T>
where
    T: BitAllUsed,
{
    pub fn update_part(&mut self, offset: usize, data: &[T]) { self.try_update_part(offset, data).expect("GpuVec failed to update"); }

    pub fn try_update_part(&mut self, offset: usize, data: &[T]) -> Result<(), ()>
    {
        let elem_size = std::mem::size_of::<T>();
        if elem_size <= 0 { return Ok(()); }
        
        let required_len = offset.checked_add(data.len()).ok_or(())?;

        if required_len > self.capacity()
        {
            let new_capacity = required_len.next_power_of_two();
            self.force_reallocate(new_capacity)?;
        }

        let write_byte_offset = (offset * elem_size) as WgpuBufferAddress;
        let write_bytes = bit::try_transmute_slice(data).ok_or_void()?;
        Gpu.queue().write_buffer(&self.buffer.buffer, write_byte_offset, write_bytes);

        if required_len > self.len
        {
            self.len = required_len;
        }

        Ok(())
    }

    fn force_reallocate(&mut self, new_capacity: usize) -> Result<(), ()>
    {
        let elem_size = std::mem::size_of::<T>();

        let new_byte_size = new_capacity
            .checked_mul(elem_size)
            .and_then(|b| WgpuBufferAddress::try_from(b).ok())
            .ok_or(())?;

        let usage = self.buffer.usage();

        if self.len > 0 && (!usage.contains(GBufferUsageFlags::CopySource) || !usage.contains(GBufferUsageFlags::CopyDest))
        {
            return Err(());
        }

        let new_wgpu_buffer = Gpu.device().create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: new_byte_size,
            usage: usage.into(),
            mapped_at_creation: false,
        });

        if self.len > 0
        {
            let copy_byte_len = (self.len * elem_size) as WgpuBufferAddress;
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

    pub fn reset(&mut self, data: &[T])
    {
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
    fn to_gpu_vec(self, desc: GBufferUsageFlags) -> GVec<T> { GVec::new(self, desc) }
}
