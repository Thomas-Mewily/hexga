use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UntypedSlice<'a>
{
    pub data    : *const u8,
    pub layout  : BufferLayout,
    pub phantom : PhantomData<&'a ()>,
}



impl<'a> Debug for UntypedSlice<'a>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UntypedSlice").finish()
    }
}

impl<'a> Deref for UntypedSlice<'a>
{
    type Target=[u8];
    fn deref(&self) -> &Self::Target
    { 
         unsafe { std::slice::from_raw_parts(self.data, self.layout.size()) }
    }
}