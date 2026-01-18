

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

/// (`u8`, `u16`, `u32`, `u64`, `usize`) + (`i8`, `i16`, `i32`, `i64`, `isize`) + (`f32`, `f64`) + (`bool`) + (`char`)
#[macro_export]
macro_rules! map_on_number_and_bool_and_char
{
    ($($macro_arms:tt)*) => {
        $crate::map_on_number!($($macro_arms)*);
        $crate::map_on!((bool, char), $($macro_arms)*);
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


