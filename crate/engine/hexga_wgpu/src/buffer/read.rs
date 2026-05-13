use super::*;


pub struct GpuSliceRead<'a, T>
    where T: Copy
{
    view : WgpuBufferView<'a>,
    phantom: PhantomData<T>,
}
impl<'a,T> GpuSliceRead<'a, T> where T: Copy 
{
    pub unsafe fn from_wgpu(view: WgpuBufferView<'a>) -> Self {
        Self { view, phantom: PhantomData }
    }
}
impl<T> Deref for GpuSliceRead<'_, T>
where T: Copy
{
    type Target = [T];
    fn deref(&self) -> &[T] { unsafe { bit::try_transmute_slice_unchecked(&self.view) }.unwrap() }
}
