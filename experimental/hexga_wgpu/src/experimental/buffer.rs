use super::*;

pub type WgpuBufferAddress = wgpu::BufferAddress;
pub type WgpuBufferUsage = wgpu::BufferUsages;

pub type WgpuBuffer = wgpu::Buffer;
pub type WgpuBufferSlice<'a> = wgpu::BufferSlice<'a>;
pub type WgpuBufferView<'a> = wgpu::BufferView<'a>;
pub type WgpuBufferViewMut<'a> = wgpu::BufferViewMut<'a>;

impl<T> GpuBufferNew<T> for WgpuBuffer
where
    T: BitAllUsed,
{
    fn new(value: &[T], usage: GpuBufferUsageFlags) -> Self
    {
        Gpu.device().create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bit::transmute_slice(value),
            usage: usage.into(),
        })
    }

    fn with_capacity(capacity: usize, usage: GpuBufferUsageFlags) -> Self
    {
        let size = capacity
            .checked_mul(std::mem::size_of::<T>())
            .and_then(|b| WgpuBufferAddress::try_from(b).ok())
            .expect("GpuBuffer capacity overflow");

        Gpu.device().create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: usage.into(),
            size,
            mapped_at_creation: false,
        })
    }
}

// AsRef<WgpuBuffer> : impossible to impl it for WgpuBuffer
pub trait WgpuSliceable<T>
where
    T: GpuBufferElement,
{
    fn wgpu_usage(&self) -> WgpuBufferUsage { self.wgpu_buffer_ref().usage() }
    
    fn wgpu_slice<S: RangeBounds<usize>>(&self, bounds: S) -> WgpuBufferSlice<'_> { WgpuSliceable::<T>::wgpu_slice(self.wgpu_buffer_ref(), bounds) }

    fn wgpu_as_slice(&self) -> WgpuBufferSlice<'_> { self.wgpu_slice(..) }
    fn wgpu_view(&self) -> WgpuBufferView<'_> { self.wgpu_slice(..).get_mapped_range() }
    fn wgpu_view_mut(&self) -> WgpuBufferViewMut<'_> { self.wgpu_slice(..).get_mapped_range_mut() }

    fn wgpu_buffer(&self) -> WgpuBuffer { self.wgpu_buffer_ref().clone() }
    fn wgpu_buffer_ref(&self) -> &WgpuBuffer;

    /// Performs a deep clone of the GPU buffer by submitting a copy command to the GPU.
    ///
    /// This method only submits the copy command and returns immediately without waiting.
    /// The returned buffer may not contain the copied data until the GPU completes the operation.
    /// Use `wgpu_deep_clone()` for a blocking version that waits for completion.
    ///
    /// # Returns
    /// A new `WgpuBuffer` with the same size and usage, but the contents are not yet guaranteed to be copied.
    fn wgpu_deep_clone_order(&self) -> WgpuBuffer { 
        WgpuSliceable::<T>::wgpu_deep_clone_order(&self.wgpu_slice(..))
    }

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
    fn wgpu_deep_clone_and_wait(&self) -> WgpuBuffer
    {
        let buff = self.wgpu_deep_clone_order();
        Gpu.wait();
        buff
    }
}

impl<T> WgpuSliceable<T> for WgpuBuffer
where
    T: GpuBufferElement
{
    fn wgpu_usage(&self) -> WgpuBufferUsage { self.usage() }

    fn wgpu_slice<S: RangeBounds<usize>>(&self, bounds: S) -> WgpuBufferSlice<'_>
    {
        let element_size = std::mem::size_of::<T>() as WgpuBufferAddress;

        let start_byte = match bounds.start_bound()
        {
            Bound::Included(&i) => (i as WgpuBufferAddress) * element_size,
            Bound::Excluded(&i) => ((i + 1) as WgpuBufferAddress) * element_size,
            Bound::Unbounded => 0,
        };

        let end_byte = match bounds.end_bound()
        {
            Bound::Included(&i) => ((i + 1) as WgpuBufferAddress) * element_size,
            Bound::Excluded(&i) => (i as WgpuBufferAddress) * element_size,
            Bound::Unbounded => self.size(),
        };

        self.slice(start_byte..end_byte)
    }

    fn wgpu_as_slice(&self) -> WgpuBufferSlice<'_> { self.slice(..) }

    fn wgpu_view(&self) -> WgpuBufferView<'_> { self.slice(..).get_mapped_range() }

    fn wgpu_view_mut(&self) -> WgpuBufferViewMut<'_> { self.slice(..).get_mapped_range_mut() }

    fn wgpu_deep_clone_order(&self) -> WgpuBuffer
    {
        WgpuSliceable::<T>::wgpu_deep_clone_order(&<wgpu::Buffer as WgpuSliceable<T>>::wgpu_slice(self, ..))
    }

    fn wgpu_buffer_ref(&self) -> &WgpuBuffer { self }
}


impl<'a,T> WgpuSliceable<T> for WgpuBufferSlice<'a>
where
    T: GpuBufferElement
{
    fn wgpu_usage(&self) -> WgpuBufferUsage { self.buffer().usage() }

    fn wgpu_slice<S: RangeBounds<usize>>(&self, bounds: S) -> WgpuBufferSlice<'_>
    {
        let element_size = std::mem::size_of::<T>() as WgpuBufferAddress;
        
        let start_byte = match bounds.start_bound()
        {
            Bound::Included(&i) => (i as WgpuBufferAddress) * element_size,
            Bound::Excluded(&i) => ((i + 1) as WgpuBufferAddress) * element_size,
            Bound::Unbounded => 0,
        } + self.offset() as WgpuBufferAddress;

        let end_byte = match bounds.end_bound()
        {
            Bound::Included(&i) => ((i + 1) as WgpuBufferAddress) * element_size,
            Bound::Excluded(&i) => (i as WgpuBufferAddress) * element_size,
            Bound::Unbounded => self.size().get(),
        } + self.offset() as WgpuBufferAddress;

        self.slice(start_byte..end_byte)
    }

    fn wgpu_deep_clone_order(&self) -> WgpuBuffer
    {
        let size = self.size().get();
        let usage = WgpuSliceable::<T>::wgpu_usage(self);

        let new_buffer = Gpu.device().create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage,
            mapped_at_creation: false,
        });

        let mut encoder = Gpu.device().create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        encoder.copy_buffer_to_buffer(self.buffer(), self.offset(), &new_buffer, 0, size);
        Gpu.queue().submit(Some(encoder.finish()));

        new_buffer
    }

    fn wgpu_buffer_ref(&self) -> &WgpuBuffer { self.buffer() }
}
