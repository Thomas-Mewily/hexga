//! Define what will be the default `int`, `uint` and `float` (and `coef` = `float`) typedef.


mod check;
#[allow(unused_imports)]
use check::*;

#[cfg(feature = "int_are_size_bits")]
mod integer_typedef
{
    /// Default unsigned integer type
    #[allow(non_camel_case_types)]
    pub type uint = usize;

    /// Default signed integer type
    #[allow(non_camel_case_types)]
    pub type int  = isize;
}

#[cfg(feature = "int_are_8_bits")]
mod integer_typedef
{
    /// Default unsigned integer type
    #[allow(non_camel_case_types)]
    pub type uint = u8;

    /// Default signed integer type
    #[allow(non_camel_case_types)]
    pub type int  = i8;
}

#[cfg(feature = "int_are_16_bits")]
mod integer_typedef
{
    /// Default unsigned integer type
    #[allow(non_camel_case_types)]
    pub type uint = u16;

    /// Default signed integer type
    #[allow(non_camel_case_types)]
    pub type int  = i16;
}

#[cfg(feature = "int_are_32_bits")]
mod integer_typedef
{
    /// Default unsigned integer type
    #[allow(non_camel_case_types)]
    pub type uint = u32;

    /// Default signed integer type
    #[allow(non_camel_case_types)]
    pub type int  = i32;
}

#[cfg(feature = "int_are_64_bits")]
mod integer_typedef
{
    /// Default unsigned integer type
    #[allow(non_camel_case_types)]
    pub type uint = u64;

    /// Default signed integer type
    #[allow(non_camel_case_types)]
    pub type int  = i64;
}
pub use integer_typedef::*;


#[cfg(feature = "float_are_size_bits")]
mod float_typedef
{
    /// Default unsigned floating type
    #[cfg(target_pointer_width = "32")]
    #[allow(non_camel_case_types)]
    pub type float = f32;

    /// Default unsigned floating type
    #[cfg(target_pointer_width = "64")]
    #[allow(non_camel_case_types)]
    pub type float = f64;
}

#[cfg(feature = "float_are_32_bits")]
mod float_typedef
{
    /// Default unsigned floating type
    #[allow(non_camel_case_types)]
    pub type float = f32;
}

#[cfg(feature = "float_are_64_bits")]
mod float_typedef
{
    /// Default unsigned floating type
    #[allow(non_camel_case_types)]
    pub type float = f64;
}
pub use float_typedef::*;

/// Same as float, but just to mark the different use case.
///
/// Generally, a coef is between `[0.0, 1.0]`, but this is not an obligation
#[allow(non_camel_case_types)]
pub type coef = float;
