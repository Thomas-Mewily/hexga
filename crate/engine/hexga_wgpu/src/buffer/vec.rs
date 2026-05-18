use super::*;

#[derive(Clone, Debug)]
pub struct GpuVec<T>
where
    T: GpuBufferElement,
{
    pub(crate) buffer: GpuBuffer<T>,
    pub(crate) len: usize,
}

impl<T> Collection for GpuVec<T> where T: GpuBufferElement {}
impl<T> Length for GpuVec<T>
where
    T: GpuBufferElement,
{
    fn len(&self) -> usize { self.len }
}
impl<T> Capacity for GpuVec<T>
where
    T: GpuBufferElement,
{
    fn capacity(&self) -> usize { self.buffer.len() }
}

impl<T> GpuBufferNew<T> for GpuVec<T>
where
    T: BitAllUsed,
{
    fn new(value: &[T], usage: GpuBufferUsageFlags) -> Self
    {
        Self {
            buffer: GpuBuffer::new(value, usage),
            len: value.len(),
        }
    }

    fn with_capacity(capacity: usize, usage: GpuBufferUsageFlags) -> Self
    {
        WithCapacity::with_capacity_and_param(capacity, usage)
    }
}

impl<T> WithCapacity for GpuVec<T>
where
    T: BitAllUsed,
{
    type Param = GpuBufferUsageFlags;

    fn with_capacity_and_param(capacity: usize, param: Self::Param) -> Self
    {
        Self {
            buffer: GpuBuffer::with_capacity_and_param(capacity, param),
            len: 0,
        }
    }
}

impl<T> Clear for GpuVec<T>
where
    T: GpuBufferElement,
{
    fn clear(&mut self) { self.len = 0; }
}

impl<T> Reserve for GpuVec<T>
where
    T: GpuBufferElement + BitAllUsed,
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

impl<T> WgpuSliceable<T> for GpuVec<T>
where
    T: GpuBufferElement,
{
    fn wgpu_usage(&self) -> WgpuBufferUsage { WgpuSliceable::<T>::wgpu_usage(&self.buffer) }

    fn wgpu_slice<S: RangeBounds<WgpuBufferAddress>>(&self, bounds: S) -> WgpuBufferSlice<'_>
    {
        WgpuSliceable::<T>::wgpu_slice(&self.buffer, bounds)
    }

    fn wgpu_as_slice(&self) -> WgpuBufferSlice<'_>
    {
        WgpuSliceable::<T>::wgpu_as_slice(&self.buffer)
    }

    fn wgpu_view(&self) -> WgpuBufferView<'_> { WgpuSliceable::<T>::wgpu_view(&self.buffer) }

    fn wgpu_view_mut(&self) -> WgpuBufferViewMut<'_>
    {
        WgpuSliceable::<T>::wgpu_view_mut(&self.buffer)
    }

    fn wgpu_deep_clone_order(&self) -> WgpuBuffer
    {
        WgpuSliceable::<T>::wgpu_deep_clone_order(&self.buffer)
    }
}

impl<T> GpuSliceable<T> for GpuVec<T>
where
    T: GpuBufferElement,
{
    fn usage(&self) -> GpuBufferUsageFlags { self.buffer.usage() }
    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSlice<'_, T>
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

        debug_assert!(
            start <= vec_len,
            "Slice start index {} out of bounds for GpuVec of length {}",
            start,
            vec_len
        );

        debug_assert!(
            end <= vec_len,
            "Slice end index {} out of bounds for GpuVec of length {}",
            end,
            vec_len
        );

        debug_assert!(start <= end, "Slice start {} must be <= end {}", start, end);

        self.buffer.slice(start..end)
    }

    fn read<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSliceRead<'_, T>
    {
        let slice = self.slice(bounds);
        self.buffer.read(slice.begin..slice.begin + slice.len())
    }
}

impl<T> GpuSliceableMut<T> for GpuVec<T>
where
    T: GpuBufferElement,
{
    fn slice_mut<S: RangeBounds<usize>>(&mut self, bounds: S) -> GpuSliceMut<'_, T>
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

        debug_assert!(
            start <= vec_len,
            "Slice start index {} out of bounds for GpuVec of length {}",
            start,
            vec_len
        );

        debug_assert!(
            end <= vec_len,
            "Slice end index {} out of bounds for GpuVec of length {}",
            end,
            vec_len
        );

        debug_assert!(start <= end, "Slice start {} must be <= end {}", start, end);

        self.buffer.slice_mut(start..end)
    }
}

impl<'a, T> Push<&'a [T]> for GpuVec<T>
where
    T: BitAllUsed,
{
    type Output = ();
    fn push(&mut self, value: &'a [T]) -> Self::Output { self.update_part(self.len(), value); }
}
impl<'a, T> TryPush<&'a [T]> for GpuVec<T>
where
    T: BitAllUsed,
{
    type Error = ();
    fn try_push(&mut self, value: &'a [T]) -> Result<Self::Output, Self::Error>
    {
        self.try_update_part(self.len(), value)
    }
}

impl<T> GpuVec<T>
where
    T: BitAllUsed,
{
    pub fn update_part(&mut self, offset: usize, data: &[T])
    {
        self.try_update_part(offset, data)
            .expect("GpuVec failed to update");
    }

    pub fn try_update_part(&mut self, offset: usize, data: &[T]) -> Result<(), ()>
    {
        let elem_size = std::mem::size_of::<T>().max(1);
        let required_len = offset.checked_add(data.len()).ok_or(())?;

        if required_len > self.capacity()
        {
            let new_capacity = required_len.next_power_of_two();
            self.force_reallocate(new_capacity)?;
        }

        let write_byte_offset = (offset * elem_size) as WgpuBufferAddress;
        let write_bytes = bit::try_transmute_slice(data).ok_or_void()?;
        Gpu.queue()
            .write_buffer(&self.buffer.buffer, write_byte_offset, write_bytes);

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

        if self.len > 0
        {
            if !usage.contains(GpuBufferUsageFlags::CopySrc)
                || !usage.contains(GpuBufferUsageFlags::CopyDst)
            {
                return Err(());
            }
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
            let mut encoder =
                Gpu.device()
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("gpu_vec_reallocate"),
                    });

            encoder.copy_buffer_to_buffer(
                &self.buffer.buffer,
                0,
                &new_wgpu_buffer,
                0,
                copy_byte_len,
            );

            Gpu.queue().submit(Some(encoder.finish()));
            Gpu.wait();
        }

        self.buffer.buffer = Arc::new(new_wgpu_buffer);

        Ok(())
    }
}

pub trait ToGpuVec<T>
where
    T: BitAllUsed,
{
    fn to_gpu_vec(self, desc: GpuBufferUsageFlags) -> GpuVec<T>;
}
impl<T> ToGpuVec<T> for &[T]
where
    T: BitAllUsed,
{
    fn to_gpu_vec(self, desc: GpuBufferUsageFlags) -> GpuVec<T> { GpuVec::new(self, desc) }
}