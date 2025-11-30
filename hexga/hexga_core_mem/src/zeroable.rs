use super::*;

/// Trait for types that can be safely created with
/// [`zeroed`](core::mem::zeroed).
///
/// An all-zeroes value may or may not be the same value as the
/// [Default](core::default::Default) value of the type.
///
///  ## Safety
///
/// * Your type must be allowed to be an "all zeroes" bit pattern (eg: no
///   [`NonNull<T>`](core::ptr::NonNull)).
pub unsafe trait Zeroable : Sized + Zeroed {}

map_on_number_and_bool!(
    ($type_name : ident) =>
    {
        unsafe impl Zeroable for $type_name {}
    }
);
unsafe impl Zeroable for char {}
unsafe impl<T: Zeroable> Zeroable for Wrapping<T> {}
unsafe impl<T: Zeroable> Zeroable for core::cmp::Reverse<T> {}
unsafe impl<T: Zeroable> Zeroable for core::num::Saturating<T> {}

// Note: we can't implement this for all `T: ?Sized` types because it would
// create NULL pointers for vtables.
// Maybe one day this could be changed to be implemented for
// `T: ?Sized where <T as core::ptr::Pointee>::Metadata: Zeroable`.
unsafe impl<T> Zeroable for *mut T {}
unsafe impl<T> Zeroable for *const T {}
unsafe impl<T> Zeroable for *mut [T] {}
unsafe impl<T> Zeroable for *const [T] {}
unsafe impl Zeroable for *mut str {}
unsafe impl Zeroable for *const str {}

unsafe impl<T: ?Sized> Zeroable for PhantomData<T> {}
unsafe impl Zeroable for PhantomPinned {}
unsafe impl<T: Zeroable> Zeroable for core::mem::ManuallyDrop<T> {}
unsafe impl<T: Zeroable> Zeroable for core::cell::UnsafeCell<T> {}
unsafe impl<T: Zeroable> Zeroable for core::cell::Cell<T> {}


pub trait Zeroed : Sized
{
    /// Calls [`zeroed`](core::mem::zeroed).
    ///
    /// This is a trait method so that you can write `MyType::zeroed()` in your
    /// code. It is a contract of this trait that if you implement it on your type
    /// you **must not** override this method.
    #[inline]
    fn zeroed() -> Self
    {
        unsafe { core::mem::zeroed() }
    }
}
impl<T> Zeroed for T where T: Zeroable {}

mod atomic_impls {
    use super::Zeroable;

    #[cfg(target_has_atomic = "8")]
    unsafe impl Zeroable for core::sync::atomic::AtomicBool {}
    #[cfg(target_has_atomic = "8")]
    unsafe impl Zeroable for core::sync::atomic::AtomicU8 {}
    #[cfg(target_has_atomic = "8")]
    unsafe impl Zeroable for core::sync::atomic::AtomicI8 {}

    #[cfg(target_has_atomic = "16")]
    unsafe impl Zeroable for core::sync::atomic::AtomicU16 {}
    #[cfg(target_has_atomic = "16")]
    unsafe impl Zeroable for core::sync::atomic::AtomicI16 {}

    #[cfg(target_has_atomic = "32")]
    unsafe impl Zeroable for core::sync::atomic::AtomicU32 {}
    #[cfg(target_has_atomic = "32")]
    unsafe impl Zeroable for core::sync::atomic::AtomicI32 {}

    #[cfg(target_has_atomic = "64")]
    unsafe impl Zeroable for core::sync::atomic::AtomicU64 {}
    #[cfg(target_has_atomic = "64")]
    unsafe impl Zeroable for core::sync::atomic::AtomicI64 {}

    #[cfg(target_has_atomic = "ptr")]
    unsafe impl Zeroable for core::sync::atomic::AtomicUsize {}
    #[cfg(target_has_atomic = "ptr")]
    unsafe impl Zeroable for core::sync::atomic::AtomicIsize {}

    #[cfg(target_has_atomic = "ptr")]
    unsafe impl<T> Zeroable for core::sync::atomic::AtomicPtr<T> {}
}

unsafe impl<T> Zeroable for core::mem::MaybeUninit<T> {}

map_on_tuple!(
    (
        $(
            $len:literal => ( $( $idx:tt $typ:ident )+ )
        )*
    ) => {
        $(
            #[cfg_attr(docsrs, doc(fake_variadic))]
            unsafe impl<$( $typ: Zeroable ),+> Zeroable for ( $( $typ ),+ ,) { }
        )*
    };
);

unsafe impl<T, const N: usize> Zeroable for [T; N] where T: Zeroable {}
