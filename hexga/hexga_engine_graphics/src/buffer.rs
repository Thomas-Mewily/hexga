use wgpu::util::DeviceExt;

use super::*;

pub type BufferAddress = wgpu::BufferAddress;

pub trait BufferCommon
{
    fn bytes_len(&self) -> usize { self.wgpu_bytes_len() as _ }
    fn bytes_capacity(&self) -> usize { self.wgpu_bytes_capacity() as _ }

    fn wgpu_bytes_capacity(&self) -> BufferAddress;
    fn wgpu_bytes_len(&self) -> BufferAddress;

    fn as_wgpu_buffer(&self) -> &wgpu::Buffer;
}
impl BufferCommon for wgpu::Buffer
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
impl BufferCommon for UntypedBuffer
{
    fn wgpu_bytes_len(&self) -> BufferAddress { self.buffer.wgpu_bytes_len() }
    fn wgpu_bytes_capacity(&self) -> BufferAddress { self.buffer.wgpu_bytes_len() }
    fn as_wgpu_buffer(&self) -> &wgpu::Buffer { self.buffer.as_wgpu_buffer() }
}
impl<T> BufferReadWith<T> for UntypedBuffer
{
    fn read_in_with(&self, vec: &mut Vec<T>, executor: ExecutorRef<'_>) -> GpuResult {
        todo!()
    }
}




#[derive(Clone, Debug)]
pub struct GpuUntypedBuffer
{
    pub buffer: UntypedBuffer,
    pub executor: Executor,
}
impl<'a> Has<'a,ExecutorRef<'a>> for GpuUntypedBuffer
{
    fn retrieve(&'a self) -> ExecutorRef<'a> {
        self.executor.as_ref()
    }
}
impl BufferCommon for GpuUntypedBuffer
{
    fn wgpu_bytes_len(&self) -> BufferAddress { self.buffer.wgpu_bytes_len() }
    fn as_wgpu_buffer(&self) -> &wgpu::Buffer { self.buffer.as_wgpu_buffer() }
    fn wgpu_bytes_capacity(&self) -> BufferAddress { self.buffer.wgpu_bytes_capacity() }
}
impl GpuUntypedBuffer
{
    pub fn with_size(size: usize, usage: BufferUsageFlags, executor: Executor) -> Self {
        let buffer = executor.device.create_buffer(&wgpu::BufferDescriptor
            {
            label: None,
            size : size as _,
            usage: usage.into(),
            mapped_at_creation: false,
        });

        Self
        {
            buffer: buffer.into(),
            executor,
        }
    }

    pub fn new(contents: &[u8], usage: BufferUsageFlags, executor: Executor) -> Self {
        let buffer = executor.device.create_buffer_init(&wgpu::util::BufferInitDescriptor
            {
                label: None,
                contents,
                usage: usage.into(),
            }
        );

        Self
        {
            buffer: buffer.into(),
            executor,
        }
    }
}

const DEFAULT_BUFFER_READ_CAPACITY : usize = 8 * 512;

/*
pub trait BufferRead<'a,T> : HasGpuExecutor<'a> + BufferReadWith<T>
{
    fn read(&'a self) -> GpuResult<Vec<T>>
    {
        self.read_with(self.executor())
    }
    fn read_in(&'a self, vec: &mut Vec<T>) -> GpuResult
    {
        self.read_in_with(vec, self.executor())
    }
}
impl<'a,T,S> BufferRead<'a,T> for S where S: HasGpuExecutor<'a> + BufferReadWith<T> {}
*/

pub trait BufferReadWith<T> : BufferCommon
{
    fn wgpu_len(&self) -> BufferAddress { self.wgpu_bytes_len() / std::mem::size_of::<T>() as u64 }
    fn wgpu_capacity(&self) -> BufferAddress { self.wgpu_bytes_capacity() / std::mem::size_of::<T>() as u64 }
    fn len(&self) -> BufferAddress { self.wgpu_len() as _ }
    fn capacity(&self) -> BufferAddress { self.wgpu_capacity() as _ }

    fn read_with(&self, executor: ExecutorRef<'_>) -> GpuResult<Vec<T>>
    {
        let mut v = Vec::with_capacity(DEFAULT_BUFFER_READ_CAPACITY);
        self.read_in_with(&mut v, executor)?;
        Ok(v)
    }
    fn read_in_with(&self, vec: &mut Vec<T>, executor: ExecutorRef<'_>) -> GpuResult;
}

fn create_staging_and_copy<'a>(src: &wgpu::Buffer, executor: ExecutorRef<'a>) -> wgpu::Buffer {
    let size = src.size();

    let staging = executor.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("staging buffer"),
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut encoder = executor
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    encoder.copy_buffer_to_buffer(src, 0, &staging, 0, size);
    executor.queue.submit(Some(encoder.finish()));

    staging
}
impl BufferReadWith<u8> for wgpu::Buffer
{
    fn read_in_with(&self, mut vec: &mut Vec<u8>, executor: ExecutorRef<'_>) -> GpuResult
    {
        let size = self.size() as usize;
        if vec.len() < size {
            vec.resize(size, 0);
        }

        let executor = executor.executor();

        let staging = create_staging_and_copy(self, executor.executor());
        let slice = staging.slice(..);

        let status = std::sync::Arc::new(std::sync::Mutex::new(None));
        let status_clone = status.clone();

        slice.map_async(wgpu::MapMode::Read, move |res| {
            *status_clone.lock().unwrap() = Some(res);
        });

        while status.lock().unwrap().is_none() {
            executor.device.poll(wgpu::PollType::Wait);
        }

        match status.lock().unwrap().take().unwrap() {
            Ok(()) => {
                let data = slice.get_mapped_range();
                vec.copy_from_slice(&data);
                drop(data);
                staging.unmap();
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
