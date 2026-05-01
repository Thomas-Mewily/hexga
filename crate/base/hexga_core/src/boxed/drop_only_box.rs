use super::*;
use core::pin::Pin;

/// A box that will, when dropped, drop in place it's content, but not deallocate
pub struct DropOnlyBox<T: ?Sized>
{
    ptr: NonNullUnaliased<T>,
}

impl<T: ?Sized> Debug for DropOnlyBox<T> where T: Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        unsafe { self.ptr.as_ref().fmt(f) }
    }
}
impl<T: ?Sized> Display for DropOnlyBox<T> where T: Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        unsafe { self.ptr.as_ref().fmt(f) }
    }
}

unsafe impl<T> FromNonNull for DropOnlyBox<T>
{
    unsafe fn from_non_null(ptr: NonNull) -> Self
    {
        unsafe { DropOnlyBox::from_raw(ptr.as_ptr() as *mut T) }
    }
}
unsafe impl<T> IntoNonNull for DropOnlyBox<T>
{
    unsafe fn into_non_null(self) -> NonNull
    {
        unsafe { NonNull::new_unchecked(Self::leak(self) as *mut T as *mut u8) }
    }

    unsafe fn as_non_null(&mut self) -> NonNull
    {
        unsafe { NonNull::new_unchecked(self.as_mut() as *mut T as *mut u8) }
    }
}

impl<T: ?Sized> DropOnlyBox<T>
{
    pub const unsafe fn from_non_null(ptr: NonNull<T>) -> Self { unsafe { Self::new(ptr) } }

    pub const unsafe fn from_raw(ptr: *mut T) -> Self
    {
        debug_assert!(!ptr.is_null());
        unsafe { Self::new(NonNullUnaliased::new_unchecked(ptr)) }
    }

    pub const unsafe fn new(ptr: NonNullUnaliased<T>) -> Self { Self { ptr } }

    pub const fn as_ptr(&self) -> *mut T { self.ptr.as_ptr() }
    pub const fn as_mut(&mut self) -> *mut T { unsafe { self.ptr.as_mut() } }

    pub const fn into_pin(s: Self) -> Pin<Self> where { unsafe { Pin::new_unchecked(s) } }

    pub const fn leak<'a>(mut s: Self) -> &'a mut T
    {
        let val = unsafe { s.ptr.as_mut() };
        mem::forget(s);
        val
    }
}
impl<T> DropOnlyBox<T>
{
    pub fn into_inner(s: Self) -> T
    {
        let val = unsafe { s.ptr.read() };
        mem::forget(s);
        val
    }
}

impl<T: ?Sized> Deref for DropOnlyBox<T>
{
    type Target = T;
    fn deref(&self) -> &T { unsafe { self.ptr.as_ref() } }
}

impl<T: ?Sized> DerefMut for DropOnlyBox<T>
{
    fn deref_mut(&mut self) -> &mut T { unsafe { self.ptr.as_mut() } }
}

impl<T: ?Sized> Drop for DropOnlyBox<T>
{
    fn drop(&mut self)
    {
        unsafe {
            ptr::drop_in_place(self.ptr.as_ptr());
        }
    }
}
