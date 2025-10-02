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
        $crate::map_on!(($first_type), $mac $(, $args)*);
        $crate::map_on!(($($rest_type),+), $mac $(, $args)*);
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


    // Entry point for list of pairs with inline macro arms
    ( ( $(($a:tt, $b:tt)),* $(,)? ), ($($params:tt)*) => $body:block ) => {
        const _: () = {
            macro_rules! __map_on_inliner {
                ($($params)*) => $body
            }

            $(
                __map_on_inliner!($a, $b);
            )*
        };
    };

    // Simple case: single identifiers
    ( ($($types:tt),+), $mac:ident $(, $args:tt)* ) => {
        $(
            $mac!($types $(, $args)*);
        )+
    };
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


/// `Add`, `Sub`
#[macro_export]
macro_rules! map_on_operator_binary_arithmetic_unit {
    ($($macro_arms:tt)*) => {
        $crate::map_on!
        (
            (
                (Add, add),
                (Sub, sub)
            ),
            $($macro_arms)*
        );
    };
}


/// (`Add`, `Sub`) + (`Mul`, `Div`, `Rem`)
#[macro_export]
macro_rules! map_on_operator_binary_arithmetic {
    ($($macro_arms:tt)*) => {
        $crate::map_on_operator_binary_arithmetic_unit!($($macro_arms)*);
        $crate::map_on!
        (
            (
                (Mul, mul),
                (Div, div),
                (Rem, rem)
            ),
            $($macro_arms)*
        );
    };
}

/// `BitOr`, `BitAnd`, `Shl`, `Shr`
#[macro_export]
macro_rules! map_on_operator_binary_bit {
    ($($macro_arms:tt)*) => {
        $crate::map_on!
        (
            (
                (BitOr, bitor),
                (BitAnd, bitand),
                (Shl, shl),
                (Shr, shr)
            ),
            $($macro_arms)*
        );
    };
}

/// (`Add`, `Sub`) + (`Mul`, `Div`, `Rem`) + (`BitOr`, `BitAnd`, `Shl`, `Shr`)
#[macro_export]
macro_rules! map_on_operator_binary
{
    ($($macro_arms:tt)*) => {
        $crate::map_on_operator_binary_arithmetic!($($macro_arms)*);
        $crate::map_on_operator_binary_bit!($($macro_arms)*);
    };
}





/// `AddAssign`, `SubAssign`
#[macro_export]
macro_rules! map_on_operator_assign_arithmetic_unit {
    ($($macro_arms:tt)*) => {
        $crate::map_on!
        (
            (
                (AddAssign, add_assign),
                (SubAssign, sub_assign)
            ),
            $($macro_arms)*
        );
    };
}


/// (`AddAssign`, `SubAssign`) + (`MulAssign`, `DivAssign`, `RemAssign`)
#[macro_export]
macro_rules! map_on_operator_assign_arithmetic {
    ($($macro_arms:tt)*) => {
        $crate::map_on_operator_assign_arithmetic_unit!($($macro_arms)*);
        $crate::map_on!
        (
            (
                (MulAssign, mul_assign),
                (DivAssign, div_assign),
                (RemAssign, rem_assign)
            ),
            $($macro_arms)*
        );
    };
}

/// `BitOrAssign`, `BitAndAssign`, `ShlAssign`, `ShrAssign`
#[macro_export]
macro_rules! map_on_operator_assign_bit {
    ($($macro_arms:tt)*) => {
        $crate::map_on!
        (
            (
                (BitOrAssign, bitor_assign),
                (BitAndAssign, bitand_assign),
                (ShlAssign, shl_assign),
                (ShrAssign, shr_assign)
            ),
            $($macro_arms)*
        );
    };
}

/// (`AddAssign`, `SubAssign`) + (`MulAssign`, `DivAssign`, `RemAssign`) + (`BitOrAssign`, `BitAndAssign`, `ShlAssign`, `ShrAssign`)
#[macro_export]
macro_rules! map_on_operator_assign
{
    ($($macro_arms:tt)*) => {
        $crate::map_on_operator_assign_arithmetic!($($macro_arms)*);
        $crate::map_on_operator_assign_bit!($($macro_arms)*);
    };
}

/// Macro for mapping over all standard [formatting traits](https://doc.rust-lang.org/std/fmt/index.html#formatting-traits) except [std::fmt::Debug]:
///
/// - Display
/// - Octal
/// - LowerHex
/// - UpperHex
/// - Pointer
/// - Binary
/// - LowerExp
/// - UpperExp
#[macro_export]
macro_rules! map_on_std_fmt_without_debug {
    ($($macro_arms:tt)*) => {
        $crate::map_on!(
            (
                Display,
                Octal,
                LowerHex,
                UpperHex,
                Pointer,
                Binary,
                LowerExp,
                UpperExp
            ),
            $($macro_arms)*
        );
    };
}

/// Macro for mapping over all standard [formatting traits](https://doc.rust-lang.org/std/fmt/index.html#formatting-traits):
///
/// - Display
/// - Debug
/// - Octal
/// - LowerHex
/// - UpperHex
/// - Pointer
/// - Binary
/// - LowerExp
/// - UpperExp
#[macro_export]
macro_rules! map_on_std_fmt {
    ($($macro_arms:tt)*) => {
        $crate::map_on_std_fmt_without_debug!($($macro_arms)*);
        $crate::map_on!((Debug), $($macro_arms)*
        );
    };
}






/// `Not`
#[macro_export]
macro_rules! map_on_operator_unary_bit
{
    ($($macro_arms:tt)*) => {
        $crate::map_on!
        (
            (
                (Not, not)
            ),
            $($macro_arms)*
        );
    };
}

/// `Neg`, `Abs`
#[macro_export]
macro_rules! map_on_operator_unary_arithmetic_unit
{
    ($($macro_arms:tt)*) => {
        $crate::map_on!
        (
            (
                (Neg, neg),
                (Abs, abs)
            ),
            $($macro_arms)*
        );
    };
}


/// (`Not`) + (`Neg`, `Abs`)
#[macro_export]
macro_rules! map_on_operator_unary
{
    ($($macro_arms:tt)*) => {
        $crate::map_on_operator_unary_bit!($($macro_arms)*);
        $crate::map_on_operator_unary_arithmetic_unit!($($macro_arms)*);
    };
}


pub mod prelude
{
    pub use super::*;
}