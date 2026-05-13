use super::*;

#[derive(Clone, Debug)]
pub struct GpuVec<T>
    where T: GpuBufferElement
{
    pub(crate) buffer: GpuBuffer<T>,
    pub(crate) len: usize,
}

impl<T> Collection for GpuVec<T> where T: GpuBufferElement {}
impl<T> Length for GpuVec<T> where T: GpuBufferElement
{
    fn len(&self) -> usize { self.len }
}
impl<T> Capacity for GpuVec<T> where T: GpuBufferElement
{
    fn capacity(&self) -> usize {
        self.buffer.len()
    }
}

impl<T> GpuBufferNew<T> for GpuVec<T>
    where T: BitAllUsed
{
    fn new(value: &[T], usage: GpuBufferUsageFlags) -> Self {
        Self { buffer: GpuBuffer::new(value, usage), len: value.len() }
    }
    
    fn with_capacity(capacity: usize, usage: GpuBufferUsageFlags) -> Self {
        WithCapacity::with_capacity_and_param(capacity, usage)
    }
}

impl<T> WithCapacity for GpuVec<T> where T: BitAllUsed
{
    type Param = GpuBufferUsageFlags;
    
    fn with_capacity_and_param(capacity: usize, param: Self::Param) -> Self {
        Self { buffer: GpuBuffer::with_capacity_and_param(capacity, param), len: 0 }
    }
}

impl<T> Clear for GpuVec<T>
    where T: GpuBufferElement
{
    fn clear(&mut self) { self.len = 0; }
}


impl<T> Reserve for GpuVec<T> where T: GpuBufferElement
{
    fn reserve(&mut self, additional: usize) {
        todo!()
    }

    fn reserve_exact(&mut self, additional: usize) {
        todo!()
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> {
        todo!()
    }

    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> {
        todo!()
    }
}

impl<T> WgpuSliceable<T> for GpuVec<T>
    where T: GpuBufferElement
{
    fn wgpu_usage(&self) -> WgpuBufferUsage {
        WgpuSliceable::<T>::wgpu_usage(&self.buffer)
    }

    fn wgpu_slice<S: RangeBounds<WgpuBufferAddress>>(&self, bounds: S) -> WgpuBufferSlice<'_> {
        WgpuSliceable::<T>::wgpu_slice(&self.buffer, bounds)
    }

    fn wgpu_as_slice(&self) -> WgpuBufferSlice<'_> {
        WgpuSliceable::<T>::wgpu_as_slice(&self.buffer)
    }

    fn wgpu_view(&self) -> WgpuBufferView<'_> {
        WgpuSliceable::<T>::wgpu_view(&self.buffer)
    }

    fn wgpu_view_mut(&self) -> WgpuBufferViewMut<'_> {
        WgpuSliceable::<T>::wgpu_view_mut(&self.buffer)
    }
    
    fn wgpu_deep_clone_order(&self) -> WgpuBuffer {
        WgpuSliceable::<T>::wgpu_deep_clone_order(&self.buffer)
    }
}

impl<T> GpuSliceable<T> for GpuVec<T> 
    where T: GpuBufferElement
{
    fn usage(&self) -> GpuBufferUsageFlags { self.buffer.usage() }
    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSlice<'_, T> 
    {
        use std::ops::Bound;
        
        let vec_len = self.len;
        
        let start = match bounds.start_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
            Bound::Unbounded => 0,
        };
        
        let end = match bounds.end_bound() {
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => vec_len,
        };
        
        debug_assert!(
            start <= vec_len,
            "Slice start index {} out of bounds for GpuVec of length {}",
            start, vec_len
        );
        
        debug_assert!(
            end <= vec_len,
            "Slice end index {} out of bounds for GpuVec of length {}",
            end, vec_len
        );
        
        debug_assert!(
            start <= end,
            "Slice start {} must be <= end {}",
            start, end
        );
        
        self.buffer.slice(start..end)
    }

    fn read<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSliceRead<'_, T> {
        let slice = self.slice(bounds);
        self.buffer.read(slice.begin..slice.begin + slice.len())
    }
}

impl<T> GpuSliceableMut<T> for GpuVec<T> 
    where T: GpuBufferElement
{
    fn slice_mut<S: RangeBounds<usize>>(&mut self, bounds: S) -> GpuSliceMut<'_, T> 
    {
        use std::ops::Bound;
        
        let vec_len = self.len;
        
        let start = match bounds.start_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
            Bound::Unbounded => 0,
        };
        
        let end = match bounds.end_bound() {
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => vec_len,
        };
        
        debug_assert!(
            start <= vec_len,
            "Slice start index {} out of bounds for GpuVec of length {}",
            start, vec_len
        );
        
        debug_assert!(
            end <= vec_len,
            "Slice end index {} out of bounds for GpuVec of length {}",
            end, vec_len
        );
        
        debug_assert!(
            start <= end,
            "Slice start {} must be <= end {}",
            start, end
        );
        
        self.buffer.slice_mut(start..end)
    }
}

impl<T> GpuVec<T> where T: BitAllUsed
{
    /*
    pub fn update_part(&mut self, offset: usize, data: &[T])
    {
        self.try_update_part(offset, data)
            .expect("failed to update the gpu vec");
    }
    pub fn try_update_part(&mut self, offset: usize, data: &[T]) -> Result<(), ()>
    {
        let elem_size = std::mem::size_of::<T>().max(1);
        let required_len = offset.checked_add(data.len()).ok_or(())?;

        if required_len > self.capacity()
        {
            let new_capacity = required_len.next_power_of_two();
            let new_byte_size = (new_capacity * elem_size) as wgpu::BufferAddress;

            let new_buffer = device().create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: new_byte_size,
                usage: self.usage,
                mapped_at_creation: false,
            });

            if self.len > 0
            {
                if !self.desc.usages.contains(BufferUsageFlags::CopySrc)
                    || !self.desc.usages.contains(BufferUsageFlags::CopyDst)
                {
                    return Err(());
                }

                let copy_byte_len = (self.len * elem_size) as wgpu::BufferAddress;

                let mut encoder = Gpu
                    .wgpu
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                encoder.copy_buffer_to_buffer(&self.buffer.wgpu, 0, &new_buffer, 0, copy_byte_len);

                Gpu.wgpu.queue.submit(Some(encoder.finish()));
            }

            self.buffer.wgpu = new_buffer;
        }

        let write_byte_offset = (offset * elem_size) as wgpu::BufferAddress;

        let write_bytes = bit::try_transmute_slice(data).map_err(|e| ())?;

        Gpu.wgpu
            .queue
            .write_buffer(&self.buffer.wgpu, write_byte_offset, write_bytes);

        if required_len > self.len
        {
            self.len = required_len;
        }

        Ok(())
    }
    */
}


