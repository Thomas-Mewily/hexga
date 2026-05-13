use super::*;


pub struct GpuSliceMutWrite<'a, T>
    where T: Copy
{
    pub(crate) view : WgpuBufferViewMut<'a>,
    phantom: PhantomData<T>,
}
impl<'a,T> GpuSliceMutWrite<'a, T> where T: Copy 
{
    pub unsafe fn from_wgpu(view: WgpuBufferViewMut<'a>) -> Self {
        Self { view, phantom: PhantomData }
    }
}