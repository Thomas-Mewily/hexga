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
impl<T> BufferRead<T> for UntypedBuffer
{
    fn read_in(&self, vec: &mut Vec<T>, executor: ExecutorRef<'_>) -> GpuResult {
        AAAAAAAAAAA
        //self.buffer.read_in(vec, executor);
        //self
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

pub trait BufferRead<T> : BufferCommon
{
    fn wgpu_len(&self) -> BufferAddress { self.wgpu_bytes_len() / std::mem::size_of::<T>() as u64 }
    fn wgpu_capacity(&self) -> BufferAddress { self.wgpu_bytes_capacity() / std::mem::size_of::<T>() as u64 }
    fn len(&self) -> BufferAddress { self.wgpu_len() as _ }
    fn capacity(&self) -> BufferAddress { self.wgpu_capacity() as _ }

    fn read(&self, executor: ExecutorRef<'_>) -> GpuResult<Vec<T>>
    {
        let mut v = Vec::with_capacity(DEFAULT_BUFFER_READ_CAPACITY);
        self.read_in(&mut v, executor)?;
        Ok(v)
    }
    fn read_in(&self, vec: &mut Vec<T>, executor: ExecutorRef<'_>) -> GpuResult;
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

fn cast_u8_slice_to_t_slice<T: Copy>(bytes: &[u8]) -> &[T] {
    let ptr = bytes.as_ptr() as *const T;
    let len = bytes.len() / std::mem::size_of::<T>();
    unsafe { std::slice::from_raw_parts(ptr, len) }
}


// Todo: better check about bit pattern with T (bytemuck or extend the hexga_bit with bytemuck like trait)
impl<T> BufferRead<T> for wgpu::Buffer
    where T: 'static + Copy + BitZero
{
    fn read_in(&self, mut vec: &mut Vec<T>, executor: ExecutorRef<'_>) -> GpuResult
    {
        let size = self.size() as usize;
        if vec.len() < size {
            vec.resize(size / std::mem::size_of::<T>(), T::zeroed());
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
                let typed = cast_u8_slice_to_t_slice::<T>(&data);
                vec.copy_from_slice(typed);
                drop(data);
                staging.unmap();
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
