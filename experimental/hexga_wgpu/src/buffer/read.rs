use super::*;
use std::slice::SliceIndex;

pub struct GSliceRead<'a, T>
where
    T: GBufferElement,
{
    mapped: Option<(WgpuBufferView<'a>, StdRange<usize>)>,
    typed: PhantomData<T>,
}

pub struct GElementRead<'a, T>
where
    T: GBufferElement,
{
    view: GSliceRead<'a, T>,
}
impl<'a, T> GElementRead<'a, T>
where
    T: GBufferElement,
{
    pub(crate) fn new(view: GSliceRead<'a, T>) -> Self {
        Self { view }
    }
}
impl<T> Deref for GElementRead<'_, T>
where
    T: GBufferElement,
{
    type Target = T;
    fn deref(&self) -> &T {
        &self.view[0]
    }
}
impl<'a, T> GSliceRead<'a, T>
where
    T: GBufferElement,
{
    pub unsafe fn from_wgpu(view: WgpuBufferView<'a>) -> Self {
        let bytes = 0..view.len();
        unsafe { Self::from_wgpu_range(view, bytes) }
    }

    pub(crate) unsafe fn from_wgpu_range(view: WgpuBufferView<'a>, bytes: StdRange<usize>) -> Self {
        Self {
            mapped: Some((view, bytes)),
            typed: PhantomData,
        }
    }

    pub(crate) fn empty() -> Self {
        Self {
            mapped: None,
            typed: PhantomData,
        }
    }
}
impl<T> Deref for GSliceRead<'_, T>
where
    T: GBufferElement,
{
    type Target = [T];
    fn deref(&self) -> &[T] {
        match &self.mapped {
            Some((view, bytes)) => unsafe { bit::try_transmute_slice_unchecked(&view[bytes.clone()]) }.unwrap(),
            None => &[],
        }
    }
}

impl<'s, T: GBufferElement> Collection<T> for GSliceRead<'s, T> {
    type Ref<'a>
        = &'a T
    where
        Self: 'a,
        T: 'a;
}

impl<'s, T: GBufferElement> Collection<[T]> for GSliceRead<'s, T> {
    type Ref<'a>
        = &'a [T]
    where
        Self: 'a,
        T: 'a;
}

impl<T, Idx> CollectionIndex<Idx> for GSliceRead<'_, T>
where
    T: GBufferElement,
    Idx: SliceIndex<[T]>,
{
    type Output = Idx::Output;
    type Ref<'a>
        = &'a Idx::Output
    where
        Self: 'a,
        Self::Output: 'a;
}
impl<T, Idx> Get<Idx> for GSliceRead<'_, T>
where
    T: GBufferElement,
    Idx: SliceIndex<[T]>,
{
    fn get<'a>(&'a self, index: Idx) -> Option<Self::Ref<'a>>
    where
        Self::Output: 'a,
    {
        self.deref().get(index)
    }
}
impl<'a, T> Length for GSliceRead<'a, T>
where
    T: GBufferElement,
{
    fn len(&self) -> usize {
        self.deref().len()
    }
}
