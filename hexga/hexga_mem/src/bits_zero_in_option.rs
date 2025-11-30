use super::*;

// Note(Mewily) Indeed, it is super neat !
unsafe impl<T: BitsZeroInOption> BitsZero for Option<T> {}

/// Trait for types which are [Zeroable](Zeroable) when wrapped in
/// [Option](core::option::Option).
///
/// ## Safety
///
/// * `Option<YourType>` must uphold the same invariants as
///   [Zeroable](Zeroable).
pub unsafe trait BitsZeroInOption: Sized {}

unsafe impl BitsZeroInOption for NonZeroI8 {}
unsafe impl BitsZeroInOption for NonZeroI16 {}
unsafe impl BitsZeroInOption for NonZeroI32 {}
unsafe impl BitsZeroInOption for NonZeroI64 {}
unsafe impl BitsZeroInOption for NonZeroI128 {}
unsafe impl BitsZeroInOption for NonZeroIsize {}
unsafe impl BitsZeroInOption for NonZeroU8 {}
unsafe impl BitsZeroInOption for NonZeroU16 {}
unsafe impl BitsZeroInOption for NonZeroU32 {}
unsafe impl BitsZeroInOption for NonZeroU64 {}
unsafe impl BitsZeroInOption for NonZeroU128 {}
unsafe impl BitsZeroInOption for NonZeroUsize {}

// Note: this does not create NULL vtable because we get `None` anyway.
unsafe impl<T: ?Sized> BitsZeroInOption for core::ptr::NonNull<T> {}
unsafe impl<T: ?Sized> BitsZeroInOption for &'_ T {}
unsafe impl<T: ?Sized> BitsZeroInOption for &'_ mut T {}

#[cfg(feature = "extern_crate_alloc")]
unsafe impl<T: ?Sized> BitsZeroInOption for alloc::boxed::Box<T> {}
