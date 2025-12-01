use super::*;

// Note(Mewily) Indeed, it is super neat !
/// [bytemuck::ZeroableInOption](https://docs.rs/bytemuck/latest/bytemuck/trait.ZeroableInOption.html) trait equivalent.
unsafe impl<T: BitZeroInOption> BitZero for Option<T> {}

/// Trait for types which are [BitZero](BitZero) when wrapped in
/// [Option](core::option::Option).
///
/// ## Safety
///
/// * `Option<YourType>` must uphold the same invariants as
///   [BitZero](BitZero).
pub unsafe trait BitZeroInOption: Sized {}

unsafe impl BitZeroInOption for NonZeroI8 {}
unsafe impl BitZeroInOption for NonZeroI16 {}
unsafe impl BitZeroInOption for NonZeroI32 {}
unsafe impl BitZeroInOption for NonZeroI64 {}
unsafe impl BitZeroInOption for NonZeroI128 {}
unsafe impl BitZeroInOption for NonZeroIsize {}
unsafe impl BitZeroInOption for NonZeroU8 {}
unsafe impl BitZeroInOption for NonZeroU16 {}
unsafe impl BitZeroInOption for NonZeroU32 {}
unsafe impl BitZeroInOption for NonZeroU64 {}
unsafe impl BitZeroInOption for NonZeroU128 {}
unsafe impl BitZeroInOption for NonZeroUsize {}

// Note: this does not create NULL vtable because we get `None` anyway.
unsafe impl<T: ?Sized> BitZeroInOption for core::ptr::NonNull<T> {}
unsafe impl<T: ?Sized> BitZeroInOption for &'_ T {}
unsafe impl<T: ?Sized> BitZeroInOption for &'_ mut T {}

#[cfg(feature = "extern_crate_alloc")]
unsafe impl<T: ?Sized> BitZeroInOption for alloc::boxed::Box<T> {}
