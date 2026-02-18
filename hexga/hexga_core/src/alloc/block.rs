use super::*;

pub struct AllocBlock
{
    ptr   : PtrUnaliased,
    layout: AllocLayout,
}

impl Debug for AllocBlock
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", *self)
    }
}

impl Clone for AllocBlock
{
    fn clone(&self) -> Self
    {
        let mut cloned = Self::from_alloc_layout(self.layout);
        cloned.deref_mut().copy_from_slice(self.deref());
        cloned
    }
}

impl From<AllocLayout> for AllocBlock
{
    fn from(layout: AllocLayout) -> Self {
        Self { ptr: unsafe { Memory.alloc_layout(layout) }, layout }
    }
}
impl AllocBlock
{
    pub fn layout(&self) -> AllocLayout { self.layout }
}
impl Drop for AllocBlock
{
    fn drop(&mut self) {
        unsafe { Memory.dealloc_layout(self.layout, self.ptr.as_ptr()); };
    }
}
impl Deref for AllocBlock
{
    type Target=[u8];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.layout.size) }
    }
}
impl DerefMut for AllocBlock
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.layout.size) }
    }
}