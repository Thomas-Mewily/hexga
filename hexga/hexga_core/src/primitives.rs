use crate::bit::*;
use crate::fmt::{Debug, Formatter};
use crate::map_on_number;
use crate::ptr::NonNull;

pub mod prelude
{
    pub use super::Empty;
    pub use core::primitive::*;
}

pub use core::primitive::*;

pub type Empty = ();

#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
mod max_uint_and_int_types
{
    pub type umax = u64;
    pub type imax = i64;
}

#[cfg(target_pointer_width = "32")]
#[allow(non_camel_case_types)]
mod max_uint_and_int_types
{
    pub type umax = usize;
    pub type imax = isize;
}

pub use max_uint_and_int_types::*;

/*
#[allow(non_camel_case_types)]
pub type umin = u8;
#[allow(non_camel_case_types)]
pub type imin = i8;
#[allow(non_camel_case_types)]
pub type fmin = f64;
*/

#[allow(non_camel_case_types)]
pub type fmax = f64;

/// A raw value container capable of storing any primitive numeric type,
/// `bool`, `char`, or pointer.
///
/// `Word` is a `union`, meaning all fields share the same memory.
/// It behaves like an untyped machine word and allows reinterpretation
/// of the same bits as different primitive types.
///
/// # Safety
///
/// Reading a field different from the one that was written is
/// undefined behavior unless the bit-pattern is valid for the
/// target type.
///
/// In particular:
/// - `bool` must be `0` or `1`.
/// - `char` must be a valid Unicode scalar value.
/// - Other numeric types may accept any bit pattern.
///
/// Prefer using safe helper methods like `as_bool()` and `as_char()`
/// when reinterpretation requires validation.
#[allow(non_camel_case_types)]
pub union word
{
    pub u8: u8,
    pub u16: u16,
    pub u32: u32,
    pub u64: u64,
    pub usize: usize,
    pub umax: umax,
    pub i8: i8,
    pub i16: i16,
    pub i32: i32,
    pub i64: i64,
    pub isize: isize,
    pub imax: imax,

    /// Bool only allow 0 (false) or 1 (true), not other representation.
    /// Use [`Self::as_bool()`] for a safe equivalent !
    pub unsafe_bool: bool,
    /// All bit pattern are not valid for word !
    /// Use [`Self::as_char()`] for a safe equivalent !
    pub unsafe_char: char,
    pub f32: f32,
    pub f64: f64,
    pub ptr: Option<NonNull>,
    /// Can be null
    pub const_ptr: *const u8,
    /// Can be null
    pub mut_ptr: *mut u8,
}

map_on_number!(
    ($type_name: tt) => {
        impl From<$type_name> for word {
            fn from(value: $type_name) -> Self {
                word { $type_name: value }
            }
        }
        impl From<word> for $type_name {
            fn from(value: word) -> Self {
                unsafe { value.$type_name }
            }
        }
    };
);
impl From<bool> for word
{
    fn from(value: bool) -> Self { Self { unsafe_bool: value } }
}
impl From<char> for word
{
    fn from(value: char) -> Self { Self { unsafe_char: value } }
}

impl TryFrom<word> for bool
{
    type Error = ();
    fn try_from(value: word) -> Result<Self, Self::Error> { value.as_bool().ok_or(()) }
}
impl TryFrom<word> for char
{
    type Error = ();
    fn try_from(value: word) -> Result<Self, Self::Error> { value.as_char().ok_or(()) }
}

impl word
{
    pub fn as_bool(&self) -> Option<bool>
    {
        unsafe {
            match self.umax
            {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            }
        }
    }

    pub fn as_char(&self) -> Option<char>
    {
        unsafe {
            let v = self.umax as u32;
            char::from_u32(v)
        }
    }
}
impl Debug for word
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        unsafe {
            write!(
                f,
                "{:#X}umax {}imax 0x{:#X} {}f32 {}f64 {:?} {:?}",
                self.umax,
                self.imax,
                self.umax,
                self.f32,
                self.f64,
                self.as_bool(),
                self.as_char().map(|c| c.escape_debug())
            )
        }
    }
}
impl Default for word
{
    fn default() -> Self { Self::ZERO }
}
impl word
{
    pub const ZERO: Self = word { umax: 0 };
    pub const ONES: Self = word { umax: umax::MAX };
}
impl Ord for word
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { unsafe { self.umax.cmp(&other.umax) } }
}
impl PartialOrd for word
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        unsafe { self.umax.partial_cmp(&other.umax) }
    }
}
impl Eq for word {}
impl PartialEq for word
{
    fn eq(&self, other: &Self) -> bool { unsafe { self.umax == other.umax } }
}
impl std::hash::Hash for word
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { unsafe { self.umax.hash(state) }; }
}
impl Copy for word {}
impl Clone for word
{
    fn clone(&self) -> Self { Self { umax: 0 } }
}
unsafe impl BitZero for word {}
unsafe impl BitAnyPattern for word {}
unsafe impl BitAllUsed for word {}
