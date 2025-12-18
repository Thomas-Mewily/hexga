use super::*;

pub type GpuBufferAddress = wgpu::BufferAddress;
// Todo: impl the range operator ?

pub trait GpuBufferNew<T>
{
    fn new(value: &[T], desc: GpuBufferDesc) -> Self;
    fn with_capacity(capacity: usize, desc: GpuBufferDesc) -> Self;
}
pub trait GpuByteBuffer
{
    fn bytes_len(&self) -> usize { self.wgpu_bytes_len() as _ }
    fn wgpu_bytes_len(&self) -> GpuBufferAddress;

    fn bytes_capacity(&self) -> usize { self.wgpu_bytes_capacity() as _ }
    fn wgpu_bytes_capacity(&self) -> GpuBufferAddress;

    fn untyped_slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuUntypedSlice<'_>;
}


pub trait GpuBufferRead<T> : GpuByteBuffer
{
    fn wgpu_len(&self) -> GpuBufferAddress { self.wgpu_bytes_len() / std::mem::size_of::<T>() as GpuBufferAddress }
    fn wgpu_capacity(&self) -> GpuBufferAddress { self.wgpu_bytes_capacity() / std::mem::size_of::<T>() as GpuBufferAddress }

    fn len(&self) -> usize { self.wgpu_len() as _ }
    fn capacity(&self) -> usize { self.wgpu_capacity() as _ }

    fn read(&self) -> GpuResult<Vec<T>> where T: BitZero + BitPattern
    {
        let mut v = Vec::new();
        self.read_in(&mut v)?;
        Ok(v)
    }
    fn read_in(&self, vec: &mut Vec<T>) -> GpuResult where T: BitZero + BitPattern;
    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSlice<'_,T>;
}
pub trait GpuBufferWrite<T> : GpuByteBuffer
{

}




impl<T> GpuBufferNew<T> for wgpu::Buffer
    where T: BitAllUsed
{
    fn new(value: &[T], desc: GpuBufferDesc) -> Self {
        Gpu.wgpu.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bit::transmute_slice(value),
            usage: desc.usages,
        })
    }

    fn with_capacity(capacity: usize, desc: GpuBufferDesc) -> Self {
        Gpu.wgpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: desc.usages,
            size: (capacity * std::mem::size_of::<T>()) as _,
            mapped_at_creation: false,
        })
    }
}
impl GpuByteBuffer for wgpu::Buffer
{
    fn wgpu_bytes_len(&self) -> GpuBufferAddress { self.size() }
    fn wgpu_bytes_capacity(&self) -> GpuBufferAddress { self.size() }

    fn untyped_slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuUntypedSlice<'_> {

        let bytes_len = self.wgpu_bytes_len();
        let start = match bounds.start_bound() {
            Bound::Included(&v) => v as GpuBufferAddress,
            Bound::Excluded(&v) => (v + 1) as GpuBufferAddress,
            Bound::Unbounded => 0,
        }.min(bytes_len);

        let end = match bounds.end_bound() {
            Bound::Included(&v) => (v + 1) as GpuBufferAddress,
            Bound::Excluded(&v) => v as GpuBufferAddress,
            Bound::Unbounded => self.bytes_len() as GpuBufferAddress,
        }.max(bytes_len);

        self.slice(start..end).into()
    }
}

fn create_staging_and_copy_slice(slice: &wgpu::BufferSlice) -> wgpu::Buffer {
    let size = slice.size().get();

    let staging = Gpu.wgpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("staging buffer"),
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut encoder =
        Gpu.wgpu.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    encoder.copy_buffer_to_buffer(
        slice.buffer(),
        slice.offset(),
        &staging,
        0,
        size,
    );

    Gpu.wgpu.queue.submit(Some(encoder.finish()));
    staging
}

impl<'a> GpuByteBuffer for wgpu::BufferSlice<'a>
{
    fn wgpu_bytes_len(&self) -> GpuBufferAddress {
        self.size().get()
    }

    fn wgpu_bytes_capacity(&self) -> GpuBufferAddress {
        self.size().get()
    }

    fn untyped_slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuUntypedSlice<'_> {
        let bytes_len = self.wgpu_bytes_len();
        let start = match bounds.start_bound() {
            Bound::Included(&v) => v as GpuBufferAddress,
            Bound::Excluded(&v) => (v + 1) as GpuBufferAddress,
            Bound::Unbounded => 0,
        }.min(bytes_len);

        let end = match bounds.end_bound() {
            Bound::Included(&v) => (v + 1) as GpuBufferAddress,
            Bound::Excluded(&v) => v as GpuBufferAddress,
            Bound::Unbounded => self.bytes_len() as GpuBufferAddress,
        }.max(bytes_len);

        self.slice(start..end).into()
    }
}
impl<'a,T> GpuBufferRead<T> for wgpu::BufferSlice<'a>
{
    fn read_in(&self, vec: &mut Vec<T>) -> GpuResult
        where T: BitZero + BitPattern
    {
        let byte_len = self.size().get() as GpuBufferAddress as usize;
        let elem_count = byte_len / std::mem::size_of::<T>();

        if vec.len() < elem_count {
            vec.resize(elem_count, T::zeroed());
        }

        let staging = create_staging_and_copy_slice(self);
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
                vec[..typed.len()].copy_from_slice(typed);
                drop(data);
                staging.unmap();
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }

    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSlice<'_,T> {
        GpuSlice::from_wgpu_slice(self.slice(get_range::<S,T>(bounds, self.bytes_len())))
    }
}

fn get_range<S: RangeBounds<usize>,T>(bounds: S, bytes_len: usize) -> std::ops::Range<u64>
{
    let elem_size = std::mem::size_of::<T>()
        .max(1); // To handle Zero Sized Type
    let bytes_len = (bytes_len / elem_size * elem_size) as GpuBufferAddress;
    let size = std::mem::size_of::<T>() as GpuBufferAddress;

    let start = match bounds.start_bound() {
        Bound::Included(&v) => v as GpuBufferAddress * size,
        Bound::Excluded(&v) => (v + 1) as GpuBufferAddress * size,
        Bound::Unbounded => 0,
    }.min(bytes_len);

    let end = match bounds.end_bound() {
        Bound::Included(&v) => (v + 1) as GpuBufferAddress * size,
        Bound::Excluded(&v) => v as GpuBufferAddress * size,
        Bound::Unbounded => bytes_len,
    }.max(bytes_len);

    start..end
}


impl<T> GpuBufferRead<T> for wgpu::Buffer
{
    fn read_in(&self, mut vec: &mut Vec<T>) -> GpuResult
        where T: BitZero + BitPattern
    {
        self.slice(..).read_in(vec)
    }

    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSlice<'_,T> {
        GpuSlice::from_wgpu_slice(self.slice(get_range::<S,T>(bounds, self.bytes_len())))
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GpuSlice<'a,T>
{
    slice : wgpu::BufferSlice<'a>,
    phantom: PhantomData<T>
}
impl<'a,T> GpuSlice<'a,T>
{
    // The caller check if it is valid
    pub(crate) const fn from_wgpu_slice(slice : wgpu::BufferSlice<'a>) -> Self { Self { slice, phantom: PhantomData }}
}

// TODO: impl TryFrom... same for the rest of the code in this file
impl<'a,T> From<wgpu::BufferSlice<'a>> for GpuSlice<'a,T>
    where T: BitAnyPattern
{
    fn from(slice: wgpu::BufferSlice<'a>) -> Self {
        assert_eq!(slice.bytes_len() % std::mem::size_of::<T>(), 0, "wrong size");
        Self { slice, phantom: PhantomData }
    }
}
impl<'a,T> From<GpuSlice<'a,T>> for wgpu::BufferSlice<'a>
{
    fn from(value: GpuSlice<'a,T>) -> Self {
        value.slice
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GpuUntypedSlice<'a>
{
    slice : wgpu::BufferSlice<'a>
}
impl<'a> From<wgpu::BufferSlice<'a>> for GpuUntypedSlice<'a>
{
    fn from(slice: wgpu::BufferSlice<'a>) -> Self {
        Self { slice }
    }
}
impl<'a> From<GpuUntypedSlice<'a>> for wgpu::BufferSlice<'a>
{
    fn from(value: GpuUntypedSlice<'a>) -> Self {
        value.slice
    }
}
impl<'a> GpuByteBuffer for GpuUntypedSlice<'a>
{
    fn wgpu_bytes_len(&self) -> GpuBufferAddress {
        self.slice.size().get()
    }
    fn wgpu_bytes_capacity(&self) -> GpuBufferAddress {
        self.slice.size().get()
    }

    fn untyped_slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuUntypedSlice<'_> {
        self.slice.untyped_slice(bounds)
    }
}

#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Hash)]
pub struct GpuUntypedBuffer
{
    pub(crate) wgpu: wgpu::Buffer,
    private_constructor: (),
}
impl Handle for GpuUntypedBuffer {}


impl<T> GpuBufferNew<T> for GpuUntypedBuffer where T: BitAllUsed
{
    fn new(value: &[T], desc: GpuBufferDesc) -> Self {
        Self { wgpu: wgpu::Buffer::new(value, desc), private_constructor: () }
    }

    fn with_capacity(capacity: usize, desc: GpuBufferDesc) -> Self {
        Self { wgpu: <wgpu::Buffer as GpuBufferNew<T>>::with_capacity(capacity, desc), private_constructor: () }
    }
}

impl From<wgpu::Buffer> for GpuUntypedBuffer
{
    fn from(buffer: wgpu::Buffer) -> Self {
        Self { wgpu: buffer, private_constructor: () }
    }
}
impl From<GpuUntypedBuffer> for wgpu::Buffer
{
    fn from(value: GpuUntypedBuffer) -> Self {
        value.wgpu
    }
}
impl GpuByteBuffer for GpuUntypedBuffer
{
    fn wgpu_bytes_len(&self) -> GpuBufferAddress { self.wgpu.wgpu_bytes_len() }
    fn wgpu_bytes_capacity(&self) -> GpuBufferAddress { self.wgpu.wgpu_bytes_len() }
    fn untyped_slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuUntypedSlice<'_> { self.wgpu.untyped_slice(bounds) }
}
impl<T> GpuBufferRead<T> for GpuUntypedBuffer where wgpu::Buffer: GpuBufferRead<T>
{
    fn read_in(&self, vec: &mut Vec<T>) -> GpuResult where T: BitZero + BitPattern { self.wgpu.read_in(vec) }
    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSlice<'_,T> { GpuBufferRead::<T>::slice(&self.wgpu, bounds) }
}





#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Hash)]
pub struct GpuBuffer<T>
{
    pub(crate) wgpu: wgpu::Buffer,
    phantom: PhantomData<T>,
}
impl<T> Handle for GpuBuffer<T> where T: BitAllUsed {}


impl<T> GpuBufferNew<T> for GpuBuffer<T> where T: BitAllUsed
{
    fn new(value: &[T], desc: GpuBufferDesc) -> Self {
        Self { wgpu: wgpu::Buffer::new(value, desc), phantom: PhantomData }
    }

    fn with_capacity(capacity: usize, desc: GpuBufferDesc) -> Self {
        Self { wgpu: <wgpu::Buffer as GpuBufferNew<T>>::with_capacity(capacity, desc), phantom: PhantomData }
    }
}

impl<T> From<wgpu::Buffer> for GpuBuffer<T>
    where T: BitAnyPattern
{
    fn from(buffer: wgpu::Buffer) -> Self {
        assert_eq!(buffer.bytes_len() % std::mem::size_of::<T>(), 0, "wrong gpu buffer size");
        Self { wgpu: buffer, phantom: PhantomData }
    }
}
impl<T> From<GpuUntypedBuffer> for GpuBuffer<T>
    where T: BitAnyPattern
{
    fn from(buffer: GpuUntypedBuffer) -> Self {
        Self::from(buffer.wgpu)
    }
}
impl<T> From<GpuBuffer<T>> for wgpu::Buffer
{
    fn from(value: GpuBuffer<T>) -> Self {
        value.wgpu
    }
}
impl<T> From<GpuBuffer<T>> for GpuUntypedBuffer
{
    fn from(value: GpuBuffer<T>) -> Self {
        GpuUntypedBuffer::from(value.wgpu)
    }
}

impl<T> GpuByteBuffer for GpuBuffer<T>
{
    fn wgpu_bytes_len(&self) -> GpuBufferAddress { self.wgpu.wgpu_bytes_len() }
    fn wgpu_bytes_capacity(&self) -> GpuBufferAddress { self.wgpu.wgpu_bytes_len() }
    fn untyped_slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuUntypedSlice<'_> { self.wgpu.untyped_slice(bounds) }
}
impl<T> GpuBufferRead<T> for GpuBuffer<T> where wgpu::Buffer: GpuBufferRead<T>
{
    fn read_in(&self, vec: &mut Vec<T>) -> GpuResult where T: BitZero + BitPattern { self.wgpu.read_in(vec) }
    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSlice<'_,T> { GpuBufferRead::<T>::slice(&self.wgpu, bounds) }
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

pub type GpuBufferUsages = wgpu::BufferUsages;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GpuBufferDesc
{
    pub usages: GpuBufferUsages,
    pub name: Option<&'static str>
}
impl Default for GpuBufferDesc
{
    fn default() -> Self {
        Self::new()
    }
}
impl GpuBufferDesc
{
    pub const fn new() -> Self { Self { usages: GpuBufferUsages::COPY_DST.union(GpuBufferUsages::COPY_SRC), name: None }}

    pub const fn add_usage(mut self, usage : GpuBufferUsages) -> Self { self.usages = self.usages.union(usage); self }
    pub const fn with_usages(mut self, usages : GpuBufferUsages) -> Self { self.usages = usages; self }
    pub const fn with_label(mut self, label : Option<&'static str>) -> Self { self.name = label; self }

    pub const VERTEX : Self = Self::new().add_usage(GpuBufferUsages::VERTEX);
    pub const INDEX : Self = Self::new().add_usage(GpuBufferUsages::INDEX);
}

pub trait ToGpuBuffer<T> where T:BitAllUsed
{
    fn to_gpu_buffer(self, desc: GpuBufferDesc) -> GpuBuffer<T>;
}
impl<T> ToGpuBuffer<T> for &[T] where T:BitAllUsed
{
    fn to_gpu_buffer(self, desc: GpuBufferDesc) -> GpuBuffer<T> {
        GpuBuffer::new(self, desc)
    }
}