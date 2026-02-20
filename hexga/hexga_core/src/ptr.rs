use crate::Box;

pub mod prelude
{
    pub use super::{NonNull, NonNullUnaliased};
}

re_export_items_from_std_or_core!(ptr);

pub type NonNull<T = u8> = ::core::ptr::NonNull<T>;

pub trait Pointer: FromNonNull + IntoNonNull {}
impl<T: ?Sized> Pointer for T where T: FromNonNull + IntoNonNull {}

pub unsafe trait FromNonNull
{
    unsafe fn from_non_null(ptr: NonNull) -> Self;
}
pub unsafe trait IntoNonNull
{
    unsafe fn into_non_null(self) -> NonNull;
    unsafe fn as_non_null(&mut self) -> NonNull;
}

unsafe impl<T> FromNonNull for &T
{
    #[inline(always)]
    unsafe fn from_non_null(ptr: NonNull) -> Self { unsafe { ptr.cast().as_ref() } }
}
unsafe impl<T> FromNonNull for &mut T
{
    #[inline(always)]
    unsafe fn from_non_null(ptr: NonNull) -> Self { unsafe { ptr.cast().as_mut() } }
}
unsafe impl<T> IntoNonNull for &T
{
    unsafe fn into_non_null(self) -> NonNull
    {
        unsafe { NonNull::new_unchecked(self as *const T as *const u8 as *mut u8) }
    }

    unsafe fn as_non_null(&mut self) -> NonNull
    {
        unsafe { NonNull::new_unchecked(*self as *const T as *const u8 as *mut u8) }
    }
}
unsafe impl<T> IntoNonNull for &mut T
{
    unsafe fn into_non_null(self) -> NonNull
    {
        unsafe { NonNull::new_unchecked(self as *mut T as *mut u8) }
    }

    unsafe fn as_non_null(&mut self) -> NonNull
    {
        unsafe { NonNull::new_unchecked(*self as *mut T as *mut u8) }
    }
}

unsafe impl<T> FromNonNull for Box<T>
{
    unsafe fn from_non_null(ptr: NonNull) -> Self
    {
        unsafe { Box::from_raw(ptr.as_ptr() as *mut T) }
    }
}
unsafe impl<T> IntoNonNull for Box<T>
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

// Copied from the std: https://doc.rust-lang.org/stable/src/core/ptr/unique.rs.html#46-49

/// A wrapper around a raw non-null `*mut T` that indicates that the possessor
/// of this wrapper owns the referent. Useful for building abstractions like
/// `Box<T>`, `Vec<T>`, `String`, and `HashMap<K, V>`.
///
/// Unlike `*mut T`, `Unaliased<T>` behaves "as if" it were an instance of `T`.
/// It implements `Send`/`Sync` if `T` is `Send`/`Sync`. It also implies
/// the kind of strong aliasing guarantees an instance of `T` can expect:
/// the referent of the pointer should not be modified without a unaliased path to
/// its owning Unaliased.
///
/// If you're uncertain of whether it's correct to use `Unaliased` for your purposes,
/// consider using `NonNull`, which has weaker semantics.
///
/// Unlike `*mut T`, the pointer must always be non-null, even if the pointer
/// is never dereferenced. This is so that enums may use this forbidden value
/// as a discriminant -- `Option<Unaliased<T>>` has the same size as `Unaliased<T>`.
/// However the pointer may still dangle if it isn't dereferenced.
///
/// Unlike `*mut T`, `Unaliased<T>` is covariant over `T`. This should always be correct
/// for any type which upholds Unaliased's aliasing requirements.
///
/// Drop should still be implemented manually
pub type NonNullUnaliased<T = u8> = NonNull<T>;

/*
pub struct PtrBox<T: ?Sized> {
    ptr: PtrUnique<T>,
}

impl<T> PtrBox<T : ?Sized> {
    pub fn new(value: T) -> Self
    {
        Self { ptr: unsafe { Memory.alloc_value(value) } }
    }

    pub fn as_ptr(b: &Self) -> NonNull<T>
    {
        b.ptr
    }

    pub fn into_raw(b: Self) -> NonNull<T>
    {
        let ptr = b.ptr;
        std::mem::forget(b);
        ptr
    }

    pub unsafe fn from_raw(ptr: NonNull<T>) -> Self {
        Self { ptr }
    }
}

impl<T> Clone for PtrBox<T> where T: Clone {
    fn clone(&self) -> Self {
        Self { ptr: unsafe { Memory.alloc_value(self.deref().clone()) } }
    }
}

impl<T> Deref for PtrBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> DerefMut for PtrBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T> Drop for PtrBox<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.ptr.as_ptr());
            Memory.dealloc(AllocLayout::for_type::<T>(), self.ptr.as_ptr() as *mut u8);
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for PtrBox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}
*/
