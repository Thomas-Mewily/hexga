use super::*;

/// [bytemuck::Pod](https://docs.rs/bytemuck/latest/bytemuck/trait.Pod.html) trait equivalent.
///
/// Marker trait for "plain old data".
///
/// The point of this trait is that once something is marked "plain old data"
/// you can really go to town with the bit fiddling and bit casting. Therefore,
/// it's a relatively strong claim to make about a type. Do not add this to your
/// type casually.
///
/// **Reminder:** The results of casting around bytes between data types are
/// _endian dependant_. Little-endian machines are the most common, but
/// big-endian machines do exist (and big-endian is also used for "network
/// order" bytes).
///
/// ## Safety
///
/// * The type must be inhabited (eg: no
///   [Infallible](core::convert::Infallible)).
/// * The type must allow any bit pattern (eg: no `bool` or `char`, which have
///   illegal bit patterns).
/// * The type must not contain any uninit (or padding) bytes, either in the
///   middle or on the end (eg: no `#[repr(C)] struct Foo(u8, u16)`, which has
///   padding in the middle, and also no `#[repr(C)] struct Foo(u16, u8)`, which
///   has padding on the end).
/// * The type needs to have all fields also be `Pod`.
/// * The type needs to be `repr(C)` or `repr(transparent)`. In the case of
///   `repr(C)`, the `packed` and `align` repr modifiers can be used as long as
///   all other rules end up being followed.
/// * It is disallowed for types to contain pointer types, `Cell`, `UnsafeCell`,
///   atomics, and any other forms of interior mutability.
/// * More precisely: A shared reference to the type must allow reads, and
///   *only* reads. RustBelt's separation logic is based on the notion that a
///   type is allowed to define a sharing predicate, its own invariant that must
///   hold for shared references, and this predicate is the reasoning that allow
///   it to deal with atomic and cells etc. We require the sharing predicate to
///   be trivial and permit only read-only access.
pub unsafe trait Pod: BitZero + Copy + 'static {}

map_on_number!(
    ($type_name : ident) =>
    {
        unsafe impl Pod for $type_name {}
    }
);

unsafe impl Pod for () {}
unsafe impl<T: Pod> Pod for Wrapping<T> {}
unsafe impl<T: Pod> Pod for core::num::Saturating<T>{}

unsafe impl<T: ?Sized + 'static> Pod for PhantomData<T> {}
unsafe impl Pod for PhantomPinned {}
unsafe impl<T: Pod> Pod for core::mem::ManuallyDrop<T> {}

unsafe impl<T,const N: usize> Pod for [T;N] where T: Pod {}
