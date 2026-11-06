use super::*;

pub struct GSliceMutWrite<'a, T>
where
    T: GpuBufferElement,
{
    pub(crate) view: WgpuBufferViewMut<'a>,
    phantom: PhantomData<T>,
}
impl<'a, T> GSliceMutWrite<'a, T>
where
    T: GpuBufferElement,
{
    pub unsafe fn from_wgpu(view: WgpuBufferViewMut<'a>) -> Self { Self { view, phantom: PhantomData } }

    pub fn try_update(&mut self, src: &[T]) -> BitResult
    {
        let slice = unsafe { bit::try_transmute_slice_unchecked(src) }?;
        self.view.copy_from_slice(slice);
        Ok(())
    }

    pub fn update(&mut self, src: &[T]) { self.try_update(src).expect("bit cast failed") }
}


impl<'a, T> Collection for GSliceMutWrite<'a, T> where T: GpuBufferElement {}
impl<'a, T> Length for GSliceMutWrite<'a, T>
where
    T: GpuBufferElement,
{
    fn len(&self) -> usize { self.view.len() }
}
