use super::*;

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct GpuVec<T>
{
    pub(crate) buffer: GpuBuffer<T>,
    pub(crate) len: usize,
    pub(crate) desc: GpuBufferDesc,
}
impl<T> GpuVec<T>
where
    T: BitAllUsed,
{
    pub fn update_part(&mut self, offset: usize, data: &[T])
    {
        self.try_update_part(offset, data)
            .expect("failed to update the gpu vec");
    }
    pub fn try_update_part(&mut self, offset: usize, data: &[T]) -> Result<(), ()>
    where
        T: BitAllUsed,
    {
        let elem_size = std::mem::size_of::<T>().max(1);
        let required_len = offset.checked_add(data.len()).ok_or(())?;

        if required_len > self.capacity()
        {
            let new_capacity = required_len.next_power_of_two();
            let new_byte_size = (new_capacity * elem_size) as wgpu::BufferAddress;

            let new_buffer = Gpu.wgpu.device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: new_byte_size,
                usage: self.desc.usages.into(),
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
}

impl<T> Clear for GpuVec<T>
{
    fn clear(&mut self) { self.len = 0; }
}

pub trait ToGpuVec<T>
where
    T: BitAllUsed,
{
    fn to_gpu_vec(self, desc: GpuBufferDesc) -> GpuVec<T>;
}
impl<T> ToGpuVec<T> for &[T]
where
    T: BitAllUsed,
{
    fn to_gpu_vec(self, desc: GpuBufferDesc) -> GpuVec<T> { GpuVec::new(self, desc) }
}

impl<T> GpuBufferNew<T> for GpuVec<T>
where
    GpuBuffer<T>: GpuBufferNew<T>,
{
    fn new(value: &[T], desc: GpuBufferDesc) -> Self
    {
        let len = value.len();
        Self {
            buffer: GpuBuffer::new(value, desc),
            len,
            desc,
        }
    }

    fn with_capacity(capacity: usize, desc: GpuBufferDesc) -> Self
    {
        Self {
            buffer: GpuBuffer::with_capacity(capacity, desc),
            len: capacity,
            desc,
        }
    }
}
impl<T> GpuBufferByte for GpuVec<T>
where
    GpuBuffer<T>: GpuBufferByte,
{
    fn wgpu_bytes_len(&self) -> GpuBufferAddress
    {
        (self.len * std::mem::size_of::<T>()) as GpuBufferAddress
    }
    fn wgpu_bytes_capacity(&self) -> GpuBufferAddress { self.buffer.wgpu_bytes_capacity() }
}
impl<T> GpuAsUntypedSlice for GpuVec<T>
where
    GpuBuffer<T>: GpuBufferByte,
{
    fn untyped_slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuUntypedSlice<'_>
    {
        self.buffer.untyped_slice(bounds)
    }
    fn try_untyped_update<T2>(&mut self, values: &[T2]) -> Result<(), ()>
    where
        T2: BitAllUsed,
    {
        self.buffer.try_untyped_update(values)
    }
}
impl<T> GpuBufferRead<T> for GpuVec<T>
where
    wgpu::Buffer: GpuBufferRead<T>,
{
    fn read_in(&self, vec: &mut Vec<T>) -> GpuResult
    where
        T: BitZero + BitPattern,
    {
        self.buffer.read_in(vec)
    }

    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSlice<'_, T>
    {
        self.buffer.slice(bounds)
    }
    fn try_update(&mut self, values: &[T]) -> Result<(), ()>
    where
        T: BitAllUsed,
    {
        self.try_update_part(0, values)
    }
}
