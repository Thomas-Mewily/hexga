use super::*;

pub type VertexFormat = wgpu::VertexFormat;

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