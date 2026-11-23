use super::*;

pub struct GSliceRead<'a, T>
where
    T: GBufferElement,
{
    view: WgpuBufferView<'a>,
    phantom: PhantomData<T>,
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

impl<'a, T> Collection for GSliceRead<'a, T> where T: GBufferElement {}
impl<'a, T> Length for GSliceRead<'a, T>
where
    T: GBufferElement,
{
    fn len(&self) -> usize { self.deref().len() }
}
