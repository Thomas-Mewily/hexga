use super::*;

pub struct AllocBlock
{
    ptr: NonNullUnaliased,
    layout: AllocLayout,
}

impl AllocBlock
{
    pub const DEFAULT_SIZE : usize = 16 * 1024; // 16K bytes
}
impl Default for AllocBlock
{
    fn default() -> Self {
        Self::from_size_and_align(Self::DEFAULT_SIZE, 1)
    }
}


impl Debug for AllocBlock
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", *self) }
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
    fn from(layout: AllocLayout) -> Self
    {
        Self {
            ptr: Memory.allocate_layout_or_panic(layout).cast(),
            layout,
        }
    }
}
impl AllocBlock
{
    pub fn layout(&self) -> AllocLayout { self.layout }
}
impl Drop for AllocBlock
{
    fn drop(&mut self) { Memory.deallocate_layout(self.ptr, self.layout); }
}
impl Deref for AllocBlock
{
    type Target = [u8];
    fn deref(&self) -> &Self::Target
    {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.layout.size) }
    }
}
impl DerefMut for AllocBlock
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.layout.size) }
    }
}
