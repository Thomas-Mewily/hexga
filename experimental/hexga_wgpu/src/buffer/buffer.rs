use super::*;

/// A strongly typed buffer that live on the GPU.
/// This have regular value semantics with Copy-on-Write (CoW) optimization:
///
/// - Cloning performs a deep copy semantically, but internally shares the underlying GPU buffer
///   until a modification is required (copy-on-write).
///
/// # Performance, Cloning and mutation
///
/// - `Clone` is O(1) and cheap: it only increments a reference count internally.
///
/// - Mutation is cheap when the buffer is uniquely owned; a deep clone of the GPU buffer
///   occurs only when the buffer is shared.
#[derive(Clone)]
pub struct GBuffer<T>
where
    T: GBufferElement,
{
    pub(crate) buffer: Arc<WgpuBuffer>,
    typed: PhantomData<T>,
}

impl<T> Handle for GBuffer<T> where T: GBufferElement {}

impl<T> Debug for GBuffer<T>
where
    T: GBufferElement,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("GpuBuffer<{}>", std::any::type_name::<T>()))
            .field("buffer", &self.buffer)
            .finish()
    }
}

impl_gpu_index!([T: GBufferElement] GBuffer<T>);

impl<T> Length for GBuffer<T>
where
    T: GBufferElement,
{
    fn len(&self) -> usize {
        let len = self.buffer.size().checked_div(size_of::<T>() as u64).unwrap_or(0);
        usize::try_from(len).expect("GPU buffer length exceeds usize")
    }
}

impl<T: GBufferElement> Capacity<T> for GBuffer<T> {
    fn capacity(&self) -> usize {
        self.len()
    }
}

impl<T> GpuBufferNew<T> for GBuffer<T>
where
    T: BitAllUsed,
{
    fn new(value: &[T], usage: GBufferUsageFlags) -> Self {
        let padded_len = size_of_val(value)
            .next_multiple_of(wgpu::COPY_BUFFER_ALIGNMENT as usize)
            .checked_div(size_of::<T>())
            .unwrap_or(0);

        // Whole padding elements must also contain valid values of T.
        let mut padded;
        let value = if padded_len > value.len() {
            padded = value.to_vec();
            padded.resize(padded_len, value[0]);
            &padded
        } else {
            value
        };

        Self {
            buffer: Arc::new(<WgpuBuffer as GpuBufferNew<T>>::new(value, usage)),
            typed: PhantomData,
        }
    }

    fn with_capacity(capacity: usize, usage: GBufferUsageFlags) -> Self {
        WithCapacity::with_capacity_and_param(capacity, usage)
    }
}

impl<T> WithCapacity for GBuffer<T>
where
    T: BitAllUsed,
{
    type Param = GBufferUsageFlags;
    fn with_capacity_and_param(capacity: usize, usage: Self::Param) -> Self {
        Self {
            buffer: Arc::new(<WgpuBuffer as GpuBufferNew<T>>::with_capacity(capacity, usage)),
            typed: PhantomData,
        }
    }
}

impl<T> WgpuSliceable<T> for GBuffer<T>
where
    T: GBufferElement,
{
    fn lib_wgpu_buffer_ref(&self) -> &WgpuBuffer {
        &self.buffer
    }
}

impl<T> GpuSliceable<T> for GBuffer<T>
where
    T: GBufferElement,
{
    fn usage(&self) -> GBufferUsageFlags {
        self.lib_wgpu_usage().into()
    }
    fn slice<S: RangeBounds<usize>>(&self, bounds: S) -> GSlice<'_, T> {
        GSlice::new(self, bounds)
    }
}

impl<T: GBufferElement> GBuffer<T> {
    pub(crate) fn read_elements(&self, range: StdRange<usize>) -> GSliceRead<'_, T> {
        if range.is_empty() {
            return GSliceRead::empty();
        }

        let element_size = size_of::<T>() as WgpuBufferAddress;
        let begin = range.start as WgpuBufferAddress * element_size;
        let end = range.end as WgpuBufferAddress * element_size;

        // wgpu requires aligned mapped views. Expose only the requested bytes.
        let mapped_begin = begin - begin % wgpu::MAP_ALIGNMENT;
        let mapped_end = end.next_multiple_of(wgpu::COPY_BUFFER_ALIGNMENT);
        let bytes = (begin - mapped_begin) as usize..(end - mapped_begin) as usize;
        let view = self.buffer.get_mapped_range(mapped_begin..mapped_end);
        unsafe { GSliceRead::from_wgpu_range(view, bytes) }
    }
}

impl<T> GpuSliceableMut<T> for GBuffer<T>
where
    T: GBufferElement,
{
    fn slice_mut<S: RangeBounds<usize>>(&mut self, bounds: S) -> GSliceMut<'_, T> {
        GSliceMut::new(self, bounds)
    }
}

pub trait ToGpuBuffer<T>
where
    T: BitAllUsed,
{
    fn to_gpu_buffer(self, desc: GBufferUsageFlags) -> GBuffer<T>;
}
impl<T> ToGpuBuffer<T> for &[T]
where
    T: BitAllUsed,
{
    fn to_gpu_buffer(self, desc: GBufferUsageFlags) -> GBuffer<T> {
        GBuffer::new(self, desc)
    }
}

#[cfg(all(test, feature = "gpu_backend_noop"))]
pub(super) mod tests {
    use super::*;

    pub(crate) fn init_gpu() {
        use std::future::Future;
        use std::pin::pin;
        use std::sync::Once;
        use std::task::{Context, Poll, Waker};

        static INIT: Once = Once::new();
        INIT.call_once(|| {
            let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
                backends: wgpu::Backends::NOOP,
                backend_options: wgpu::BackendOptions {
                    noop: wgpu::NoopBackendOptions { enable: true },
                    ..Default::default()
                },
                ..Default::default()
            });

            // The noop backend resolves both requests immediately.
            let context = &mut Context::from_waker(Waker::noop());
            let Poll::Ready(Ok(adapter)) = pin!(instance.request_adapter(&wgpu::RequestAdapterOptions::default())).poll(context) else {
                panic!("Could not initialize the noop adapter");
            };
            let Poll::Ready(Ok((device, queue))) = pin!(adapter.request_device(&wgpu::DeviceDescriptor::default())).poll(context) else {
                panic!("Could not initialize the noop device");
            };

            assert!(crate::experimental::GPU.init(GpuContext::from_wgpu(instance, adapter, device, queue)).is_ok());
        });
    }

    fn mapped_buffer() -> GBuffer<u16> {
        init_gpu();
        let raw = Gpu.device().create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 12,
            usage: wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: true,
        });
        raw.slice(..).get_mapped_range_mut()[..10].copy_from_slice(bit::transmute_slice(&[10u16, 20, 30, 40, 50]));

        GBuffer {
            buffer: Arc::new(raw),
            typed: PhantomData,
        }
    }

    #[test]
    fn clone_shares_allocation_until_mapped_write() {
        let original = mapped_buffer();
        let mut clone = original.clone();
        assert!(Arc::ptr_eq(&original.buffer, &clone.buffer));

        clone.write(..).update(&[1, 2, 3, 4, 5, 6]);
        assert!(!Arc::ptr_eq(&original.buffer, &clone.buffer));
        assert_eq!(&*original.read(..5), &[10, 20, 30, 40, 50]);
        assert_eq!(&*clone.as_read(), &[1, 2, 3, 4, 5, 6]);

        let allocation = Arc::as_ptr(&clone.buffer);
        clone.write(..).update(&[6, 5, 4, 3, 2, 1]);
        assert_eq!(Arc::as_ptr(&clone.buffer), allocation);
        assert_eq!(&*clone.as_read(), &[6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn standard_range_indices_match_slice_get() {
        fn check<Idx>(buffer: &GSlice<'_, u16>, data: &[u16], index: Idx)
        where
            Idx: std::slice::SliceIndex<[u16], Output = [u16]> + Clone,
            for<'a> GSlice<'a, u16>: Get<Idx, Output = [u16]>,
        {
            let expected = data.get(index.clone());
            assert_eq!(Get::get(buffer, index.clone()).as_deref(), expected);

            let mapped = buffer.as_read();
            assert_eq!(Get::get(&mapped, index), expected);
        }

        let buffer = mapped_buffer();
        let slice = buffer.slice(1..5);
        let data = [20, 30, 40, 50];

        check(&slice, &data, 1..3);
        check(&slice, &data, 2..);
        check(&slice, &data, ..2);
        check(&slice, &data, 1..=3);
        check(&slice, &data, ..=1);
        check(&slice, &data, ..);
        check(&slice, &data, 4..4);
        check(&slice, &data, 3..2);
        check(&slice, &data, ..5);
        check(&slice, &data, ..=usize::MAX);

        let range = hexga::range::Range { start: 1, end: 3 };
        assert_eq!(&*Get::get(&slice, range).unwrap(), &data[1..3]);
    }

    #[test]
    fn try_read_accepts_custom_range_bounds() {
        struct Tail(usize);

        impl RangeBounds<usize> for Tail {
            fn start_bound(&self) -> Bound<&usize> {
                Bound::Excluded(&self.0)
            }
            fn end_bound(&self) -> Bound<&usize> {
                Bound::Unbounded
            }
        }

        let buffer = mapped_buffer();
        let slice = buffer.slice(..5);
        assert_eq!(&*slice.try_read(Tail(2)).unwrap(), &[40, 50]);
        assert!(slice.try_read(Tail(4)).unwrap().is_empty());
        assert!(slice.try_read(Tail(usize::MAX)).is_none());
    }

    #[test]
    fn truncation_preserves_capacity_and_limits_collection_reads() {
        let buffer = mapped_buffer();
        assert_eq!(Capacity::<u16>::capacity(&buffer), 6);
        assert_eq!(*First::first(&buffer).unwrap(), 10);

        let mut vec = GVec { buffer, len: 5 };
        vec.truncate(usize::MAX);
        assert_eq!(vec.len(), 5);

        vec.truncate(2);
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.capacity(), 6);
        assert_eq!(*Last::last(&vec).unwrap(), 20);
        assert_eq!(&*TryGet::try_get(&vec, ..).ok().unwrap(), &[10, 20]);

        let error = TryGet::try_get(&vec, 2usize).err().unwrap();
        assert_eq!(error, IndexOutOfBounds::new(2, 0..2));
        let error = TryGet::try_get(&vec, 1..3).err().unwrap();
        assert_eq!(error, IndexOutOfBounds::new(1..3, 0..2));

        vec.clear();
        assert!(Last::last(&vec).is_none());
        assert!(First::first(&vec).is_none());
        assert_eq!(vec.capacity(), 6);
    }

    #[test]
    fn indexed_reads_use_allocation_and_subslice_lengths() {
        let mut buffer = mapped_buffer();
        assert_eq!(buffer.len(), 6);
        assert!(Get::get(&buffer, 6usize).is_none());
        assert!(Get::get(&buffer, ..=usize::MAX).is_none());
        assert!(Get::get(&buffer, 6..6).unwrap().is_empty());

        {
            let mut slice = buffer.slice_mut(1..4);
            let nested = slice.slice_mut(1..);
            assert_eq!(*Get::get(&nested, 0usize).unwrap(), 30);
            let read = Get::get(&nested, ..=1).unwrap();
            assert_eq!(&*read, &[30, 40]);
            assert_eq!(Get::get(&read, 1..), Some(&[40][..]));
            assert!(Get::get(&nested, ..3).is_none());
        }

        {
            let slice = buffer.slice(1..);
            let bounds = (Bound::Excluded(0), Bound::Included(2));
            let read = Get::get(&slice, bounds).unwrap();
            assert_eq!(&*read, &[30, 40]);
        }

        let vec = GVec { buffer, len: 3 };
        assert!(Get::get(&vec, ..4).is_none());
        assert_eq!(&*Get::get(&vec, ..).unwrap(), &[10, 20, 30]);
    }
}
