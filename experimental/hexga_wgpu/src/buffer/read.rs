use super::*;

pub struct GSliceRead<'a, T>
where
    T: GBufferElement,
{
    view: WgpuBufferView<'a>,
    phantom: PhantomData<T>,
}

pub struct GElementRead<'a, T>
where
    T: GBufferElement,
{
    view: GSliceRead<'a, T>,
    index: usize,
}
impl<'a, T> GElementRead<'a, T>
where
    T: GBufferElement,
{
    pub(crate) fn new(view: GSliceRead<'a, T>, index: usize) -> Self { Self { view, index } }
}
impl<T> Deref for GElementRead<'_, T>
where
    T: GBufferElement,
{
    type Target = T;
    fn deref(&self) -> &T { &self.view[self.index] }
}
impl<'a, T> GSliceRead<'a, T>
where
    T: GBufferElement,
{
    pub unsafe fn from_wgpu(view: WgpuBufferView<'a>) -> Self { Self { view, phantom: PhantomData } }
}
impl<T> Deref for GSliceRead<'_, T>
where
    T: GBufferElement,
{
    type Target = [T];
    fn deref(&self) -> &[T] { unsafe { bit::try_transmute_slice_unchecked(&self.view) }.unwrap() }
}

impl<'a, T> Collection<usize> for GSliceRead<'a, T>
where
    T: GBufferElement,
{
    type Output = T;
    type Guard<'b> = &'b T where Self: 'b, Self::Output: 'b;
}
impl<'a, T> Get<usize> for GSliceRead<'a, T>
where
    T: GBufferElement,
{
    fn get<'b>(&'b self, index: usize) -> Option<Self::Guard<'b>> where Self::Output: 'b { self.deref().get(index) }
}
impl<'a, T> Length for GSliceRead<'a, T>
where
    T: GBufferElement,
{
    fn len(&self) -> usize { self.deref().len() }
}
