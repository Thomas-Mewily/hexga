use super::*;


pub struct GpuSliceMutWrite<'a, T>
    where T: GpuBufferElement
{
    pub(crate) view : WgpuBufferViewMut<'a>,
    phantom: PhantomData<T>,
}
impl<'a,T> GpuSliceMutWrite<'a, T> where T: GpuBufferElement 
{
    pub unsafe fn from_wgpu(view: WgpuBufferViewMut<'a>) -> Self {
        Self { view, phantom: PhantomData }
    }

    pub fn update(&mut self, src : &[T])
    {
        self.view.copy_from_slice(unsafe { bit::try_transmute_slice_unchecked(src) }.unwrap());
    }
}

/*
pub trait GpuBufferWrite<T>
{
    fn update(&mut self, src : &[T]);
}
*/

impl<'a, T> Collection for GpuSliceMutWrite<'a, T> where T: GpuBufferElement {}
impl<'a, T> Length for GpuSliceMutWrite<'a, T> where T: GpuBufferElement
{
    fn len(&self) -> usize { 
        self.view.len()
    }
}