use super::*;

pub type GpuBufferAddress = wgpu::BufferAddress;
// Todo: impl the range operator ?

pub trait GpuBufferNew<T>
{
    fn new(value: &[T], desc: GpuBufferDesc) -> Self;
    fn with_capacity(capacity: usize, desc: GpuBufferDesc) -> Self;
}
pub trait GpuBufferByte
{
    fn bytes_len(&self) -> usize { self.wgpu_bytes_len() as _ }
    fn wgpu_bytes_len(&self) -> GpuBufferAddress;

    fn bytes_capacity(&self) -> usize { self.wgpu_bytes_capacity() as _ }
    fn wgpu_bytes_capacity(&self) -> GpuBufferAddress { self.wgpu_bytes_len() }
}


pub trait GpuBufferAsWgpuSlice
{
    fn as_wgpu_slice(&self) -> wgpu::BufferSlice<'_>;
}

pub trait GpuAsUntypedSlice
{
    fn untyped_slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuUntypedSlice<'_>;

    fn untyped_update<T>(&mut self, values: &[T]) where T: BitAllUsed { self.try_untyped_update(values).expect("failed to update the gpu buffer") }
    fn try_untyped_update<T>(&mut self, values: &[T]) -> Result<(),()> where T: BitAllUsed
    {
        self.untyped_slice(..).update(values)
    }
}

pub trait GpuBufferRead<T> : GpuBufferByte
{
    fn len(&self) -> usize { self.wgpu_len() as _ }
    fn wgpu_len(&self) -> GpuBufferAddress { self.wgpu_bytes_len() / std::mem::size_of::<T>() as GpuBufferAddress }

    fn capacity(&self) -> usize { self.wgpu_capacity() as _ }
    fn wgpu_capacity(&self) -> GpuBufferAddress { self.wgpu_bytes_capacity() / std::mem::size_of::<T>() as GpuBufferAddress }

    fn read(&self) -> GpuResult<Vec<T>> where T: BitZero + BitPattern
    {
        let mut v = Vec::new();
        self.read_in(&mut v)?;
        Ok(v)
    }
    fn read_in(&self, vec: &mut Vec<T>) -> GpuResult where T: BitZero + BitPattern;

    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSlice<'_,T>;

    fn update(&mut self, values: &[T]) where T: BitAllUsed { self.try_update(values).expect("failed to update the gpu buffer") }
    fn try_update(&mut self, values: &[T]) -> Result<(),()> where T: BitAllUsed
    {
        self.slice(..).update(values)
    }
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
impl GpuBufferByte for wgpu::Buffer
{
    fn wgpu_bytes_len(&self) -> GpuBufferAddress { self.size() }
}
impl GpuAsUntypedSlice for wgpu::Buffer
{
    fn untyped_slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuUntypedSlice<'_> {
        let bytes_len = self.size();

        let start = match bounds.start_bound() {
            Bound::Included(&v) => v as GpuBufferAddress,
            Bound::Excluded(&v) => (v + 1) as GpuBufferAddress,
            Bound::Unbounded => 0,
        }
        .min(bytes_len);

        let end = match bounds.end_bound() {
            Bound::Included(&v) => (v + 1) as GpuBufferAddress,
            Bound::Excluded(&v) => v as GpuBufferAddress,
            Bound::Unbounded => bytes_len,
        }
        .min(bytes_len);

        assert!(
            start <= end,
            "GpuUntypedSlice::untyped_slice: invalid range"
        );

        GpuUntypedSlice {
            buffer: self,
            offset: start,
            size: end - start,
        }
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

impl<'a> GpuBufferByte for wgpu::BufferSlice<'a>
{
    fn wgpu_bytes_len(&self) -> GpuBufferAddress {
        self.size().get()
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
        todo!()
    }
}


/*
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
    }.min(bytes_len);

    start..end
}
    */


impl<T> GpuBufferRead<T> for wgpu::Buffer
{
    fn read_in(&self, mut vec: &mut Vec<T>) -> GpuResult
        where T: BitZero + BitPattern
    {
        self.slice(..).read_in(vec)
    }

    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSlice<'_,T>
    {
        let untyped_slice = GpuUntypedSlice
        {
            buffer: self,
            offset: 0,
            size: self.size(),
        };
        let slice: GpuSlice<'_, T> = untyped_slice.slice(bounds);
        unsafe { GpuSlice::from_raw_parts(self, slice.offset, slice.size) }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GpuSlice<'a,T>
{
    buffer: &'a wgpu::Buffer,
    offset: GpuBufferAddress,
    size: GpuBufferAddress,
    phantom: PhantomData<T>
}
impl<'a,T> From<GpuSlice<'a,T>> for GpuUntypedSlice<'a>
{
    fn from(value: GpuSlice<'a,T>) -> Self {
        let size = std::mem::size_of::<T>() as GpuBufferAddress;
        unsafe { GpuUntypedSlice::from_raw_parts(value.buffer, value.offset * size, value.size * size) }
    }
}
impl<'a,T> GpuSlice<'a,T>
{
    pub unsafe fn from_raw_parts(buffer: &'a wgpu::Buffer, offset: GpuBufferAddress, size: GpuBufferAddress) -> Self
    {
        Self { buffer, offset, size, phantom: PhantomData }
    }

    pub unsafe fn from_untyped_slice_unchecked(slice : GpuUntypedSlice<'a>) -> Self
    {
        let size = std::mem::size_of::<T>().max(1) as GpuBufferAddress;
        unsafe { Self::from_raw_parts(slice.buffer, slice.offset / size, slice.size / size) }
    }

    fn update(&self, values: &[T]) -> Result<(), ()> where T: BitAllUsed
    {
        let untyped: GpuUntypedSlice<'_> = (*self).into();

        let elem_size = std::mem::size_of::<T>();
        if (values.len() as GpuBufferAddress) > self.size {
            return Err(()); // overflow
        }
        untyped.update(values)
    }
}
impl<'a,T> GpuBufferAsWgpuSlice for GpuSlice<'a,T>
{
    fn as_wgpu_slice(&self) -> wgpu::BufferSlice<'a>
    {
        let size = std::mem::size_of::<T>() as GpuBufferAddress;
        self.buffer.slice((self.offset * size)..((self.offset + self.size) * size))
    }
}

// TODO: impl TryFrom... same for the rest of the code in this file
impl<'a,T> From<&'a wgpu::Buffer> for GpuSlice<'a,T>
    where T: BitAnyPattern
{
    fn from(buffer: &'a wgpu::Buffer) -> Self {
        assert_eq!(buffer.bytes_len() % std::mem::size_of::<T>(), 0, "wrong size");
        unsafe { Self::from_untyped_slice_unchecked(GpuUntypedSlice::from(buffer)) }
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GpuUntypedSlice<'a>
{
    buffer: &'a wgpu::Buffer,
    offset: GpuBufferAddress,
    size: GpuBufferAddress,
}
impl<'a> GpuUntypedSlice<'a>
{
    pub unsafe fn from_raw_parts(buffer: &'a wgpu::Buffer, offset: GpuBufferAddress, size: GpuBufferAddress) -> Self
    {
        Self { buffer, offset, size }
    }

    fn update<T>(&self, values: &[T]) -> Result<(), ()> where T: BitAllUsed,
    {
        let value_bytes = values.len() * size_of::<T>();
        if value_bytes as GpuBufferAddress > self.size {
            return Err(()); // overflow: values won't fit
        }

        let staging = Gpu.wgpu.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("update_bytes staging"),
            contents: bit::try_transmute_slice(values).map_err(|_| ())?,
            usage: wgpu::BufferUsages::COPY_SRC,
        });

        let mut encoder = Gpu.wgpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("update_bytes encoder"),
        });

        encoder.copy_buffer_to_buffer(
            &staging,
            0,
            self.buffer,
            self.offset,
            value_bytes as GpuBufferAddress,
        );

        Gpu.wgpu.queue.submit(Some(encoder.finish()));

        Ok(())
    }
}
impl<'a> GpuBufferAsWgpuSlice for GpuUntypedSlice<'a>
{
    fn as_wgpu_slice(&self) -> wgpu::BufferSlice<'a>
    {
        self.buffer.slice(self.offset..self.offset + self.size)
    }
}

impl<'a> From<&'a wgpu::Buffer> for GpuUntypedSlice<'a>
{
    fn from(buffer: &'a wgpu::Buffer) -> Self {
        Self { buffer, offset: 0, size: buffer.size() }
    }
}
impl<'a> GpuBufferByte for GpuUntypedSlice<'a>
{
    fn wgpu_bytes_len(&self) -> GpuBufferAddress { self.size }
}
impl<'a> GpuAsUntypedSlice for GpuUntypedSlice<'a>
{
    fn untyped_slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuUntypedSlice<'_> {
        let start = match bounds.start_bound() {
            Bound::Included(&v) => v as GpuBufferAddress,
            Bound::Excluded(&v) => (v + 1) as GpuBufferAddress,
            Bound::Unbounded => 0,
        };

        let end = match bounds.end_bound() {
            Bound::Included(&v) => (v + 1) as GpuBufferAddress,
            Bound::Excluded(&v) => v as GpuBufferAddress,
            Bound::Unbounded => self.size,
        };

        assert!(
            start <= end && end <= self.size,
            "GpuUntypedSlice::untyped_slice out of bounds"
        );

        GpuUntypedSlice {
            buffer: self.buffer,
            offset: self.offset + start,
            size: end - start,
        }
    }
}
impl<'a, T> GpuBufferRead<T> for GpuUntypedSlice<'a>
{
    fn read_in(&self, vec: &mut Vec<T>) -> GpuResult where T: BitZero + BitPattern,
    {
        let elem_size = std::mem::size_of::<T>().max(1);
        let elem_count = (self.size / elem_size as GpuBufferAddress) as usize;
        if vec.len() < elem_count {
            vec.resize(elem_count, T::zeroed());
        }

        let staging = create_staging_and_copy_slice(&self.as_wgpu_slice());

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

    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuSlice<'_, T> {
        let elem_size = std::mem::size_of::<T>().max(1) as GpuBufferAddress;
        let bytes_len = self.size;

        let start = match bounds.start_bound() {
            std::ops::Bound::Included(&v) => (v as GpuBufferAddress) * elem_size,
            std::ops::Bound::Excluded(&v) => ((v + 1) as GpuBufferAddress) * elem_size,
            std::ops::Bound::Unbounded => 0,
        }
        .min(bytes_len);

        let end = match bounds.end_bound() {
            std::ops::Bound::Included(&v) => ((v + 1) as GpuBufferAddress) * elem_size,
            std::ops::Bound::Excluded(&v) => (v as GpuBufferAddress) * elem_size,
            std::ops::Bound::Unbounded => bytes_len,
        }
        .min(bytes_len);

        let size = end - start;

        assert!(
            start <= end && size % elem_size == 0,
            "GpuSlice<T>::slice: requested range not aligned with element size"
        );

        let untyped = GpuUntypedSlice {
            buffer: self.buffer,
            offset: self.offset + start,
            size,
        };

        unsafe { GpuSlice::from_untyped_slice_unchecked(untyped) }
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
impl GpuBufferByte for GpuUntypedBuffer
{
    fn wgpu_bytes_len(&self) -> GpuBufferAddress { self.wgpu.wgpu_bytes_len() }
}
impl GpuAsUntypedSlice for GpuUntypedBuffer
{
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

impl<T> GpuBufferByte for GpuBuffer<T>
{
    fn wgpu_bytes_len(&self) -> GpuBufferAddress { self.wgpu.wgpu_bytes_len() }
}
impl<T> GpuAsUntypedSlice for GpuBuffer<T>
{
    fn untyped_slice<S: RangeBounds<usize>>(&self, bounds: S) -> GpuUntypedSlice<'_> { self.wgpu.untyped_slice(bounds) }
    fn try_untyped_update<T2>(&mut self, values: &[T2]) -> Result<(),()> where T2: BitAllUsed {
        if std::mem::size_of::<T>() != std::mem::size_of::<T2>() { return Err(()); }
        self.untyped_slice(..).update(values)
    }
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
#[bit_index]
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