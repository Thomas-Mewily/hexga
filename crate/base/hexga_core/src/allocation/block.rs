use super::*;

pub struct AllocBlock<T = u8>
where
    T: BitAnyPattern,
{
    ptr: NonNullUnaliased<T>,
    layout: AllocLayout,
}

pub const BLOCK_DEFAULT_SIZE: usize = 16 * 1024; // 16K bytes

impl<T> AllocBlock<T>
where
    T: BitAnyPattern,
{
    //pub const DEFAULT_SIZE : usize = 16 * 1024; // 16K bytes
}
impl<T> Default for AllocBlock<T>
where
    T: BitAnyPattern,
    T: Default,
{
    fn default() -> Self { Self::from_size_and_align(BLOCK_DEFAULT_SIZE, MAX_ALIGN) }
}

impl<T> Debug for AllocBlock<T>
where
    T: BitAnyPattern,
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "{:?}", *self) }
}

impl<T> Clone for AllocBlock<T>
where
    T: BitAnyPattern,
{
    fn clone(&self) -> Self
    {
        let mut cloned = Self::from_alloc_layout(self.layout);
        cloned.deref_mut().copy_from_slice(self.deref());
        cloned
    }
}

impl<T> From<AllocLayout> for AllocBlock<T>
where
    T: BitAnyPattern,
{
    fn from(layout: AllocLayout) -> Self
    {
        debug_assert_eq!(layout.align() % mem::align_of::<T>(), 0);
        Self {
            ptr: Memory.alloc_layout_or_panic(layout).cast(),
            layout,
        }
    }
}
impl<T> Collection for AllocBlock<T> where T: BitAnyPattern {}
impl<T> Capacity for AllocBlock<T>
where
    T: BitAnyPattern,
{
    #[inline(always)]
    fn capacity(&self) -> usize { self.capacity() }
}
impl<T> AllocBlock<T>
where
    T: BitAnyPattern,
{
    pub const fn capacity(&self) -> usize { self.layout.size / size_of::<T>() }
    pub const fn capacity_u8(&self) -> usize { self.layout.size }
}
impl<T> AllocBlock<T>
where
    T: BitAnyPattern,
{
    pub fn layout(&self) -> AllocLayout { self.layout }
}
impl<T> Drop for AllocBlock<T>
where
    T: BitAnyPattern,
{
    fn drop(&mut self)
    {
        for v in self.iter_mut()
        {
            unsafe { ptr::drop_in_place(v as *mut T) };
        }
        Memory.dealloc_layout(self.ptr.cast(), self.layout);
    }
}
impl<T> Deref for AllocBlock<T>
where
    T: BitAnyPattern,
{
    type Target = [T];
    fn deref(&self) -> &Self::Target
    {
        unsafe { slice::from_raw_parts::<T>(self.ptr.as_ptr(), self.capacity()) }
    }
}
impl<T> DerefMut for AllocBlock<T>
where
    T: BitAnyPattern,
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        unsafe { slice::from_raw_parts_mut::<T>(self.ptr.as_ptr(), self.capacity()) }
    }
}
