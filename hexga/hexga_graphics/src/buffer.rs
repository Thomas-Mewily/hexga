use super::*;

pub type BufferAddress = wgpu::BufferAddress;

pub trait GpuByteBuffer
{
    fn bytes_len(&self) -> usize { self.wgpu_bytes_len() as _ }
    fn wgpu_bytes_len(&self) -> BufferAddress;

    fn bytes_capacity(&self) -> usize { self.wgpu_bytes_capacity() as _ }
    fn wgpu_bytes_capacity(&self) -> BufferAddress;

    fn as_wgpu_buffer(&self) -> &wgpu::Buffer;
}
impl GpuByteBuffer for wgpu::Buffer
{
    fn wgpu_bytes_len(&self) -> BufferAddress { self.size() }
    fn as_wgpu_buffer(&self) -> &wgpu::Buffer { self }
    fn wgpu_bytes_capacity(&self) -> BufferAddress { self.size() }
}


#[derive(Clone, Debug)]
pub struct UntypedBuffer
{
    buffer: wgpu::Buffer,
}
impl Handle for UntypedBuffer {}

impl From<wgpu::Buffer> for UntypedBuffer
{
    fn from(buffer: wgpu::Buffer) -> Self {
        Self { buffer }
    }
}
impl From<UntypedBuffer> for wgpu::Buffer
{
    fn from(value: UntypedBuffer) -> Self {
        value.buffer
    }
}
impl GpuByteBuffer for UntypedBuffer
{
    fn wgpu_bytes_len(&self) -> BufferAddress { self.buffer.wgpu_bytes_len() }
    fn wgpu_bytes_capacity(&self) -> BufferAddress { self.buffer.wgpu_bytes_len() }
    fn as_wgpu_buffer(&self) -> &wgpu::Buffer { self.buffer.as_wgpu_buffer() }
}
impl<T> GpuBufferRead<T> for UntypedBuffer  where wgpu::Buffer: GpuBufferRead<T>
{
    fn read_in(&self, vec: &mut Vec<T>) -> GpuResult {
        self.buffer.read_in(vec)
    }
}



pub trait GpuBufferRead<T> : GpuByteBuffer
{
    fn wgpu_len(&self) -> BufferAddress { self.wgpu_bytes_len() / std::mem::size_of::<T>() as u64 }
    fn wgpu_capacity(&self) -> BufferAddress { self.wgpu_bytes_capacity() / std::mem::size_of::<T>() as u64 }

    fn len(&self) -> BufferAddress { self.wgpu_len() as _ }
    fn capacity(&self) -> BufferAddress { self.wgpu_capacity() as _ }

    fn read(&self) -> GpuResult<Vec<T>>
    {
        let mut v = Vec::new();
        self.read_in(&mut v)?;
        Ok(v)
    }
    fn read_in(&self, vec: &mut Vec<T>) -> GpuResult;
}

fn create_staging_and_copy<'a>(src: &wgpu::Buffer) -> wgpu::Buffer {
    let size = src.size();

    let staging = Gpu.device().create_buffer(&wgpu::BufferDescriptor {
        label: Some("staging buffer"),
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut encoder = Gpu.device().create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    encoder.copy_buffer_to_buffer(src, 0, &staging, 0, size);
    Gpu.queue().submit(Some(encoder.finish()));

    staging
}


impl<T> GpuBufferRead<T> for wgpu::Buffer
    where T: BitZero + BitPattern
{
    fn read_in(&self, mut vec: &mut Vec<T>) -> GpuResult
    {
        let size = self.size() as usize;
        if vec.len() < size {
            vec.resize(size / std::mem::size_of::<T>(), T::zeroed());
        }

        let staging = create_staging_and_copy(self);
        let slice = staging.slice(..);

        let status = std::sync::Arc::new(std::sync::Mutex::new(None));
        let status_clone = status.clone();

        slice.map_async(wgpu::MapMode::Read, move |res| {
            *status_clone.lock().unwrap() = Some(res);
        });

        while status.lock().unwrap().is_none() {
            Gpu.device().poll(wgpu::PollType::Wait);
        }

        match status.lock().unwrap().take().unwrap() {
            Ok(()) => {
                let data = slice.get_mapped_range();
                let typed = bit::transmute_slice(&data);
                vec.copy_from_slice(typed);
                drop(data);
                staging.unmap();
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
