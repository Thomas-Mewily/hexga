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

    // Limitation :
    // Can only be used in const context (ex: impl Trait).
    // Ex: this **won't** work inside a function :
    // ```
    // map_on!((i32, f64, bool),
    //    ($T:ident) => {
    //        println!("Type: {}", std::any::type_name::<$T>());
    //    }
    // );
    // ```
    ($tokens:tt, $($macro_arms:tt)+) => {
        const _: () = {
            macro_rules! __map_on_inliner {
                $($macro_arms)+
            }

            $crate::map_on!(@expand_tokens $tokens);
        };
    };

    // Recursive expansion
    (@expand_tokens ($first:tt $(, $rest:tt)*)) => {
        __map_on_inliner!($first);
        $crate::map_on!(@expand_tokens ($($rest),*))
    };

    (@expand_tokens ($last:tt)) => {
        __map_on_inliner!($last);
    };

    (@expand_tokens ()) => {};
}

/// `u8`, `u16`, `u32`, `u64`, `usize`
#[macro_export]
macro_rules! map_on_integer_unsigned {
    ($($macro_arms:tt)*) => {
        $crate::map_on!((u8, u16, u32, u64, usize), $($macro_arms)*);
    };
}


/// `i8`, `i16`, `i32`, `i64`, `isize`
#[macro_export]
macro_rules! map_on_integer_signed {
    ($($macro_arms:tt)*) => {
        $crate::map_on!((i8, i16, i32, i64, isize), $($macro_arms)*);
    };
}

/// (`u8`, `u16`, `u32`, `u64`, `usize`) + (`i8`, `i16`, `i32`, `i64`, `isize`)
#[macro_export]
macro_rules! map_on_integer 
{
    ($($macro_arms:tt)*) => {
        $crate::map_on_integer_unsigned!($($macro_arms)*);
        $crate::map_on_integer_signed!($($macro_arms)*);
    };
}

/// `f32`, `f64`
#[macro_export]
macro_rules! map_on_float 
{
    ($($macro_arms:tt)*) => {
        $crate::map_on!((f32, f64), $($macro_arms)*);
    };
}

/// (`u8`, `u16`, `u32`, `u64`, `usize`) + (`i8`, `i16`, `i32`, `i64`, `isize`) + (`f32`, `f64`)
#[macro_export]
macro_rules! map_on_number 
{
    ($($macro_arms:tt)*) => {
        $crate::map_on_integer!($($macro_arms)*);
        $crate::map_on_float!($($macro_arms)*);
    };
}

/// (`u8`, `u16`, `u32`, `u64`, `usize`) + (`i8`, `i16`, `i32`, `i64`, `isize`) + (`f32`, `f64`) + (`bool`)
#[macro_export]
macro_rules! map_on_number_and_bool 
{
    ($($macro_arms:tt)*) => {
        $crate::map_on_number!($($macro_arms)*);
        $crate::map_on!((bool), $($macro_arms)*);
    };
}

/* 
// TODO
/// All standard binary operator traits from `core::ops`
///
/// Includes: Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Shl, Shr
#[macro_export]
macro_rules! map_on_operator_binary {
    ($mac:ident $(, $args:tt)* ) => 
    {
        $crate::map_on!
        (
            (
                (std::ops::Add, add)
            ), 
            $mac $(, $args)*
        );
    };
}
*/