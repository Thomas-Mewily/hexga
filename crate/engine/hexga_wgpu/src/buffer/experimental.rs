use super::*;

pub type WgpuBufferAddress = wgpu::BufferAddress;
pub type WgpuBufferUsage = wgpu::BufferUsages;

pub type WgpuBuffer = wgpu::Buffer;
pub type WgpuBufferSlice<'a> = wgpu::BufferSlice<'a>;
pub type WgpuBufferView<'a> = wgpu::BufferView<'a>;
pub type WgpuBufferViewMut<'a> = wgpu::BufferViewMut<'a>;

impl<T> GpuBufferNew<T> for WgpuBuffer
    where T: BitAllUsed + Copy
{
    fn new(value: &[T], usage: GpuBufferUsageFlags) -> Self {
        device()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bit::transmute_slice(value),
                usage: usage.into(),
            })
    }
}

pub trait WgpuSliceable<T> where T: Copy
{
    fn wgpu_usage(&self) -> WgpuBufferUsage;
    fn wgpu_slice<S: RangeBounds<WgpuBufferAddress>>(&self, bounds: S) -> WgpuBufferSlice<'_>;
    fn wgpu_as_slice(&self) -> WgpuBufferSlice<'_>;
    fn wgpu_view(&self) -> WgpuBufferView<'_>;
    fn wgpu_view_mut(&self) -> WgpuBufferViewMut<'_>;

    /// Performs a deep clone of the GPU buffer by submitting a copy command to the GPU.
    /// 
    /// This method only submits the copy command and returns immediately without waiting.
    /// The returned buffer may not contain the copied data until the GPU completes the operation.
    /// Use `wgpu_deep_clone()` for a blocking version that waits for completion.
    ///
    /// # Returns
    /// A new `WgpuBuffer` with the same size and usage, but the contents are not yet guaranteed to be copied.
    fn wgpu_deep_clone_order(&self) -> WgpuBuffer;

    /// Performs a deep clone of the GPU buffer and blocks until the copy is complete.
    ///
    /// This method submits the copy command and waits for GPU execution to finish.
    /// The returned buffer is guaranteed to contain the copied data.
    ///
    /// # Returns
    /// A new `WgpuBuffer` with the same size, usage, and a complete copy of the original buffer's data.
    ///
    /// # Note
    /// This method blocks the current thread. For non-blocking behavior, use `wgpu_deep_clone_order()`.
    fn wgpu_deep_clone(&self) -> WgpuBuffer 
    { 
        let buff = self.wgpu_deep_clone_order(); 
        device().poll(wgpu::PollType::Wait);
        buff
    }
}

impl<T: Copy> GpuSliceableMut<T> for WgpuBuffer  {}

impl<T: Copy> WgpuSliceable<T> for WgpuBuffer {
    fn wgpu_usage(&self) -> WgpuBufferUsage {
        self.usage()
    }
    
    fn wgpu_slice<S: RangeBounds<WgpuBufferAddress>>(&self, bounds: S) -> WgpuBufferSlice<'_> {
        let element_size = std::mem::size_of::<T>() as WgpuBufferAddress;
        
        let start_byte = match bounds.start_bound() {
            Bound::Included(&i) => (i as WgpuBufferAddress) * element_size,
            Bound::Excluded(&i) => ((i + 1) as WgpuBufferAddress) * element_size,
            Bound::Unbounded => 0,
        };
        
        let end_byte = match bounds.end_bound() {
            Bound::Included(&i) => ((i + 1) as WgpuBufferAddress) * element_size,
            Bound::Excluded(&i) => (i as WgpuBufferAddress) * element_size,
            Bound::Unbounded => self.size(),
        };
        
        self.slice(start_byte..end_byte)
    }
    
    fn wgpu_as_slice(&self) -> WgpuBufferSlice<'_> {
        self.slice(..)
    }
    
    fn wgpu_view(&self) -> WgpuBufferView<'_> {
        self.slice(..).get_mapped_range()
    }
    
    fn wgpu_view_mut(&self) -> WgpuBufferViewMut<'_> {
        self.slice(..).get_mapped_range_mut()
    }

    fn wgpu_deep_clone_order(&self) -> WgpuBuffer {
        let size = self.size();
        let usage = self.usage();
        
        let new_buffer = device().create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage,
            mapped_at_creation: false,
        });
        
        let mut encoder = device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });
        
        encoder.copy_buffer_to_buffer(self, 0, &new_buffer, 0, size);
        queue().submit(Some(encoder.finish()));
        
        new_buffer
    }
}
