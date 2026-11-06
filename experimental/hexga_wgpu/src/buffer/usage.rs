use super::*;

/// Different ways that you can use a buffer.
///
/// The usages determine what kind of memory the buffer is allocated from and what
/// actions the buffer can partake in.
///
/// Corresponds to [WebGPU `GpuBufferUsageFlags`](
/// https://gpuweb.github.io/gpuweb/#typedefdef-GpuBufferUsageFlags).
#[bit_index]
#[repr(u32)]
pub enum GpuBufferUsage
{
    /// Allow a buffer to be mapped for reading using [`Buffer::map_async`] + [`Buffer::get_mapped_range`].
    /// This does not include creating a buffer with [`BufferDescriptor::mapped_at_creation`] set.
    ///
    /// If [`Features::MAPPABLE_PRIMARY_BUFFERS`] isn't enabled, the only other usage a buffer
    /// may have is COPY_DST.
    MapRead = 0,
    /// Allow a buffer to be mapped for writing using [`Buffer::map_async`] + [`Buffer::get_mapped_range_mut`].
    /// This does not include creating a buffer with `mapped_at_creation` set.
    ///
    /// If [`Features::MAPPABLE_PRIMARY_BUFFERS`] feature isn't enabled, the only other usage a buffer
    /// may have is COPY_SRC.
    MapWrite = 1,
    /// Allow a buffer to be the source buffer for a [`CommandEncoder::copy_buffer_to_buffer`] or [`CommandEncoder::copy_buffer_to_texture`]
    /// operation.
    CopySource = 2,
    /// Allow a buffer to be the destination buffer for a [`CommandEncoder::copy_buffer_to_buffer`], [`CommandEncoder::copy_texture_to_buffer`],
    /// [`CommandEncoder::clear_buffer`] or [`Queue::write_buffer`] operation.
    CopyDest = 3,

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

    /// `CopySource | CopyDest`
    /// 
    /// Allow a buffer to be the source buffer for a [`CommandEncoder::copy_buffer_to_buffer`] or [`CommandEncoder::copy_buffer_to_texture`]
    /// operation.
    /// Allow a buffer to be the destination buffer for a [`CommandEncoder::copy_buffer_to_buffer`], [`CommandEncoder::copy_texture_to_buffer`],
    /// [`CommandEncoder::clear_buffer`] or [`Queue::write_buffer`] operation.
    CopySourceDest = Self::CopySource | Self::CopyDest,

    /// `MapRead | MapWrite`
    /// 
    MapReadWrite = Self::MapRead | Self::MapWrite,
}
impl GpuBufferUsageFlags
{
    pub const fn from_wgpu(value: wgpu::BufferUsages) -> Self { unsafe { Self::from_bits_unchecked(value.bits()) } }
}
impl From<wgpu::BufferUsages> for GpuBufferUsageFlags
{
    #[inline(always)]
    fn from(value: wgpu::BufferUsages) -> Self { Self::from_wgpu(value) }
}
impl From<GpuBufferUsageFlags> for wgpu::BufferUsages
{
    fn from(value: GpuBufferUsageFlags) -> Self { Self::from_bits(value.bits()).expect("Oops! WGPU added more usage flags and this code is not updated") }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_wgpu_enum_bit() {
        type WgpuBuffUsage = wgpu::BufferUsages;
        assert_eq!(GpuBufferUsage::MapRead.bits(), WgpuBuffUsage::MAP_READ.bits());
        assert_eq!(GpuBufferUsage::MapWrite.bits(), WgpuBuffUsage::MAP_WRITE.bits());
        assert_eq!(GpuBufferUsage::CopySource.bits(), WgpuBuffUsage::COPY_SRC.bits());
        assert_eq!(GpuBufferUsage::CopyDest.bits(), WgpuBuffUsage::COPY_DST.bits());
        assert_eq!(GpuBufferUsage::Index.bits(), WgpuBuffUsage::INDEX.bits());
        assert_eq!(GpuBufferUsage::Vertex.bits(), WgpuBuffUsage::VERTEX.bits());
        assert_eq!(GpuBufferUsage::Uniform.bits(), WgpuBuffUsage::UNIFORM.bits());
        assert_eq!(GpuBufferUsage::Storage.bits(), WgpuBuffUsage::STORAGE.bits());
        assert_eq!(GpuBufferUsage::Indirect.bits(), WgpuBuffUsage::INDIRECT.bits());
        assert_eq!(GpuBufferUsage::QueryResolve.bits(), WgpuBuffUsage::QUERY_RESOLVE.bits());
        assert_eq!(GpuBufferUsage::BlasInput.bits(), WgpuBuffUsage::BLAS_INPUT.bits());
        assert_eq!(GpuBufferUsage::TlasInput.bits(), WgpuBuffUsage::TLAS_INPUT.bits());
    }
}