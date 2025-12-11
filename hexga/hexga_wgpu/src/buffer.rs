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

    let staging = Gpu.wgpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("staging buffer"),
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut encoder = Gpu.wgpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    encoder.copy_buffer_to_buffer(src, 0, &staging, 0, size);
    Gpu.wgpu.queue.submit(Some(encoder.finish()));

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
            Gpu.wgpu.device.poll(wgpu::PollType::Wait);
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



/// Different ways that you can use a buffer.
///
/// The usages determine what kind of memory the buffer is allocated from and what
/// actions the buffer can partake in.
///
/// Corresponds to [WebGPU `GPUBufferUsageFlags`](
/// https://gpuweb.github.io/gpuweb/#typedefdef-gpubufferusageflags).
#[bitindex]
#[repr(u32)]
pub enum BufferUsage
{
    /// Allow a buffer to be mapped for reading using [`Buffer::map_async`] + [`Buffer::get_mapped_range`].
    /// This does not include creating a buffer with [`BufferDescriptor::mapped_at_creation`] set.
    ///
    /// If [`Features::MAPPABLE_PRIMARY_BUFFERS`] isn't enabled, the only other usage a buffer
    /// may have is COPY_DST.
    MapReap = 0,
    /// Allow a buffer to be mapped for writing using [`Buffer::map_async`] + [`Buffer::get_mapped_range_mut`].
    /// This does not include creating a buffer with `mapped_at_creation` set.
    ///
    /// If [`Features::MAPPABLE_PRIMARY_BUFFERS`] feature isn't enabled, the only other usage a buffer
    /// may have is COPY_SRC.
    MapWrite = 1,
    /// Allow a buffer to be the source buffer for a [`CommandEncoder::copy_buffer_to_buffer`] or [`CommandEncoder::copy_buffer_to_texture`]
    /// operation.
    CopySrc = 2,
    /// Allow a buffer to be the destination buffer for a [`CommandEncoder::copy_buffer_to_buffer`], [`CommandEncoder::copy_texture_to_buffer`],
    /// [`CommandEncoder::clear_buffer`] or [`Queue::write_buffer`] operation.
    CopyDst = 3,
    /// Allow a buffer to be the index buffer in a draw operation.
    Index = 4,
    /// Allow a buffer to be the vertex buffer in a draw operation.
    Vertex = 5,
    /// Allow a buffer to be a [`BufferBindingType::Uniform`] inside a bind group.
    Uniform = 6,
    /// Allow a buffer to be a [`BufferBindingType::Storage`] inside a bind group.
    Storage = 7,
    /// Allow a buffer to be the indirect buffer in an indirect draw call.
    Indirect = 8,
    /// Allow a buffer to be the destination buffer for a [`CommandEncoder::resolve_query_set`] operation.
    QueryResolve = 9,
    /// Allows a buffer to be used as input for a bottom level acceleration structure build
    BlasInput = 10,
    /// Allows a buffer to be used as input for a top level acceleration structure build
    TlasInput = 11,
}
impl BufferUsageFlags
{
    pub const fn from_wgpu(value: wgpu::BufferUsages) -> Self
    {
        unsafe { Self::from_bits_unchecked(value.bits()) }
    }
}
impl From<wgpu::BufferUsages> for BufferUsageFlags
{
    #[inline(always)]
    fn from(value: wgpu::BufferUsages) -> Self
    {
        Self::from_wgpu(value)
    }
}
impl From<BufferUsageFlags> for wgpu::BufferUsages
{
    fn from(value: BufferUsageFlags) -> Self
    {
        Self::from_bits(value.bits()).expect("")
    }
}