use core::{pin::Pin, ptr::NonNull};
use super::*;

/// A box that will, when dropped, drop in place it's content, but not deallocate
pub struct DropOnlyBox<T: ?Sized> {
    ptr: PtrUnaliased<T>,
}

impl<T : ?Sized> DropOnlyBox<T>
{
    pub unsafe fn from_non_null(ptr: NonNull<T>) -> Self
    {
        unsafe { Self::new(ptr) }
    }

    pub unsafe fn from_raw(ptr: *mut T) -> Self
    {
        debug_assert!(!ptr.is_null());
        unsafe { Self::new(PtrUnaliased::new_unchecked(ptr)) }
    }

    pub unsafe fn new(ptr: PtrUnaliased<T>) -> Self {
        Self {
            ptr,
        }
    }

    pub fn as_ptr(&self) -> *mut T { self.ptr.as_ptr() }
    pub fn as_mut(&mut self) -> *mut T { unsafe { self.ptr.as_mut() } }

    pub fn into_pin(s: Self) -> Pin<Self> where { unsafe { Pin::new_unchecked(s) } }

    pub fn leak<'a>(mut s: Self) -> &'a mut T { let val = unsafe { s.ptr.as_mut() }; std::mem::forget(s); val }
}
impl<T> DropOnlyBox<T>
{
    pub fn into_inner(s: Self) -> T { let val = unsafe { s.ptr.read() }; std::mem::forget(s); val }
}

impl<T : ?Sized> Deref for DropOnlyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T : ?Sized> DerefMut for DropOnlyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T : ?Sized> Drop for DropOnlyBox<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.ptr.as_ptr());
        }
    }
}