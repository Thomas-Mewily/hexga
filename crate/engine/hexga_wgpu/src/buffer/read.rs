use super::*;


pub struct GpuSliceRead<'a, T>
    where T: GpuBufferElement
{
    view : WgpuBufferView<'a>,
    phantom: PhantomData<T>,
}
impl<'a,T> GpuSliceRead<'a, T> where T: GpuBufferElement 
{
    pub unsafe fn from_wgpu(view: WgpuBufferView<'a>) -> Self {
        Self { view, phantom: PhantomData }
    }
}
impl<T> Deref for GpuSliceRead<'_, T>
where T: GpuBufferElement
{
    type Target = [T];
    fn deref(&self) -> &[T] { unsafe { bit::try_transmute_slice_unchecked(&self.view) }.unwrap() }
}

impl<'a, T> Collection for GpuSliceRead<'a, T> where T: GpuBufferElement {}
impl<'a, T> Length for GpuSliceRead<'a, T> where T: GpuBufferElement
{
    fn len(&self) -> usize { 
        self.deref().len()
    }
}