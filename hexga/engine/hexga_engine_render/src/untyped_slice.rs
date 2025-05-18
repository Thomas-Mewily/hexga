use super::*;

pub struct UntypedSlice<'a>
{
    pub(crate) data    : *const u8,
    pub(crate) layout  : BufferLayout,
    pub(crate) phantom : PhantomData<&'a ()>,
}
impl<'a> Deref for UntypedSlice<'a>
{
    type Target=BufferLayout;
    fn deref(&self) -> &Self::Target { &self.layout }
}
impl<'a> DerefMut for UntypedSlice<'a>
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.layout }
}