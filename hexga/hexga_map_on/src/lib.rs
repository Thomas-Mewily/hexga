#![no_std]

/// A powerful macro to impl other macros for the given types.
/// 
/// Can be used to impl trait to a lot of type using macro, where generic can't.
/// 
/// ```rust
/// use hexga_map_on::map_on;
/// 
/// trait Zero
/// {
///     const ZERO : Self;
/// }
/// 
/// macro_rules! impl_zero {
///     ($type_name:ty) => {
///         impl Zero for $type_name
///         {
///             const ZERO : Self = 0 as Self;
///         }
///     };
/// }
/// 
/// map_on!
/// (
///     (
///         i8, i16, i32, i64, isize,
///         u8, u16, u32, u64, usize,
///         f32, f64
///     ), 
///     impl_zero
/// );
/// 
/// // ^^ this call impl Zero for all the given type
/// 
/// assert_eq!(i32::ZERO  , 0);
/// assert_eq!(usize::ZERO, 0);
/// assert_eq!(f32::ZERO  , 0.);
/// ``` 
#[macro_export]
macro_rules! map_on {
    // Base case: single type
    ( ($type_name:tt), $mac:ident $(, $args:tt)* ) => {
        $mac!($type_name $(, $args)*);
    };
    // Recursive case: multiple types
    ( ($first_type:tt, $($rest_type:tt),+), $mac:ident $(, $args:tt)* ) => {
        map_on!(($first_type), $mac $(, $args)*);
        map_on!(($($rest_type),+), $mac $(, $args)*);
    };
}

/// `i8`, `i16`, `i32`, `i64`, `isize`
#[macro_export]
macro_rules! map_on_integer_signed {
    ($mac:ident $(, $args:tt)* ) => {
        $crate::map_on!((i8, i16, i32, i64, isize), $mac $(, $args)*);
    };
}

/// `u8`, `u16`, `u32`, `u64`, `usize`
#[macro_export]
macro_rules! map_on_integer_unsigned {
    ($mac:ident $(, $args:tt)* ) => {
        $crate::map_on!((u8, u16, u32, u64, usize), $mac $(, $args)*);
    };
}

/// (`i8`, `i16`, `i32`, `i64`, `isize`) + (`u8`, `u16`, `u32`, `u64`, `usize`)
#[macro_export]
macro_rules! map_on_integer 
{
    ($mac:ident $(, $args:tt)* ) => {
        $crate::map_on_integer_signed!($mac $(, $args)*);
        $crate::map_on_integer_unsigned!($mac $(, $args)*);
    };
}

/// `f32`, `f64`
#[macro_export]
macro_rules! map_on_float 
{
    ($mac:ident $(, $args:tt)* ) => {
        $crate::map_on!((f32, f64), $mac $(, $args)*);
    };
}

/// (`i8`, `i16`, `i32`, `i64`, `isize`) + (`u8`, `u16`, `u32`, `u64`, `usize`) + (`f32`, `f64`)
#[macro_export]
macro_rules! map_on_number 
{
    ($mac:ident $(, $args:tt)* ) => {
        $crate::map_on_integer!($mac $(, $args)*);
        $crate::map_on_float!($mac $(, $args)*);
    };
}

/// (`i8`, `i16`, `i32`, `i64`, `isize`) + (`u8`, `u16`, `u32`, `u64`, `usize`) + (`f32`, `f64`) + (`bool`)
#[macro_export]
macro_rules! map_on_number_and_bool 
{
    ($mac:ident $(, $args:tt)* ) => {
        $crate::map_on_number!($mac $(, $args)*);
        $crate::map_on!((bool), $mac $(, $args)*);
    };
}